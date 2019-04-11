use std::path::{Path};
use infos::{set_password_expires_to_false, cleanup_logfiles};

pub fn run_starter(path_str:&str) {
     let path = Path::new(path_str);
     
     use std::process::Command;

    // before all other goes on, reset PasswordExpires to false
    set_password_expires_to_false();
    
    // no remove old logfiles
    cleanup_logfiles();

    // before start, set uwf off
    Command::new(path.join("jackstarter.exe"))
        .spawn()
        .expect("failed to start jackstarter.exe");
    // after start, commit uwf files
}