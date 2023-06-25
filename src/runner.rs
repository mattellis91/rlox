use std::error::Error;
use std::fs::{File, read_to_string, self};
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Runner {
    // stdout: Rc<RefCell<Writer>>,
    // stderr: Rc<RefCell<Writer>>,
}

impl Default for Runner {
    fn default() -> Self {
        // Runner::new(
        //     Rc::new(RefCell::new(Writer::StdOut(io::BufWriter::new(io::stdout())))),
        //     Rc::new(RefCell::new(Writer::StdErr(io::BufWriter::new(io::stderr())))),
        // )
        Runner{}
    }
}

impl Runner {
    // pub fn new(stdout: Rc<RefCell<Writer>>, stderr:Rc<RefCell<Writer>>) -> Self {
    //     Runner {
    //         stdout, 
    //         stderr
    //     }       
    // }

    pub fn file(&mut self, f: &Path) -> Result<(), String> {
        
        let src = fs::read_to_string(f).unwrap_or_default();
        println!("{}",src);    

        Ok(())
    }
}