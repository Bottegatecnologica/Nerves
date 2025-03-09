use std::collections::{HashMap, HashSet};
use crate::ast::nodes::{Program, Realm, Being, Ritual, Statement, Expression, Type, Literal, Variable};
use crate::semantic::SemanticError;

// Struttura per tenere traccia dell'ambiente semantico
pub struct SemanticContext {
    // Tabella dei simboli per ogni realm, being e scope locale
    realm_table: HashMap<String, RealmInfo>,
    current_realm: Option<String>,
    current_being: Option<String>,
    current_ritual: Option<String>,
    // Tiene traccia dello scope attuale
    scope_stack: Vec<HashMap<String, Type>>,
}

// Informazioni sul realm
struct RealmInfo {
    beings: HashMap<String, BeingInfo>,
}

// Informazioni sul being
struct BeingInfo {
    variables: HashMap<String, Type>,
    rituals: HashMap<String, RitualInfo>,
}

// Informazioni sul ritual
struct RitualInfo {
    parameters: Vec<Variable>,
    return_type: Type,
}

impl SemanticContext {
    // Crea un nuovo contesto semantico
    pub fn new() -> Self {
        SemanticContext {
            realm_table: HashMap::new(),
            current_realm: None,
            current_being: None,
            current_ritual: None,
            scope_stack: vec![HashMap::new()],
        }
    }

    // Entra in un nuovo scope
    pub fn enter_scope(&mut self) {
        self.scope_stack.push(HashMap::new());
    }

    // Esci dallo scope corrente
    pub fn exit_scope(&mut self) {
        self.scope_stack.pop();
    }

    // Aggiungi una variabile allo scope corrente
    pub fn add_variable(&mut self, name: &str, var_type: Type) -> Result<(), SemanticError> {
        let scope = self.scope_stack.last_mut().unwrap();
        
        if scope.contains_key(name) {
            return Err(SemanticError::Generic(format!("Variable '{}' already defined in this scope", name)));
        }
        
        scope.insert(name.to_string(), var_type);
        Ok(())
    }

    // Cerca una variabile negli scope, partendo dal più interno
    pub fn lookup_variable(&self, name: &str) -> Option<&Type> {
        for scope in self.scope_stack.iter().rev() {
            if let Some(var_type) = scope.get(name) {
                return Some(var_type);
            }
        }
        
        // Se non trovata negli scope locali, cerca nelle variabili del being
        if let Some(realm) = &self.current_realm {
            if let Some(being) = &self.current_being {
                if let Some(realm_info) = self.realm_table.get(realm) {
                    if let Some(being_info) = realm_info.beings.get(being) {
                        if let Some(var_type) = being_info.variables.get(name) {
                            return Some(var_type);
                        }
                    }
                }
            }
        }
        
        None
    }

    // Aggiungi un realm al contesto
    pub fn add_realm(&mut self, name: &str) -> Result<(), SemanticError> {
        if self.realm_table.contains_key(name) {
            return Err(SemanticError::Generic(format!("Realm '{}' already defined", name)));
        }
        
        self.realm_table.insert(name.to_string(), RealmInfo {
            beings: HashMap::new(),
        });
        
        self.current_realm = Some(name.to_string());
        Ok(())
    }

    // Aggiungi un being al realm corrente
    pub fn add_being(&mut self, name: &str) -> Result<(), SemanticError> {
        if let Some(realm) = &self.current_realm {
            if let Some(realm_info) = self.realm_table.get_mut(realm) {
                if realm_info.beings.contains_key(name) {
                    return Err(SemanticError::Generic(
                        format!("Being '{}' already defined in realm '{}'", name, realm)
                    ));
                }
                
                realm_info.beings.insert(name.to_string(), BeingInfo {
                    variables: HashMap::new(),
                    rituals: HashMap::new(),
                });
                
                self.current_being = Some(name.to_string());
                return Ok(());
            }
        }
        
        Err(SemanticError::Generic("No current realm".to_string()))
    }

