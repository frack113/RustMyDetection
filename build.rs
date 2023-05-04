// build.rs

//not find better for now :)
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate winapi;
extern crate winres;

enum Mimic { Acrobat, Excel, Powerpoint, Rustmydetection, Word }

fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();

    let exe_metadata = Mimic::Rustmydetection;

    match exe_metadata {
      Mimic::Acrobat => {
        res.set_icon("./ico/Acrobat.ico");
        res.set("FileVersion", "23.1.20143.0");
        res.set("InternalName", "Acrobat");
        res.set("CompanyName", "Adobe Systems Incorporated");
        res.set("FileDescription", "Adobe Acrobat");
        res.set("LegalCopyright", "Copyright 1984-2022 Adobe Systems Incorporated and its licensors. All rights reserved.");
        res.set("OriginalFilename", "Acrobat.exe");
        res.set("ProductName", "Adobe Acrobat");
        res.set("ProductVersion", "23.1.20143.0");
      }
      Mimic::Excel => {
        res.set_icon("./ico/EXCEL_258.ico");
        res.set("FileVersion", "16.0.5387.1000");
        res.set("InternalName", "Excel");
        res.set("CompanyName", "Microsoft Corporation");
        res.set("FileDescription", "Microsoft Excel");
        res.set("LegalCopyright", "Microsoft® is a registered trademark of Microsoft Corporation.");
        res.set("OriginalFilename", "Excel.exe");
        res.set("ProductName", "Microsoft Office 2016");
        res.set("ProductVersion", "16.0.5387.1000");
      },
      Mimic::Powerpoint => {
        res.set_icon("./ico/POWERPNT_1300.ico");
        res.set("FileVersion", "16.0.4266.1001");
        res.set("InternalName", "POWERPNT");
        res.set("CompanyName", "Microsoft Corporation");
        res.set("FileDescription", "Microsoft PowerPoint");
        res.set("LegalCopyright", "Microsoft® is a registered trademark of Microsoft Corporation.");
        res.set("OriginalFilename", "POWERPNT.EXE");
        res.set("ProductName", "Microsoft Office 2016");
        res.set("ProductVersion", "16.0.4266.1001");
      },
      Mimic::Rustmydetection => {
        //res.set_icon("rust.ico");
        res.set("FileVersion", "1.0.0");
        res.set("InternalName", "RustMyDetection");
        res.set("CompanyName", "Frack113");
        res.set("FileDescription", "Rust My Detection");
        res.set("LegalCopyright", "Fairplay");
        res.set("OriginalFilename", "RustMyDetection.exe");
        res.set("ProductName", "RustMyDetection");
        res.set("ProductVersion", "1.0.0");
      },
      Mimic::Word => {
        res.set_icon("./ico/WORDICON_200.ico");
        res.set("FileVersion", "16.0.4266.1001");
        res.set("InternalName", "WinWord");
        res.set("CompanyName", "Microsoft Corporation");
        res.set("FileDescription", "Microsoft Word");
        res.set("LegalCopyright", "Microsoft® is a registered trademark of Microsoft Corporation.");
        res.set("OriginalFilename", "WinWord.exe");
        res.set("ProductName", "Microsoft Office 2016");
        res.set("ProductVersion", "16.0.4266.1001");
      }
    };
    res.set_language(winapi::um::winnt::MAKELANGID(
      winapi::um::winnt::LANG_ENGLISH,
      winapi::um::winnt::SUBLANG_ENGLISH_US));
    res.compile().unwrap();
  }
}