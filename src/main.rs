#![windows_subsystem = "windows"]

//not find better for now :)
#![allow(unused_imports)]
#![allow(dead_code)]

use std::env;
use std::path::PathBuf;
use std::process::Command;

extern crate winreg;
use winreg::HKEY;
use winreg::RegKey;
use winreg::enums::*;

mod tools {

    use RegKey;
    use HKEY;
    use Command;

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
    
        let base = RegKey::predef(root);
        let  (settings, _disp) = base.create_subkey(regpath).unwrap();
        settings.set_value(name, &value).unwrap();
    }
    
    pub fn spawn(exe_path: &str){
        let exe_in = std::env::current_exe().unwrap();
        // set new
        let exe_out = std::path::PathBuf::from(exe_path);
        // create all the missing folder
        let create_path_result = std::fs::create_dir_all(exe_out.clone().parent().unwrap());
    
        match create_path_result{
            Err(_) => {},
            Ok(_) => {
                let create_path_result = std::fs::copy(exe_in, exe_out.clone());
                match create_path_result{
                    Err(_) => {},
                    Ok(_) => {
                        let _child = Command::new(exe_path)
                            .spawn()
                            .expect("failed to execute process");
                        std::process::exit(0);
                    }
                }
            }
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
    //get current exe
    let my_name = std::env::current_exe()
        .expect("Can't get the exec path")
        .file_name()
        .expect("Can't get the exec name").
        to_string_lossy().
        into_owned();

    match my_name.as_ref(){
        "rustmydetection.exe" => {tools::spawn(r"setup.exe");},
        "setup.exe" => {tools::spawn(r"C:\Program File\Microsoft Office\Office16\WinWord.exe");},
        _ => {}
    }

}
