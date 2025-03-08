pub mod hive;

use std::collections::HashMap;
use crate::ast::nodes::{Program, Realm, Being, Ritual, Expression, Statement};

/// Contesto di esecuzione per Nervs
pub struct NervsRuntime {
    /// Memoria globale per i realm
    realms: HashMap<String, RuntimeRealm>,
}

/// Stato di esecuzione per un realm
struct RuntimeRealm {
    /// Beings attivi nel realm
    beings: HashMap<String, RuntimeBeing>,
}

/// Stato di esecuzione per un being
struct RuntimeBeing {
    /// Variabili del being
    variables: HashMap<String, RuntimeValue>,
    /// Rituali definiti
    rituals: HashMap<String, Ritual>,
}

/// Rappresentazione di un valore durante l'esecuzione
#[derive(Clone, Debug)]
enum RuntimeValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Void,
}

impl NervsRuntime {
    /// Inizializza il runtime da un programma Nervs
    pub fn new(program: &Program) -> Self {
        let mut realms = HashMap::new();
        
        for realm in &program.realms {
            let mut runtime_realm = RuntimeRealm {
                beings: HashMap::new(),
            };
            
            for being in &realm.beings {
                let runtime_being = RuntimeBeing {
                    variables: being.variables.iter()
                        .map(|var| (var.name.clone(), RuntimeValue::Void))
                        .collect(),
                    rituals: being.rituals.iter()
                        .map(|ritual| (ritual.name.clone(), ritual.clone()))
                        .collect(),
                };
                
                runtime_realm.beings.insert(being.name.clone(), runtime_being);
            }
            
            realms.insert(realm.name.clone(), runtime_realm);
        }
        
        NervsRuntime { realms }
    }
    
    /// Esegue un ritual in un being specifico
    pub fn execute_ritual(&mut self, realm_name: &str, being_name: &str, ritual_name: &str) -> Result<RuntimeValue, String> {
        let realm = self.realms.get_mut(realm_name)
            .ok_or_else(|| format!("Realm {} not found", realm_name))?;
        
        let being = realm.beings.get_mut(being_name)
            .ok_or_else(|| format!("Being {} not found in realm {}", being_name, realm_name))?;
        
        let ritual = being.rituals.get(ritual_name)
            .ok_or_else(|| format!("Ritual {} not found in being {}", ritual_name, being_name))?;
        
        // Esecuzione base del ritual (da implementare completamente)
        // Qui andrà la logica di esecuzione degli statement
        Ok(RuntimeValue::Void)
    }
}

/// Inizializza il runtime del linguaggio
pub fn initialize(program: &Program) -> NervsRuntime {
    NervsRuntime::new(program)
}