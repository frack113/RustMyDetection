use reqwest::blocking::Client;
use reqwest::blocking::multipart;
use std::collections::HashMap;
use serde_json::{Result, Value};
use rand::distributions::{Alphanumeric, DistString};
use std::ffi::{OsString, OsStr};

pub fn io_file (file: &str) {

    let file_path = std::path::PathBuf::from(file);
    let size_file = std::fs::metadata(file);
    match size_file{
        Err(_) => return(),
        Ok(_) => {}
    };
    let file_size_string = size_file.unwrap().len().to_string();

    let name = file_path.file_name().unwrap_or(OsStr::new("missingname"));
    let file_name_string = name.to_str().unwrap().to_string();

    let ext = file_path.extension().unwrap_or(OsStr::new("arc"));
    let file_ext_string = ext.to_str().unwrap().to_string();
 
    let csrf = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
    let agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/112.0key";
    
    let mut params = HashMap::new();
    params.insert("csrf_test_name", &csrf);

    let client = Client::new();

    let ufile_storage  = client.post("https://ufile.io/v1/upload/select_storage")
        .header("User-Agent",agent)
        .form(&params)
        .send().unwrap();
    let json_url: Value = serde_json::from_str(&ufile_storage.text().unwrap()).unwrap();
    let storage_base_url = json_url["storageBaseUrl"].as_str().unwrap();

    params.insert("file_size", &file_size_string);
    let ufile_create_session = client.post(format!("{}v1/upload/create_session",storage_base_url))
        .header("User-Agent",agent)
        .form(&params)
        .send().unwrap();
    let json_fuid: Value = serde_json::from_str(&ufile_create_session.text().unwrap()).unwrap();
    let fuid = json_fuid["fuid"].as_str();

    let binding_file_name_string = file_name_string.to_owned();
    let file_form = multipart::Form::new()
        .text("fuid", fuid.expect("REASON").to_string())
        .text("chunk_index","1")
        .text("filename",binding_file_name_string)
        .file("file", file_path)
        .unwrap();
    let _ufile_upload = client.post(format!("{}/v1/upload/chunk",storage_base_url))
        .header("User-Agent",agent)
        .multipart(file_form)
        .send().unwrap();

    let mut finalise_params = HashMap::new();
    let binding_fuid = fuid.expect("No fuid string").to_string();
    let binding_total_chunk = "1".to_owned();
    finalise_params.insert("csrf_test_name", &csrf);
    finalise_params.insert("fuid", &binding_fuid); 
    finalise_params.insert("file_name", &file_name_string);
    finalise_params.insert("file_type", &file_ext_string);
    finalise_params.insert("total_chunks", &binding_total_chunk);

    let _ufile_file_finalise = client.post(format!("{}v1/upload/finalise",storage_base_url))
        .header("User-Agent",agent)
        .form(&finalise_params)
        .send().unwrap();  
    
}