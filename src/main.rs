//#![windows_subsystem = "windows"]

//not find better for now :)
#![allow(unused_imports)]
#![allow(dead_code)]

mod upload;
mod escape;
mod tools;
mod persistance;

fn main() {

    if tools::reg_countdown("Environment", "WTEMP", 5){
        escape::jump_man(true);
    }

    println!("End...");
}

