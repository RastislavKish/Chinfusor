use std::fs;

fn main() {
    let path=std::env::var("HOME").unwrap()+"/.config/chinfusor/controller.dat";
    let mut accelerated=false;

    for arg in std::env::args() {
        if arg.contains("--accelerated") {
            accelerated=true;
            }
        }

    let s=fs::read_to_string(&path).unwrap_or("".to_string());

    if !s.starts_with("Start") {
        let command=if !accelerated {
            "StartBackgroundRecording"
            } else {
            "StartAcceleratedRecording"
            };
        fs::write(&path, command).unwrap();
        }
    else {
        fs::write(&path, "StopRecording").unwrap();
        }
    }
