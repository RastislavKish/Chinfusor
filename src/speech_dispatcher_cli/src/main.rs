use speech_dispatcher::{Connection, Priority};

fn main() {
    let connection=Connection::open("speech_dispatcher_cli", "speech_dispatcher_cli_connection", "", speech_dispatcher::Mode::Threaded).unwrap();

    loop {
        let mut input=String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input=input.replace("\\n", "\n").trim().to_string();

        if input=="quit" {
            break;
            }

        if input.contains("=") {
            let parts: Vec<String>=input.split('=').map(|i| i.to_string()).collect();
            if parts.len()!=2 {
                println!("Invalid configuration");
                continue;
                }

            match &parts[0][..] {
                "module" => {
                    connection.set_output_module(parts[1].clone()).unwrap();
                    },
                "language" => {
                    connection.set_language(&parts[1]).unwrap();
                    },
                "pitch" => {
                    if let Ok(n)=parts[1].parse::<i32>() {
                        connection.set_voice_pitch(n).unwrap();
                        }
                    else {
                        println!("Invalid inpuut");
                        }
                    },
                "rate" => {
                    if let Ok(n)=parts[1].parse::<i32>() {
                        connection.set_voice_rate(n).unwrap();
                        }
                    else {
                        println!("Invalid inpuut");
                        }
                    },
                "volume" => {
                    if let Ok(n)=parts[1].parse::<i32>() {
                        connection.set_volume(n).unwrap();
                        }
                    else {
                        println!("Invalid inpuut");
                        }
                    },
                "pause" => {
                    connection.pause().unwrap();
                    },
                "resume" => {
                    connection.resume().unwrap();
                    },
                _ => {
                    println!("Unknown command");
                    continue;
                    },
                };
            }
        else {
            connection.say(Priority::Message, input);
            }
        }
    }
