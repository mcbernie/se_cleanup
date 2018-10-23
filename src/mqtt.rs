use std::thread;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use rumqtt::MqttClient;
use mqtt3::{Packet, QoS};
use std::path::{Path};
use std::env;
use getfileversion;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn run_mqtt(path_str: &'static str) {
    //let path = Path::new(path_str);
    let mymac = get_mac();
    let myversion = get_file_version(env::current_exe().unwrap());
    println!(" mac is: {}", &mymac);


    let client_options = MqttOptions::new("rumqtt-demo10", "big-cash.de:1883").unwrap()
            .set_keep_alive(10)
            .set_reconnect_opts(ReconnectOptions::AfterFirstSuccess(10))
            .set_security_opts(SecurityOptions::UsernamePassword(("setest".to_string(), "testse".to_string())));

    let (mut tx, rx) = MqttClient::start(client_options);
    let topic = String::from(format!("SE/{}", &mymac));
    let sender_topic = String::from(format!("SESERVER/SE/{}", &mymac));

    tx.subscribe(vec![(topic.clone(), QoS::AtLeastOnce)]).expect("Error subscribing");

    let _t = thread::spawn(move || {
        
        let tx = Arc::new(RwLock::new(tx));

        for m in rx {
            match m {
                (Packet::Publish(p), _) => {
                    let body = p.payload.as_ref();
                    if let Ok(string) = String::from_utf8(body.clone()) {
                        match string.as_ref() {
                            "ping" => {
                                println!("get ping, send pong");
                                send_reply(Arc::clone(&tx), sender_topic.clone(), "shellpong".to_owned());
                                send_reply(Arc::clone(&tx), sender_topic.clone(), format!("Shell Version:{}", VERSION).to_owned());
                                send_reply(Arc::clone(&tx), sender_topic.clone(), format!("Product Version:v{:}.{:}.{:}.{:}", myversion.0, myversion.1, myversion.2, myversion.3).to_owned());
                            },
                            x if x.contains("shellvnc|") => {
                                println!("get vnc, open vnc");
                                open_vnc(path_str, x.to_string().split("|").collect::<Vec<_>>().last().unwrap());
                            },
                            _ => {
                                println!("{}? kenn ich nicht!", string);
                            },
                        };

                        
                    }
                }
                _ => {
                    //println!("other stuff {:?}", m);
                }
            }
        }

    });

}

fn send_reply(tx: Arc<RwLock<MqttClient>>, topic: String, reply: String) {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        let mut client = tx.write().expect("Error acquiring a write lock to the MQTT Client");
        client.publish(topic, QoS::AtLeastOnce, reply.as_bytes().to_vec()).expect("Error publishing answer");
    });
}

/// open vnc single client click on client
fn open_vnc(path_str: &str, port: &str) {
    use std::error::Error;
    use std::io::prelude::*;
    use std::fs::OpenOptions;
    use std::fs::copy;

    println!("open_vnc to port: {}", port);
    // Files and path
    let path = Path::new(path_str);
    let template_path = path.join("helpdesk.vorlage.txt");
    let helpdesk_path = path.join("Helpdesk.txt");
    let remote_command = path.join("remotehelp.exe");


    let file_contents = format!("\n[HOST]\nHilfe\n-connect bigcash.dnsalias.net:{} -noregistry", port);

    // copy template..
    // from c#: File.Copy("c:\\Jackpot\\helpdesk.vorlage.txt", "c:\\Jackpot\\Helpdesk.txt", true);
    if let Err(e) = copy(&template_path, &helpdesk_path) {
        eprintln!("Could not copy template {} to helpdesk file {}: {}", template_path.display(), helpdesk_path.display(), e.description());
        return;
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&helpdesk_path)
        .unwrap();


    if let Err(e) = file.write(file_contents.as_bytes()) {
        eprintln!("Could not write to helpdesk file {}: {}", helpdesk_path.display(), e.description());
        return;
    }

    // start thread 
    use std::process::Command;
    let remote_command_display = remote_command.display();
    if let Err(r) = Command::new(&remote_command).spawn() {
        eprintln!("Error on start remotehelp {}: {}", remote_command_display, r.description());
    }
}


fn get_mac() -> String {
 
    let mut mac : String = "000000000000".to_string();
    
    match get_mac_address() {
        Ok(Some(ma)) => {
            if ma.len() > 1 {
                println!("little mac warning.. got more than 1 mac..");
            }
            mac = ma[0].to_string();
        }
        Ok(None) => println!("no mac found"),
        Err(e) => println!("{:?}", e),
    }

    mac.to_uppercase()
}


fn get_mac_address() ->  Result<Option<Vec<String>>, &'static str> {
    use std::process::Command;
    let output = if cfg!(target_os = "windows") {
        Command::new("getmac")
            .args(&["/FO", "csv", "/V"])
            .output()
            .expect("failed to execute getmac")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo \"Physical Address\", \"Transportname\"")
            .output()
            .expect("failed to execute process")
    };

    // get second line of output
    let out = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = out.split("\n").collect();

    if lines.len() < 2 {
        return Err("Not enough lines");
    }

    let mut macs: Vec<String> = Vec::new();

    for mac_l in lines {
        if mac_l.contains("Ethernet") {
            let mac_line: Vec<&str> = mac_l.split(",").collect();
            let mac_content = mac_line[2].clone();
            let the_mac = mac_content.replace("\"", "").replace("-", "");
            macs.push(the_mac);
        }
    }

    Ok(Some(macs))
}