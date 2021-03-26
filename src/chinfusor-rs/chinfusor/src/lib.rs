use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::thread;

use subprocess::{Exec, Popen, Redirection};
use text_processor::{Alphabet, LanguageChunk};

pub enum SdInputCommand {
    Init,
    Audio(AudioSettings),
    LogLevel(LogLevelSettings),
    Set(SpeechSettings),
    Speak(String),
    Char(char),
    Key(String),
    Pause,
    Stop,
    Quit,
    }
pub struct AudioSettings {
    lines: Vec<String>,
    }
impl AudioSettings {

    pub fn new(lines: Vec<String>) -> AudioSettings {
        AudioSettings {lines}
        }

    pub fn generate_sd_command(&self) -> String {
        format!("AUDIO\n{}\n.\n", self.lines.join("\n"))
        }
    }
pub struct SpeechSettings {
    lines: Vec<String>,
    }
impl SpeechSettings {

    pub fn new(lines: Vec<String>) -> SpeechSettings {
        SpeechSettings {lines}
        }

    pub fn generate_sd_command(&self) -> String {
        format!("SET\n{}\n.\n", self.lines.join("\n"))
        }
    pub fn generate_sd_command_parametrized(language: &str) -> String {
        format!("SET\nvolume=100\npitch=7\nvoice=male1\nlanguage={}\n.\n", language)
        }
    pub fn generate_sd_command_from_engine_configuration(config: &SpeechEngineConfiguration) -> String {
        format!("SET\nlanguage={}\nvoice={}\npunctuation_mode={}\npitch={}\nrate={}\nvolume={}\n.\n", config.language, config.voice, config.punctuation_mode, config.pitch, config.rate, config.volume)
        }
    pub fn generate_sd_command_from_pitch(pitch: i32) -> String {
        format!("SET\npitch={}\n.\n", pitch)
        }
    }
pub struct LogLevelSettings {
    lines: Vec<String>,
    }
impl LogLevelSettings {

    pub fn new(lines: Vec<String>) -> LogLevelSettings {
        LogLevelSettings {lines}
        }

    pub fn generate_sd_command(&self) -> String {
        format!("LOGLEVEL\n{}\n.\n", self.lines.join("\n"))
        }
    }
pub struct Process {
    process: Popen,
    stdin: File,
    stdout_receiver: mpsc::Receiver<String>,
    //reader_thread_handle: thread::JoinHandle<()>,
    }
impl Process {

    pub fn new(file_path: &str, arg: &str, firejailed: bool) -> Process {
        let process=if !firejailed {
            Exec::cmd(file_path).arg(arg).stdin(Redirection::Pipe).stdout(Redirection::Pipe).popen().expect(&format!("Unable to start {}.", file_path))
            } else {
            Exec::cmd("firejail").args(&vec![file_path, arg]).stdin(Redirection::Pipe).stdout(Redirection::Pipe).stderr(Redirection::Pipe).popen().expect(&format!("Unable to start {}.", file_path))
            };
        let stdin=if let Some(f)=&process.stdin {
            (*f).try_clone().unwrap()
            } else {
            panic!("Unable to take stdin of {}.", file_path);
            };
        let stdout=if let Some(f)=&process.stdout {
            (*f).try_clone().unwrap()
            } else {
            panic!("Unable to take stdout of {}.", file_path);
            };

        let (stdout_transmitter, stdout_receiver)=mpsc::channel::<String>();

        thread::spawn(move || Process::stdout_processing_loop(stdout, stdout_transmitter));

        Process {process, stdin, stdout_receiver}
        }

    pub fn read_line(&mut self) -> Option<String> {
        if let Ok(line)=self.stdout_receiver.try_recv() {
            return Some(line);
            }
        else {
            return None;
            }
        }
    pub fn write(&mut self, input: &str) {
        self.stdin.write(input.as_bytes()).unwrap();
        }
    pub fn write_line(&mut self, input: &str) {
        self.stdin.write(input.as_bytes()).unwrap();
        self.stdin.write(&vec! ['\n' as u8]).unwrap();
        }
    pub fn wait_for_exit(&mut self) {
        self.process.wait().unwrap();
        }

