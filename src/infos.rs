


pub fn password_expires() -> Result<String, &'static str> {
    use std::process::Command;
    let output = Command::new("wmic")
        .args(&["useraccount", "where", "\"Name='nico'\"", "GET", "PasswordExpires"])
        .output()
        .expect("failed to execute wmic useraccount");

    // get second line of output
    let out = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = out.split("\n").collect();
    if lines.len() < 2 {
        return Err("Not enough lines");
    }

    let value = lines[1];
    println!("VALUE:[{:?}]", value);

    Ok(value)

}

pub fn set_password_expires_to_false() -> Result<(), &'static str> {

    use std::process::Command;
    let output = Command::new("wmic")
        .args(&["useraccount", "where", "\"Name='nico'\"", "SET", "PasswordExpires=false"])
        .output()
        .expect("failed to execute wmic useraccount to set passwordexpires");

    // get second line of output
    let out = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = out.split("\n").collect();
    if lines.len() < 2 {
        return Err("Not enough lines");
    }

    let value = lines[1];
    println!("VALUE:[{:?}]", value);

    Ok(())

}


