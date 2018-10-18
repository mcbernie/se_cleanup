use std::path::{Path};

pub fn run_starter(path_str:&str) {
     let path = Path::new(path_str);
     
     use std::process::Command;

    // before start, set uwf off
    Command::new(path.join("jackstarter.exe"))
        .spawn()
        .expect("failed to start jackstarter.exe");
    // after start, commit uwf files
}