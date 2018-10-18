extern crate clap;
extern crate rumqtt;
extern crate mqtt3;
extern crate interfaces;
//extern crate winreg;

use clap::{Arg, App};

use std::path::{Path, PathBuf};
use std::fs;

use std::thread;
use std::time::Duration;


mod mqtt;
mod starter;

//use winreg::RegKey;
//use winreg::enums::*;

fn main() {
    let matches = App::new("se_prepper")
        .version("1.0")
        .about("prepare a star entertainer image")
        .author("Nicolas Brueggemann")
        .arg(Arg::with_name("prepare")
            .short("p")
            .long("prepare")
            .help("Make full preperation")
            .takes_value(false))
        .arg(Arg::with_name("shell")
            .short("s")
            .long("shell")
            .help("Run as Shell")
            .takes_value(false))
        .arg(Arg::with_name("mqtt")
            .short("m")
            .long("mqtt")
            .help("Start MQTT Connection")
            .takes_value(false))            
        .get_matches();

    let se_path = Path::new("c:/jackpot");
    if se_path.exists() == false {
        eprintln!("SE does not exists");
    }

    if matches.is_present("prepare") {
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

    // RUN AS SHELL
    if matches.is_present("shell") {
        println!("start shell");
        starter::run_starter(se_path.clone().to_str().unwrap());
    }

    // MQTT Connection
    if matches.is_present("mqtt") {
        println!("start mqtt client connection");
        mqtt::run_mqtt(se_path.clone().to_str().unwrap());
    }


    // run endless loop....
    if matches.is_present("shell") || matches.is_present("mqtt") {
        loop {
            thread::sleep(Duration::from_millis(1000));
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

//fn reset_registry() {
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
//}

// 1) look for se folder
// 2) remove non static files
// 3) setup registry
// 4) look for driver folders
// 5) determine hardware number
// 6) install ocx or dll
// 7) look if dx is installed, if not install it