    // Aggiungi una variabile al being corrente
    pub fn add_being_variable(&mut self, var: &Variable) -> Result<(), SemanticError> {
        if let Some(realm) = &self.current_realm {
            if let Some(being) = &self.current_being {
                if let Some(realm_info) = self.realm_table.get_mut(realm) {
                    if let Some(being_info) = realm_info.beings.get_mut(being) {
                        if being_info.variables.contains_key(&var.name) {
                            return Err(SemanticError::Generic(
                                format!("Variable '{}' already defined in being '{}'", var.name, being)
                            ));
                        }
                        
                        being_info.variables.insert(var.name.clone(), var.var_type.clone());
                        return Ok(());
                    }
                }
            }
        }
        
        Err(SemanticError::Generic("No current being".to_string()))
    }

    // Aggiungi un ritual al being corrente
    pub fn add_ritual(&mut self, ritual: &Ritual) -> Result<(), SemanticError> {
        if let Some(realm) = &self.current_realm {
            if let Some(being) = &self.current_being {
                if let Some(realm_info) = self.realm_table.get_mut(realm) {
                    if let Some(being_info) = realm_info.beings.get_mut(being) {
                        if being_info.rituals.contains_key(&ritual.name) {
                            return Err(SemanticError::Generic(
                                format!("Ritual '{}' already defined in being '{}'", ritual.name, being)
                            ));
                        }
                        
                        // Verifica che non ci siano nomi duplicati nei parametri
                        let mut param_names = HashSet::new();
                        for param in &ritual.parameters {
                            if !param_names.insert(&param.name) {
                                return Err(SemanticError::Generic(
                                    format!("Duplicate parameter name '{}' in ritual '{}'", param.name, ritual.name)
                                ));
                            }
                        }
                        
                        being_info.rituals.insert(ritual.name.clone(), RitualInfo {
                            parameters: ritual.parameters.clone(),
                            return_type: ritual.return_type.clone(),
                        });
                        
                        self.current_ritual = Some(ritual.name.clone());
                        return Ok(());
                    }
                }
            }
        }
        
        Err(SemanticError::Generic("No current being".to_string()))
    }

    // Verifica che una chiamata a ritual sia valida
    pub fn check_ritual_call(&self, name: &str, args: &[Expression]) -> Result<Type, SemanticError> {
        if let Some(realm) = &self.current_realm {
            if let Some(being) = &self.current_being {
                if let Some(realm_info) = self.realm_table.get(realm) {
                    if let Some(being_info) = realm_info.beings.get(being) {
                        if let Some(ritual_info) = being_info.rituals.get(name) {
                            // Verifica che il numero di argomenti corrisponda
                            if args.len() != ritual_info.parameters.len() {
                                return Err(SemanticError::Generic(
                                    format!(
                                        "Ritual '{}' expects {} arguments, but {} were provided",
                                        name, ritual_info.parameters.len(), args.len()
                                    )
                                ));
                            }
                            
                            // Verifica il tipo di ogni argomento
                            for (_i, (arg, param)) in args.iter().zip(&ritual_info.parameters).enumerate() {
                                let arg_type = self.infer_expression_type(arg)?;
                                if !self.types_compatible(&arg_type, &param.var_type) {
                                    return Err(SemanticError::TypeMismatch {
                                        expected: format!("{:?}", param.var_type),
                                        found: format!("{:?}", arg_type),
                                    });
                                }
                            }
                            
                            return Ok(ritual_info.return_type.clone());
                        }
                    }
                }
            }
        }
        
        Err(SemanticError::UndefinedRitual(name.to_string()))
    }

