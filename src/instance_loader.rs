use std::fs;
use std::str;


pub fn load_instance(fp: &str) -> io::Result<Instance>{
   let content = fs::read_to_string(fp).expect("");
   println!("{content}");
}    
