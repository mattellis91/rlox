use std::fs;
use std::path::Path;

pub struct Runner {}

impl Default for Runner {
    fn default() -> Self {
        Runner{}
    }
}

impl Runner {
    pub fn file(&mut self, f: &Path) -> Result<(), String> {
        
        let src = fs::read_to_string(f).unwrap_or_default();
        println!("{}",src);    

        Ok(())
    }
}