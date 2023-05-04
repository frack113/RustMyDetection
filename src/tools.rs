use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::os::windows::process::CommandExt;

extern crate winreg;
use winreg::HKEY;
use winreg::RegKey;
use winreg::enums::*;
use winreg::types::ToRegValue;

use std::fs::File;
use std::io::Write;

/* learn how use generic type for value */
pub fn set_reg_str(root: HKEY, regpath: &str,name: &str, value: &str)  -> bool {
    let base = winreg::RegKey::predef(root);
    let rootkey = base.create_subkey(regpath);
    match rootkey{
        Ok(subkey) => {
            let set_result = subkey.0.set_value(name, &value);
            match set_result{
                Err(_) => return false,
                Ok(_) => return true,
            }
        },
        Err(_) => return false,
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


pub fn reg_countdown(regpath:&str,regvalue:&str,count:u32)-> bool{
    let base = winreg::RegKey::predef(HKEY_CURRENT_USER);
    let (rootkey, disp) = base.create_subkey(regpath).expect("erreur");
    match disp{
        REG_CREATED_NEW_KEY => {
            rootkey.set_value(regvalue, &count).expect("fail");
        },
        REG_OPENED_EXISTING_KEY => {
            let subkey_value: Result<u32, std::io::Error> = rootkey.get_value(regvalue);
            match subkey_value{
                Ok(val_reg) => {
                    let new_val = val_reg - 1;
                    rootkey.set_value(regvalue, &new_val).expect("fail");
                    if new_val == 0 {
                        rootkey.delete_value(regvalue).expect("fail");
                        return false
                    }
                },
                Err(_) => rootkey.set_value(regvalue, &count).expect("fail"),
            }
        },
    }
    return true
}

/*
    Drop the file add in the executable with include_bytes!
    Bypass error without panic
*/
pub fn drop_include_bytes(folder:&str, name: &str, data: &[u8]){
    let drop_file_path = PathBuf::new().join(folder).join(name);
    let drop_file = File::create(drop_file_path);
    match drop_file{
        Err(_) => {},
        Ok(mut ret) => {
            let write = ret.write_all(data);
            match write{
                Err(_) => {},
                Ok(_) => {}
            }
        }
    };
}

/*
    Drop the file add in the executable with include_str!
*/
pub fn drop_include_str(folder:&str, name: &str, data: &str){
    drop_include_bytes(folder, name, data.as_bytes());
}

/*
    Execute an executable with a commandline
*/
pub fn run_exe(executable: &str,commandline: &str, folder: Option<&str>) {
    let mut my_path = std::env::current_dir().unwrap();

    if folder.is_some(){
        my_path = PathBuf::from(folder.unwrap());
    }

    let mut child = Command::new(executable)
                                .current_dir(my_path)
                                .raw_arg(commandline)
                                .spawn()
                                .expect("faild start");

    let _ret_code = child.wait().expect("faild waiting");
}

/*
    Execute an executable with a reverse commandline 
*/
pub fn run_exe_reverse(executable: &str,commandline: &str,folder: Option<&str>) {
    let commandline_reversed: String = commandline.chars().rev().collect();
    run_exe(executable,&commandline_reversed,folder);
}
