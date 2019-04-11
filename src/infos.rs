

///return the boolean value of wmic passwordexpires entry for se_user
pub fn password_expires() -> Result<bool, String> {
    use std::process::Command;
    let output = Command::new("cmd")
        .args(&["/C", "wmic useraccount where Name='se_user' GET PasswordExpires"])
        .output()
        .expect("failed to execute wmic useraccount");

    let out = String::from_utf8_lossy(&output.stdout);
    let out_err = String::from_utf8_lossy(&output.stderr);


    if out.contains("FALSE") {
        Ok(false)
    } else if out.contains("TRUE") {
        Ok(true)
    } else {
        Err(out_err.to_string())
    }

}

///sets the passwordexpires field for se_user to false
pub fn set_password_expires_to_false() {
    use std::process::Command;
    let output = Command::new("cmd")
        .args(&["/C","wmic useraccount where Name='se_user' SET PasswordExpires=false"])
        .output()
        .expect("failed to execute wmic useraccount to set passwordexpires");
    let _out = String::from_utf8_lossy(&output.stdout);
}

///instant reboot of the system
pub fn reboot_system() {
    use std::process::Command;
    let _output = Command::new("cmd")
        .args(&["/C", "wmic os where Primary='TRUE' reboot"])
        .output()
        .expect("failed to execute Primary");
}

///removes all files in Logs dir of Jackpot folder if they older than 30 days
pub fn cleanup_logfiles() {
    use std::process::Command;
    let output = Command::new("cmd")
        .args(&["/C", "ForFiles /p \"C:\\Jackpot\\Logs\" /s /d -30 /c \"cmd /c del @file\""])
        .output()
        .expect("failed to remove all logs older than 30 days");

    let _out = String::from_utf8_lossy(&output.stdout);
}

//next step optimising
/*fn wmic(command: &'static str) -> Result<String, String> {
    use std::process::Command;
    let output = Command::new("cmd")
        .args(&["/C", "wmic", command])
        .output()
        .expect(&format!("Error on call wmic command: {:?}", command));

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}*/
