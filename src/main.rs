extern crate clap;
//extern crate winreg;

use clap::{Arg, App};

use std::path::{Path, PathBuf};
use std::fs;

//use winreg::RegKey;
//use winreg::enums::*;

fn main() {
    let matches = App::new("se_prepper")
        .version("1.0")
        .about("prepare a star entertainer image")
        .author("Nicolas Brueggemann")
        .arg(Arg::with_name("full")
            .short("f")
            .long("full")
            .help("Make full preperation")
            .takes_value(false))
        .arg(Arg::with_name("shell")
            .short("s")
            .long("shell")
            .help("Run as Shell")
            .takes_value(false))
        .get_matches();

    let se_path = Path::new("c:/jackpot");
    if se_path.exists() == false {
        panic!("Jackpot Path does not Exists!");
    }

    if matches.is_present("full") {
        // 1) remove TJNC settings
        {
            let tjnc_path = se_path.join("tjnc");
            remove_bunch_of_files(tjnc_path, vec!("tjnc.ini", "tjnc.ini.bak"));
        }

        {
            let tjnc_path = se_path.join("tjnc").join("");
            remove_bunch_of_files(tjnc_path, vec!("tjnc.ini", "tjnc.ini.bak"));
        }

    }

    if matches.is_present("shell") {
        // hey there...
        // run software as a shell helper...
        use std::process::Command;

        Command::new(se_path.join("jackstarter.exe"))
            .spawn()
            .expect("failed to start jackstarter.exe");

        loop {
            // make endless loop 
            // if system is breaking, software will restart
        }
    }

}

fn remove_bunch_of_files( root_dir: PathBuf, filenames: Vec<&str> ) {
    for filename in filenames {
        remove_file(root_dir.join(filename).to_str().unwrap())
    }
}

fn remove_file(filename: &str) {

    let path = Path::new(filename);

    if path.exists() == false {
        return;
    }

    let ext_filename = path.file_name().unwrap().to_str().unwrap_or("?");

    match fs::remove_file(path.to_str().unwrap()) {
        Err(why) => println!("! error on remove {:} {:?}", ext_filename, why.kind()),
        Ok(_) => println!("removed {:}", ext_filename),
    }

}

fn reset_registry() {
   /* let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    {
        // always create key / ort open key..
        let path = Path::new("Software").join("VB and VBA Program Settings");
        let key = hkcu.create_subkey(&path).unwrap();
    }

    {

        let path = Path::new("Software\\VB and VBA Program Settings").join("TJNC");
        {
            let key = hkcu.create_subkey(&path).unwrap();
        }

        let key = hkcu.open_subkey(&path).unwrap();

    }*/
}

// 1) look for se folder
// 2) remove non static files
// 3) setup registry
// 4) look for driver folders
// 5) determine hardware number
// 6) install ocx or dll
// 7) look if dx is installed, if not install it
