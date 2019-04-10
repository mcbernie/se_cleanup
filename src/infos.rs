

//password_expires return the boolean value of wmic passwordexpires entry for se_user
pub fn password_expires() -> Result<bool, &'static str> {
    use std::process::Command;
    let output = Command::new("wmic")
        .args(&["useraccount", "where", "\"Name='se_user'\"", "GET", "PasswordExpires"])
        .output()
        .expect("failed to execute wmic useraccount");

    let out = String::from_utf8_lossy(&output.stdout);

    if out.contains("FALSE") {
        Ok(false)
    } else if out.contains("TRUE") {
        Ok(true)
    } else {
        Err("NO VALID DATA FOUND")
    }

}

//set_password_expires_to_false sets the passwordexpires field for se_user to false
pub fn set_password_expires_to_false() -> Result<(), &'static str> {
    use std::process::Command;
    let output = Command::new("wmic")
        .args(&["useraccount", "where", "\"Name='se_user'\"", "SET", "PasswordExpires=false"])
        .output()
        .expect("failed to execute wmic useraccount to set passwordexpires");

    // get second line of output
    let out = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = out.split("\r\n").collect();
    if lines.len() < 2 {
        return Err("Not enough lines");
    }

    //let value = lines[1];

    Ok(())
}


