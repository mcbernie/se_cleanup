use std::thread;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use rumqtt::{MqttOptions, MqttClient, ReconnectOptions, SecurityOptions};
use mqtt3::{Packet, QoS};
use std::path::{Path};


pub fn run_mqtt(path: &Path) {
    let mymac = get_mac();
    println!("my mac is: {}", &mymac);

    println!("set client options");
    let client_options = MqttOptions::new("rumqtt-demo10", "big-cash.de:1883").unwrap()
            .set_keep_alive(10)
            .set_reconnect_opts(ReconnectOptions::AfterFirstSuccess(10))
            .set_security_opts(SecurityOptions::UsernamePassword(("setest".to_string(), "testse".to_string())));

    println!("start connection...");
    let (mut tx, rx) = MqttClient::start(client_options);
    let topic = String::from(format!("SE/{}", &mymac));
    let sender_topic = String::from(format!("SESERVER/SE/{}", &mymac));

    println!("subscribe...");
    tx.subscribe(vec![(topic.clone(), QoS::AtLeastOnce)]).expect("Error subscribing");

    let t = thread::spawn(move || {
        
        let tx = Arc::new(RwLock::new(tx));

        for m in rx {
            match m {
                (Packet::Publish(p), _) => {
                    let body = p.payload.as_ref();
                    if let Ok(string) = String::from_utf8(body.clone()) {
                        match string.as_ref() {
                            "ping" => {
                                println!("get ping, send pong");
                                send_reply(Arc::clone(&tx), sender_topic.clone(), "pong".to_owned());
                            },
                            x if x.contains("vnc|") => {
                                println!("get vnc, open vnc");
                                open_vnc(path, x.to_string().split("|").collect::<Vec<_>>().last().unwrap());
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
fn open_vnc(path: &Path, port: &str) {
    use std::error::Error;
    use std::io::prelude::*;
    use std::fs::OpenOptions;
    use std::fs::copy;

    println!("open_vnc to port: {}", port);
    // Files and path
    let template_path = path.join("helpdesk.vorlage.txt");
    let path = path.join("Helpdesk.txt");
    let remote_command = path.join("remotehelp.exe");


    let file_contents = format!("\n[HOST]\nHilfe\n-connect bigcash.dnsalias.net:{} -noregistry", port);

    // copy template..
    // from c#: File.Copy("c:\\Jackpot\\helpdesk.vorlage.txt", "c:\\Jackpot\\Helpdesk.txt", true);
    if let Err(e) = copy(&template_path, &path) {
        eprintln!("Could not copy template {} to helpdesk file {}: {}", template_path.display(), path.display(), e.description());
        return;
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)
        .unwrap();


    if let Err(e) = file.write(file_contents.as_bytes()) {
        eprintln!("Could not write to helpdesk file {}: {}", path.display(), e.description());
        return;
    }

    // start thread 
    use std::process::Command;
    Command::new(remote_command)
        .spawn()
        .expect("failed to start jackstarter.exe");

}


fn get_mac() -> String {
    use interfaces::{Interface};
    let ifs = Interface::get_all().expect("could not get interfaces");

    let mut mac : String = "0000000000".to_string();
    for i in ifs.iter() {
        if i.is_up() && !i.is_loopback() {
            mac = i.hardware_addr().unwrap().as_bare_string();
            break;
        }
    }
    mac.to_uppercase()
}