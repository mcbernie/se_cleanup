use std::path::{Path, PathBuf};
use std::env;
use std::fs;


pub fn update() {
    // lets begin!
    use getfileversion::*;

    if let Ok(path) = exists_a_update() {
        // look if version is new
        let own_v = get_file_version(env::current_exe().unwrap());
        let other_v = get_file_version(path.to_path_buf());

        if is_other_file_is_newer(own_v, other_v) {
            // replace
            println!("found update, replace own file");
            fs::copy(path.to_path_buf(), env::current_exe().unwrap());
        }

    
    }

}

fn is_other_file_is_newer( f1:(u16,u16,u16,u16), f2:(u16,u16,u16,u16)) -> bool {
    if f1.0 > f2.0 {
        return true;
    }

    if f1.0 == f2.0 {
        if f1.1 > f2.1 {
            return true;
        }

        if f1.1 == f2.1 {
            if f1.2 > f2.2 {
                return true;
            }

            if f1.2 == f2.2 {
                if f1.3 > f2.3 {
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
    let se_update_path = Path::new("c:/jackpot/temp");

    if se_update_path.exists() {
        return Ok(se_update_path);
    }

    Err("Not Found")
}