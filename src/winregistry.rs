#[cfg(windows)] extern crate winreg;

/*pub struct se_registry {
    path: &str,
    /*version: &str,
    revision: &str,
    format: &str,
    hwsnr: &str,
    senr: &str,
    pos_channel: u8,
    pos_music_vol: u8,
    pos_spot_vol: u8,
    pos_channel_index: u8,
    clock_on: bool,
    bundesliga: bool,
    sterne: bool*/
}

pub fn get_se_settings() -> Result<se_registry, &str> {

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let se_reg = hkcu.open_subkey("Software\\VB and VBA Program Settings\\TripleJackpot\\Allgemein").unwrap();

    let se_path: String = se_reg.get_value("Path").unwrap();

    se_registry {
        path: &se_path,
    }
}*/

pub fn create_run_once_command(name: &str, command: &str) {

    let result_of_command = format!("cmd /C \"{:}\"", command);

    set_registry_for_run_once_command(name.to_string(), result_of_command);

}

#[allow(unused_variables)]
#[cfg(not(windows))]
fn set_registry_for_run_once_command(name: String,command: String) {

}

#[cfg(windows)]
fn set_registry_for_run_once_command(name: String, command: String) {
    use self::winreg::RegKey;
    use self::winreg::enums::*;
    use std::io;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    //create_subkey will  create alls keys or simple open a key if already exists! important: gives writa access.
    let run_once = hklm.create_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\RunOnce").unwrap();/*_or_else( |e| match e.kind() {
        io::ErrorKind::NotFound => println!("key does not exists: {:?}", e),
        io::ErrorKind::PermissionDenied => println!("Access Denied: {:?}", e),
        _ => println!("{:?}", e)
    });*/

    run_once.set_value(name, &command).unwrap_or_else( |e| match e.kind() {
        _ => println!("error on set value... {:} :{:?}",&command, e)
    });
}