    // Inferisci il tipo di un'espressione
    pub fn infer_expression_type(&self, expr: &Expression) -> Result<Type, SemanticError> {
        match expr {
            Expression::Literal(lit) => {
                match lit {
                    Literal::Integer(_) => Ok(Type::Integer),
                    Literal::Float(_) => Ok(Type::Float),
                    Literal::String(_) => Ok(Type::String),
                    Literal::Boolean(_) => Ok(Type::Boolean),
                }
            },
            Expression::Variable(name) => {
                if let Some(var_type) = self.lookup_variable(name) {
                    Ok(var_type.clone())
                } else {
                    Err(SemanticError::UndefinedVariable(name.clone()))
                }
            },
            Expression::BinaryOperation { left, operator, right } => {
                let left_type = self.infer_expression_type(left)?;
                let right_type = self.infer_expression_type(right)?;
                
                // Tipizzazione delle operazioni binarie
                match operator {
                    // Operazioni aritmetiche
                    crate::ast::nodes::BinaryOperator::Add |
                    crate::ast::nodes::BinaryOperator::Subtract |
                    crate::ast::nodes::BinaryOperator::Multiply |
                    crate::ast::nodes::BinaryOperator::Divide => {
                        if matches!(left_type, Type::Integer | Type::Float) &&
                           matches!(right_type, Type::Integer | Type::Float) {
                            // Promozione di tipo: se uno è float, il risultato è float
                            if matches!(left_type, Type::Float) || matches!(right_type, Type::Float) {
                                Ok(Type::Float)
                            } else {
                                Ok(Type::Integer)
                            }
                        } else {
                            Err(SemanticError::TypeMismatch {
                                expected: "numeric type".to_string(),
                                found: format!("{:?} and {:?}", left_type, right_type),
                            })
                        }
                    },
                    // Operazioni di confronto
                    crate::ast::nodes::BinaryOperator::Equal |
                    crate::ast::nodes::BinaryOperator::NotEqual => {
                        // Confronto di uguaglianza permesso tra tipi compatibili
                        if self.types_compatible(&left_type, &right_type) {
                            Ok(Type::Boolean)
                        } else {
                            Err(SemanticError::TypeMismatch {
                                expected: format!("{:?}", left_type),
                                found: format!("{:?}", right_type),
                            })
                        }
                    },
                    crate::ast::nodes::BinaryOperator::LessThan |
                    crate::ast::nodes::BinaryOperator::GreaterThan => {
                        // Confronto di ordine permesso solo tra numeri
                        if matches!(left_type, Type::Integer | Type::Float) &&
                           matches!(right_type, Type::Integer | Type::Float) {
                            Ok(Type::Boolean)
                        } else {
                            Err(SemanticError::TypeMismatch {
                                expected: "numeric type".to_string(),
                                found: format!("{:?} and {:?}", left_type, right_type),
                            })
                        }
                    },
                }
            },
            Expression::FunctionCall { name, arguments } => {
                self.check_ritual_call(name, arguments)
            },
        }
    }

    // Verifica la compatibilità tra due tipi
    pub fn types_compatible(&self, t1: &Type, t2: &Type) -> bool {
        // Regole di compatibilità base
        if t1 == t2 {
            return true;
        }
        
        // Si potrebbero aggiungere altre regole di compatibilità
        // ad esempio Integer e Float potrebbero essere compatibili in certi contesti
        
        false
    }
}

// Funzione principale di analisi semantica
pub fn analyze_program(program: &Program) -> Result<(), SemanticError> {
    let mut context = SemanticContext::new();
    
    // Analizza tutti i realm
    for realm in &program.realms {
        analyze_realm(&mut context, realm)?;
    }
    
    Ok(())
}

// Analizza un realm
fn analyze_realm(context: &mut SemanticContext, realm: &Realm) -> Result<(), SemanticError> {
    context.add_realm(&realm.name)?;
    
    // Analizza tutti i being nel realm
    for being in &realm.beings {
        analyze_being(context, being)?;
    }
    
    Ok(())
}

// Analizza un being
fn analyze_being(context: &mut SemanticContext, being: &Being) -> Result<(), SemanticError> {
    context.add_being(&being.name)?;
    
    // Aggiungi tutte le variabili del being
    for var in &being.variables {
        context.add_being_variable(var)?;
    }
    
    // Analizza tutti i ritual nel being
    for ritual in &being.rituals {
        analyze_ritual(context, ritual)?;
    }
    
    Ok(())
}

// Analizza un ritual
fn analyze_ritual(context: &mut SemanticContext, ritual: &Ritual) -> Result<(), SemanticError> {
    context.add_ritual(ritual)?;
    
    // Crea un nuovo scope per i parametri e il corpo del ritual
    context.enter_scope();
    
    // Aggiungi i parametri allo scope locale
    for param in &ritual.parameters {
        context.add_variable(&param.name, param.var_type.clone())?;
    }
    
    // Analizza il corpo del ritual
    let mut has_return = false;
    for stmt in &ritual.body {
        if let Statement::Return(_) = stmt {
            has_return = true;
        }
        analyze_statement(context, stmt, &ritual.return_type)?;
    }
    
    // Verifica che un ritual non void abbia un return
    if !matches!(ritual.return_type, Type::Void) && !has_return {
        return Err(SemanticError::Generic(
            format!("Ritual '{}' must return a value of type {:?}", ritual.name, ritual.return_type)
        ));
    }
    
    // Esci dallo scope del ritual
    context.exit_scope();
    
    Ok(())
}

