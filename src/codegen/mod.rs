pub mod generator;

use crate::ast::nodes::Program;
use std::path::Path;

pub fn generate(_program: &Program, _output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Da implementare
    Ok(())
}