    pub fn stdout_processing_loop(mut f: File, stdout_transmitter: mpsc::Sender<String>) {
        let mut buffer: Vec<u8>=Vec::with_capacity(1000);
        let mut small_buffer: Vec<u8>=vec![0;1];

        loop {
            if let Ok(n)=f.read(&mut small_buffer) {
                if n==0 {
                    break;
                    }
                }
            else {
                break;
                }
            if small_buffer[0]!='\n' as u8 {
                buffer.push(small_buffer[0]);
                }
            else {
                stdout_transmitter.send(String::from_utf8(buffer.clone()).unwrap()).unwrap();
                buffer.clear();
                }
            }
        }
    }
pub struct Config {
    pub latin: SpeechEngineConfiguration,
    pub chinese: SpeechEngineConfiguration,
    pub cyrillic: SpeechEngineConfiguration,
    }
impl Config {

    pub fn new() -> Config {
        Config {latin: SpeechEngineConfiguration::new("en"), chinese: SpeechEngineConfiguration::new("cmn"), cyrillic: SpeechEngineConfiguration::new("ru")}
        }

    pub fn load_from_file(&mut self, file_path: &str) {
        if let Ok(s)=fs::read_to_string(file_path) {
            for line in s.lines() {
                if line.starts_with("latin") {
                    self.latin.load_from_string(line);
                    }
                else if line.starts_with("chinese") {
                    self.chinese.load_from_string(line);
                    }
                else if line.starts_with("cyrillic") {
                    self.cyrillic.load_from_string(line);
                    }
                }
            }
        }
    }
pub struct SpeechEngineConfiguration {
    pub module: String,
    pub arg: String,
    pub language: String,
    pub voice: String,
    pub punctuation_mode: String,
    pub pitch: i32,
    pub capitals_pitch: i32,
    pub rate: i32,
    pub volume: i32,
    pub firejailed: bool,
    }
impl SpeechEngineConfiguration {

    pub fn new(language: &str) -> SpeechEngineConfiguration {
        SpeechEngineConfiguration {module: "/usr/lib/speech-dispatcher-modules/sd_espeak-ng".to_string(), arg: "/etc/speech-dispatcher/modules/espeak-ng.conf".to_string(), language: language.to_string(), voice: "male1".to_string(), punctuation_mode: "some".to_string(), pitch: 10, capitals_pitch: 50, rate: 2, volume: 100, firejailed: false}
        }
    pub fn load_from_string(&mut self, line: &str) {
        let settings: Vec<String>=line.split(',').map(|i| i.to_string()).collect();

        if settings.len()!=11 {
            return;
            }

        let module=settings[1].clone();
        let arg=settings[2].clone();
        let language=settings[3].clone();
        let voice=settings[4].clone();
        let punctuation_mode=settings[5].clone();
        let pitch=if let Ok(n)=settings[6].parse::<i32>() {
            if n>=-100 && n<=100 {
                n
                } else {
                self.pitch
                }
            } else {
            self.pitch
            };
        let capitals_pitch=if let Ok(n)=settings[7].parse::<i32>() {
            if n>=-100 && n<=100 {
                n
                } else {
                self.capitals_pitch
                }
            } else {
            self.capitals_pitch
            };
        let rate=if let Ok(n)=settings[8].parse::<i32>() {
            if n>=-100 && n<=100 {
                n
                } else {
                self.rate
                }
            } else {
            self.rate
            };
        let volume=if let Ok(n)=settings[9].parse::<i32>() {
            if n>=-100 && n<=100 {
                n
                } else {
                self.volume
                }
            } else {
            self.volume
            };
        let firejailed=match &settings[10][..] {
            "yes" | "true" => true,
            _ => false,
            };

        self.module=module;
        self.arg=arg;
        self.language=language;
        self.voice=voice;
        self.punctuation_mode=punctuation_mode;
        self.pitch=pitch;
        self.capitals_pitch=capitals_pitch;
        self.rate=rate;
        self.volume=volume;
        self.firejailed=firejailed;
        }
    }

