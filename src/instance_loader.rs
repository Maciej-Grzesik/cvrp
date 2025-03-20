#[path ="../core/core.rs"]mod core;
use std::fs;
use std::str;

pub fn load_instance(fp: &str) {
   //let content = fs::read_to_string(fp).expect("");
   core::test123();
   //println!("{content}");
}       
