use std::path::Path;
use std::env;
//use std::fs;
//use std::error::Error;
use std::process;
use winregistry;
use infos;

pub fn update() {
    // lets begin!
    use getfileversion::*;

    if let Ok(path) = exists_a_update() {
        // look if version is new
        let own_path = env::current_exe().unwrap();

        let own_v = get_file_version(own_path.clone());
        let other_v = get_file_version(path.to_path_buf());

        if is_other_file_is_newer(own_v, other_v) {
            // replace
            // info!("found update, replace own file");
            winregistry::create_run_once_command("updateShell", &format!("copy /Y {:} {:}", path.display(), own_path.display()));
            let _e = infos::reboot_system();
            process::exit(0);

        }

    
    }

}

fn is_other_file_is_newer( f1:(u16,u16,u16,u16), f2:(u16,u16,u16,u16)) -> bool {
    if f2.0 > f1.0 {
        return true;
    }

    if f1.0 == f2.0 {
        if f2.1 > f1.1 {
            return true;
        }

        if f1.1 == f2.1 {
            if f2.2 > f1.2 {
                return true;
            }

            if f1.2 == f2.2 {
                if f2.3 > f1.3 {
                    return true;
                }
            }
        }
    }

    false
}

#[test]
fn check_if_is_one_greater_than_file_two() {

    assert_eq!(true, is_other_file_is_newer((1,0,0,96),(1,0,0,97)));
    assert_eq!(true, is_other_file_is_newer((1,0,0,96),(2,0,0,0)));
    assert_eq!(true, is_other_file_is_newer((1,0,0,96),(1,1,0,0)));
    assert_eq!(false, is_other_file_is_newer((1,0,0,96),(0,1,0,0)));
    assert_eq!(false, is_other_file_is_newer((1,0,0,96),(1,0,0,95)));
    assert_eq!(true, is_other_file_is_newer((0,0,0,1),(0,0,0,2)));
    assert_eq!(false, is_other_file_is_newer((0,0,0,2),(0,0,0,1)));
    assert_eq!(true, is_other_file_is_newer((0,0,1,0),(0,0,2,0)));
    assert_eq!(true, is_other_file_is_newer((0,1,1,0),(0,2,1,0)));

}

fn exists_a_update() -> Result<&'static Path, &'static str> {
    let se_update_path = Path::new("c:\\jackpot\\temp\\se_shell.exe");

    if se_update_path.exists() {
        return Ok(se_update_path);
    }

    Err("Not Found")
}