pub fn run(config: Config) {
    let mut engines=[Process::new("/usr/lib/speech-dispatcher-modules/sd_espeak-ng", "/etc/speech-dispatcher/modules/espeak-ng.conf", config.latin.firejailed),
    Process::new("/usr/lib/speech-dispatcher-modules/sd_espeak-ng", "/etc/speech-dispatcher/modules/espeak-ng.conf", config.chinese.firejailed),
    Process::new("/usr/lib/speech-dispatcher-modules/sd_espeak-ng", "/etc/speech-dispatcher/modules/espeak-ng.conf", config.cyrillic.firejailed)];

    let (latin_engine, chinese_engine, cyrillic_engine)=(0, 1, 2);
    let mut currently_speaking_engine=latin_engine;

    let mut currently_spoken_text: Vec<LanguageChunk>=Vec::new();
    let mut currently_spoken_text_position: usize=0;
    let mut capitalized=false;
    let mut original_pitch=config.latin.pitch;
    let mut speaking=false;
    let (sd_input_transmitter, sd_input_receiver)=mpsc::channel::<SdInputCommand>();

    let default_language=config.latin.language.clone();
    thread::spawn(move || sd_input_processing_loop(sd_input_transmitter, &default_language));

    loop {
        let sd_input=if speaking {
            match sd_input_receiver.try_recv() {
                Ok(result) => Some(result),
                _ => None,
                }
            } else {
            match sd_input_receiver.recv() {
                Ok(result) => Some(result),
                _ => None,
                }
            };

        if let Some(sd_command)=sd_input {

            match sd_command {
                SdInputCommand::Init => {
                    for engine in engines.iter_mut() {
                        engine.write_line("INIT");
                        }
                    },
                SdInputCommand::Audio(settings) => {
                    for engine in engines.iter_mut() {
                        engine.write(&settings.generate_sd_command());
                        }

                    engines[latin_engine].write(&SpeechSettings::generate_sd_command_from_engine_configuration(&config.latin));
                    engines[chinese_engine].write(&SpeechSettings::generate_sd_command_from_engine_configuration(&config.chinese));
                    engines[cyrillic_engine].write(&SpeechSettings::generate_sd_command_from_engine_configuration(&config.cyrillic));
                    },
                SdInputCommand::Set(_settings) => {
                    //currently_speaking_engine.write(&settings.generate_sd_command());
                    //another_engine.write(&settings.generate_sd_command());
                    },
                SdInputCommand::LogLevel(settings) => {
                    for engine in engines.iter_mut() {
                        engine.write(&settings.generate_sd_command());
                        }
                    },
                SdInputCommand::Speak(text) => {
                    if !speaking {

                        currently_spoken_text=text_processor::parse_text(&text.replace("<speak>", "").replace("</speak>", ""), true);

                        if currently_spoken_text.len()>0 {
                            currently_spoken_text_position=0;

                            let text=match &currently_spoken_text[0] {
                                LanguageChunk::Latin(text) => {
                                    currently_speaking_engine=latin_engine;
                                    text
                                    },
                                LanguageChunk::Chinese(text) => {
                                    currently_speaking_engine=chinese_engine;
                                    text
                                    },
                                LanguageChunk::Cyrillic(text) => {
                                    currently_speaking_engine=cyrillic_engine;
                                    text
                                    },
                                };

                            engines[currently_speaking_engine].write(&format!("SPEAK\n<speak>{}</speak>\n.\n", text));

                            speaking=true;
                            println!("701 BEGIN");
                            }
                        else {
                            println!("701 BEGIN");
                            println!("702 END");
                            }
                        }
                    },
                SdInputCommand::Key(text) => {
                    engines[latin_engine].write(&format!("KEY\n{}\n.\n", text));

                    speaking=true;

                    println!("701 BEGIN");
                    }
                SdInputCommand::Char(ch) => {
                    let (engine, capitalized_pitch)=match text_processor::identify_character(ch) {
                        Alphabet::Latin => {
                            currently_speaking_engine=latin_engine;
                            original_pitch=config.latin.pitch;
                            (&mut engines[latin_engine], config.latin.capitals_pitch)
                            },
                        Alphabet::Chinese => {
                            currently_speaking_engine=chinese_engine;
                            original_pitch=config.chinese.pitch;
                            (&mut engines[chinese_engine], config.chinese.capitals_pitch)
                            },
                        Alphabet::Cyrillic => {
                            currently_speaking_engine=cyrillic_engine;
                            original_pitch=config.cyrillic.pitch;
                            (&mut engines[cyrillic_engine], config.cyrillic.capitals_pitch)
                            },
                        };

                    if ch.is_uppercase() {
                        engine.write(&SpeechSettings::generate_sd_command_from_pitch(capitalized_pitch));
                        capitalized=true;
                        }

                    engine.write(&format!("CHAR\n{}\n.\n", ch));

                    speaking=true;

                    println!("701 BEGIN");
                    },
                SdInputCommand::Pause => {
                    engines[currently_speaking_engine].write_line("PAUSE");
                    loop {
                        while let Some(line)=engines[currently_speaking_engine].read_line() {

                            if line=="704 PAUSE" {
                                speaking=false;
                                println!("704 PAUSE");
                                break;
                                }
                            else if line=="702 END".to_string() {
                                speaking=false;
                                println!("702 END");
                                break;
                                }
                            else if line.starts_with("700") {
                                println!("{}", line);
                                }
                            }
                        if !speaking {
                            break;
                            }
                        std::thread::sleep(std::time::Duration::from_millis(1));
                        }

                    if capitalized {
                        engines[currently_speaking_engine].write(&SpeechSettings::generate_sd_command_from_pitch(original_pitch));
                        capitalized=false;
                        }

                    continue;
                    },
                SdInputCommand::Stop => {
                    engines[currently_speaking_engine].write_line("STOP");
                    loop {
                        while let Some(line)=engines[currently_speaking_engine].read_line() {

                            if line=="703 STOP" {
                                speaking=false;
                                println!("703 STOP");
                                break;
                                }
                            else if line=="702 END".to_string() {
                                speaking=false;
                                println!("702 END");
                                break;
                                }
                            else if line.starts_with("700") {
                                println!("{}", line);
                                }
                            }
                        if !speaking {
                            break;
                            }
                        std::thread::sleep(std::time::Duration::from_millis(1));
                        }

                    if capitalized {
                        engines[currently_speaking_engine].write(&SpeechSettings::generate_sd_command_from_pitch(original_pitch));
                        capitalized=false;
                        }

                    continue;
                    },
                SdInputCommand::Quit => {
                    for engine in engines.iter_mut() {
                        engine.write_line("QUIT");
                        }

                    println!("210 OK QUIT");

                    break;
                    },
                };
            }

        if speaking {
            while let Some(line)=engines[currently_speaking_engine].read_line() {
                //println!("{}", line);
                if line.starts_with("700") {
                    println!("{}", line);
                    }
                else if line=="702 END".to_string() {
                    currently_spoken_text_position+=1;
                    if currently_spoken_text_position>=currently_spoken_text.len() {
                        speaking=false;
                        if capitalized {
                            engines[currently_speaking_engine].write(&SpeechSettings::generate_sd_command_from_pitch(original_pitch));
                            capitalized=false;
                            }

                        println!("702 END");

                        continue;
                        }

                    let text=match &currently_spoken_text[currently_spoken_text_position] {
                        LanguageChunk::Latin(text) => {
                            currently_speaking_engine=latin_engine;
                            text
                            },
                        LanguageChunk::Chinese(text) => {
                            currently_speaking_engine=chinese_engine;
                            text
                            },
                        LanguageChunk::Cyrillic(text) => {
                            currently_speaking_engine=cyrillic_engine;
                            text
                            },
                        };

                    engines[currently_speaking_engine].write(&format!("SPEAK\n<speak>{}</speak>\n.\n", text));
                    }

                }
            std::thread::sleep(std::time::Duration::from_millis(1));
            }
        }

    for engine in engines.iter_mut() {
        engine.wait_for_exit();
        }
    }

