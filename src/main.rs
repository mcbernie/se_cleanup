#![windows_subsystem = "windows"]

extern crate clap;
extern crate rumqtt;
extern crate mqtt3;
extern crate rand;

extern crate flexi_logger;

#[macro_use]
extern crate log;

use flexi_logger::{detailed_format, Logger};


use clap::{Arg, App};
use std::path::{Path, PathBuf};
use std::fs;
use std::thread;
use std::time::Duration;

mod updater;
mod mqtt;
mod starter;
mod getfileversion;
mod winregistry;
mod infos;

fn setup_logging() {

    let mut logging_path = "Logs";

    let se_path = Path::new("c:\\Jackpot");
    if se_path.exists() {
        logging_path = "C:\\Jackpot\\Logs"
    }


    //Logger::with_env()
    let logger = Logger::with_str("info")
        .format(detailed_format)
        .print_message()
        .log_to_file()
        .directory(logging_path)
        .rotate_over_size(20000)
        .o_timestamp(true)
        .start_reconfigurable();

    if let Err(e) = logger {
        eprintln!("Logger initialization failed with {}", e)
    }

}

fn main() {

    setup_logging();

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
        .arg(Arg::with_name("nodefault")
            .short("n")
            .long("no")
            .help("Dont run with default behavior")
            .takes_value(false))
        .arg(Arg::with_name("testwmic")
            .short("w")
            .long("wmic")
            .help("Test WMIC Settings")
            .takes_value(false))                      
        .get_matches();

    if matches.is_present("testwmic") {
        infos::password_expires();
        infos::set_password_expires_to_false();
        panic!("BLA");
    }


    let se_path = Path::new("c:/jackpot");

    let mut run_shell = false;
    let mut run_mqtt = false;

    if se_path.exists() {
        if matches.is_present("nodefault")  {
            if matches.is_present("shell") {
                run_shell = true;
            }
            if matches.is_present("mqtt") {
                run_mqtt = true;
            }            
        } else {
            if cfg!(target_os = "windows") {
                info!("check for updates");
                updater::update();
                run_shell = true;
                run_mqtt = true;
            }
        }
    } else {
        /*if matches.is_present("shell") {
                run_shell = true;
        }*/
        if matches.is_present("mqtt") {
            run_mqtt = true;
        }            
    }

    if matches.is_present("prepare") {
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
    if run_shell {
        info!("start shell");
        starter::run_starter(se_path.clone().to_str().unwrap());
    }

    // MQTT Connection
    if run_mqtt {
        info!("start mqtt client connection");
        mqtt::run_mqtt(se_path.clone().to_str().unwrap());
    }

    // run endless loop....
    if run_shell || run_mqtt {
        loop {
            updater::update();
            thread::sleep(Duration::from_millis(5000));
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
        Err(why) => error!("! error on remove {:} {:?}", ext_filename, why.kind()),
        Ok(_) => info!("removed {:}", ext_filename),
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