// Analizza uno statement
fn analyze_statement(context: &mut SemanticContext, stmt: &Statement, expected_return_type: &Type) -> Result<(), SemanticError> {
    match stmt {
        Statement::VariableDeclaration { variable, initializer } => {
            // Se c'è un initializer, verifica che il tipo sia compatibile
            if let Some(init) = initializer {
                let init_type = context.infer_expression_type(init)?;
                if !context.types_compatible(&init_type, &variable.var_type) {
                    return Err(SemanticError::TypeMismatch {
                        expected: format!("{:?}", variable.var_type),
                        found: format!("{:?}", init_type),
                    });
                }
            }
            
            // Aggiungi la variabile allo scope corrente
            context.add_variable(&variable.name, variable.var_type.clone())?;
            Ok(())
        },
        Statement::Assignment { name, value } => {
            // Verifica che la variabile esista
            let var_type = context.lookup_variable(name)
                .ok_or_else(|| SemanticError::UndefinedVariable(name.clone()))?;
            
            // Verifica che il tipo del valore sia compatibile
            let value_type = context.infer_expression_type(value)?;
            if !context.types_compatible(&value_type, var_type) {
                return Err(SemanticError::TypeMismatch {
                    expected: format!("{:?}", var_type),
                    found: format!("{:?}", value_type),
                });
            }
            
            Ok(())
        },
        Statement::RitualCall { name, arguments } => {
            // Verifica che la chiamata al ritual sia valida
            let _ = context.check_ritual_call(name, arguments)?;
            Ok(())
        },
        Statement::Conditional { condition, true_branch, false_branch } => {
            // Verifica che la condizione sia booleana
            let condition_type = context.infer_expression_type(condition)?;
            if !matches!(condition_type, Type::Boolean) {
                return Err(SemanticError::TypeMismatch {
                    expected: "Boolean".to_string(),
                    found: format!("{:?}", condition_type),
                });
            }
            
            // Analizza i branch
            context.enter_scope();
            for stmt in true_branch {
                analyze_statement(context, stmt, expected_return_type)?;
            }
            context.exit_scope();
            
            if let Some(false_stmts) = false_branch {
                context.enter_scope();
                for stmt in false_stmts {
                    analyze_statement(context, stmt, expected_return_type)?;
                }
                context.exit_scope();
            }
            
            Ok(())
        },
        Statement::Cycle { condition, body } => {
            // Se c'è una condizione, verifica che sia booleana
            if let Some(cond) = condition {
                let condition_type = context.infer_expression_type(cond)?;
                if !matches!(condition_type, Type::Boolean) {
                    return Err(SemanticError::TypeMismatch {
                        expected: "Boolean".to_string(),
                        found: format!("{:?}", condition_type),
                    });
                }
            }
            
            // Analizza il corpo del ciclo
            context.enter_scope();
            for stmt in body {
                analyze_statement(context, stmt, expected_return_type)?;
            }
            context.exit_scope();
            
            Ok(())
        },
        Statement::Return(expr_opt) => {
            match (expr_opt, expected_return_type) {
                // Return senza espressione per tipo void
                (None, Type::Void) => Ok(()),
                
                // Return con espressione per tipo non void
                (Some(expr), _) => {
                    let expr_type = context.infer_expression_type(expr)?;
                    if !context.types_compatible(&expr_type, expected_return_type) {
                        return Err(SemanticError::TypeMismatch {
                            expected: format!("{:?}", expected_return_type),
                            found: format!("{:?}", expr_type),
                        });
                    }
                    Ok(())
                },
                
                // Return senza espressione per tipo non void
                (None, _) => {
                    Err(SemanticError::TypeMismatch {
                        expected: format!("{:?}", expected_return_type),
                        found: "void".to_string(),
                    })
                },
            }
        },
    }
}

// Funzione principale che analizza l'intero programma
pub fn analyze(program: &Program) -> Result<(), Box<dyn std::error::Error>> {
    analyze_program(program).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}