fn sd_input_processing_loop(tx: mpsc::Sender<SdInputCommand>, default_language: &str) {
    let stdin=std::io::stdin();
    loop {
        let mut input=String::new();
        stdin.read_line(&mut input).unwrap();

        match input.trim() {
            "INIT" => {
                println!("299-Chinfusor: Initialized successfully.");
                println!("299 OK LOADED SUCCESSFULLY");
                tx.send(SdInputCommand::Init).unwrap();
                },
            "AUDIO" => {
                let mut lines: Vec<String>=Vec::new();
                println!("207 OK RECEIVING AUDIO SETTINGS");

                loop {
                    let mut input=String::new();
                    stdin.read_line(&mut input).unwrap();
                    input=input.trim().to_string();

                    if input=="." {
                        break;
                        }

                    lines.push(input);
                    }

                println!("203 OK AUDIO INITIALIZED");
                tx.send(SdInputCommand::Audio(AudioSettings::new(lines))).unwrap();
                },
            "LOGLEVEL" => {
                let mut lines: Vec<String>=Vec::new();
                println!("207 OK RECEIVING LOGLEVEL SETTINGS");

                loop {
                    let mut input=String::new();
                    stdin.read_line(&mut input).unwrap();
                    input=input.trim().to_string();

                    if input=="." {
                        break;
                        }

                    lines.push(input);
                    }

                println!("203 OK LOGLEVEL SET");
                tx.send(SdInputCommand::LogLevel(LogLevelSettings::new(lines))).unwrap();
                },
            "SET" => {
                let mut lines: Vec<String>=Vec::new();
                println!("203 OK RECEIVING SETTINGS");

                loop {
                    let mut input=String::new();
                    stdin.read_line(&mut input).unwrap();
                    input=input.trim().to_string();

                    if input=="." {
                        break;
                        }

                    lines.push(input);
                    }

                println!("203 OK SETTINGS RECEIVED");
                tx.send(SdInputCommand::Set(SpeechSettings::new(lines))).unwrap();
                },
            "SPEAK" => {
                let mut text=String::new();
                println!("202 OK RECEIVING MESSAGE");

                loop {
                    let mut line=String::new();
                    stdin.read_line(&mut line).unwrap();

                    match &line[..] {
                        ".\n" => {
                            break;
                            }
                        _ => {
                            text+=&line;
                            }
                        };

                    }

                println!("200 OK SPEAKING");
                tx.send(SdInputCommand::Speak(text)).unwrap();
                },
            "CHAR" => {
                let mut character_string=String::new();
                let mut the_rest=String::new();

                println!("202 OK RECEIVING MESSAGE");

                stdin.read_line(&mut character_string).unwrap();
                stdin.read_line(&mut the_rest).unwrap();

                println!("200 OK SPEAKING");

                if let Some(ch)=character_string.chars().next() {
                    tx.send(SdInputCommand::Char(ch)).unwrap();
                    }
                },
            "KEY" => {
                let mut key_string=String::new();
                let mut the_rest=String::new();

                stdin.read_line(&mut key_string).unwrap();
                stdin.read_line(&mut the_rest).unwrap();

                tx.send(SdInputCommand::Key(key_string)).unwrap();
                },
            "PAUSE" => {
                tx.send(SdInputCommand::Pause).unwrap();
                },
            "STOP" => {
                tx.send(SdInputCommand::Stop).unwrap();
                },
            "QUIT" => tx.send(SdInputCommand::Quit).unwrap(),
            "LIST VOICES" => {
                println!("200-Default	{}	none", default_language);
                println!("200 OK VOICE LIST SENT");
                },
            _ => {},
            };
        }
    }

