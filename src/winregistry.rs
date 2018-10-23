#[cfg(windows)] extern crate winreg;

pub struct se_registry {
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
}