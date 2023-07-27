pub mod text_processor;

use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use lazy_static::lazy_static;
use notify::{event::EventKind, RecursiveMode, Watcher};
use regex::Regex;
use subprocess::{Exec, Popen, Redirection};
use text_processor::LanguageChunk;

lazy_static! {
    static ref MINI_THREAD_POOL: MiniThreadPool=MiniThreadPool::new();
    static ref UNICODE_RANGES_MATCHING_REGEX: Regex=Regex::new(
    r"u((0x[\dabcdefABCDEF]+)|(\d+))-u((0x[\dabcdefABCDEF]+)|(\d+))" //Matches strings of style u0x12D-u123
    ).unwrap();
    static ref COLON_SEARCHING_REGEX: Regex=Regex::new(": ?").unwrap();
    }

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
pub enum MiniThreadPoolRequest {
    ReadUntilSdEndSignal(Arc<Mutex<File>>, mpsc::Sender<String>),
    }
pub struct MiniThreadPool {
    requests_transmitter: Mutex<mpsc::Sender<MiniThreadPoolRequest>>,
    }
impl MiniThreadPool {

    pub fn new() -> MiniThreadPool {
        let (tx, rx)=mpsc::channel::<MiniThreadPoolRequest>();

        thread::spawn(move || MiniThreadPool::mini_thread_pool_loop(rx));

        MiniThreadPool {requests_transmitter: Mutex::new(tx)}
        }

    pub fn get_requests_transmitter(&self) -> mpsc::Sender<MiniThreadPoolRequest> {
        let requests_transmitter=self.requests_transmitter.lock().unwrap();
        requests_transmitter.clone()
        }

    fn mini_thread_pool_loop(requests_receiver: mpsc::Receiver<MiniThreadPoolRequest>) {

        let mut stdout_reading_buffer: Vec<u8>=Vec::with_capacity(1000);
        let mut small_stdout_reading_buffer: Vec<u8>=vec![0;1];

        while let Ok(request)=requests_receiver.recv() {
            match request {
                MiniThreadPoolRequest::ReadUntilSdEndSignal(stdout, stdout_transmitter) => {
                    let mut stdout=stdout.lock().unwrap();
                    let mut end_signal_detected=false;

                    while !end_signal_detected {
                        if let Ok(n)=stdout.read(&mut small_stdout_reading_buffer) {
                            if n==0 {
                                break;
                                }
                            }
                        else {
                            break;
                            }

                        if small_stdout_reading_buffer[0]!='\n' as u8 {
                            stdout_reading_buffer.push(small_stdout_reading_buffer[0]);
                            }
                        else {
                            let line=String::from_utf8(stdout_reading_buffer.clone()).unwrap();

                            end_signal_detected=line=="702 END" || line=="703 STOP" || line=="704 PAUSE";

                            stdout_transmitter.send(line).unwrap();

                            stdout_reading_buffer.clear();
                            }
                        }
                    },
                };
            }
        }
    }
pub struct Process {
    process: Popen,
    stdin: File,
    stdout: Arc<Mutex<File>>,
    stdout_transmitter: mpsc::Sender<String>,
    stdout_receiver: mpsc::Receiver<String>,
    mini_thread_pool_requests_transmitter: mpsc::Sender<MiniThreadPoolRequest>,
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
            Arc::new(Mutex::new((*f).try_clone().unwrap()))
            } else {
            panic!("Unable to take stdout of {}.", file_path);
            };

        let (stdout_transmitter, stdout_receiver)=mpsc::channel::<String>();

        let mini_thread_pool_requests_transmitter=MINI_THREAD_POOL.get_requests_transmitter();

        Process {process, stdin, stdout, stdout_transmitter, stdout_receiver, mini_thread_pool_requests_transmitter}
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

    pub fn activate_asynchronous_reading_until_sd_end_signal(&self) {
        self.mini_thread_pool_requests_transmitter.send(MiniThreadPoolRequest::ReadUntilSdEndSignal(self.stdout.clone(), self.stdout_transmitter.clone())).unwrap();
        }
    }
pub struct Config {
    pub engines: Vec<SpeechEngineConfiguration>,
    pub punctuation_characters: HashSet<char>,
    }
impl Config {

    pub fn new() -> Config {
        let engines=vec![SpeechEngineConfiguration::new("latin")];
        let punctuation_characters: HashSet<char>=[',', '.', '?', '，', '。', '？', '-', ' ', ':', '\r', '\n'].iter().cloned().collect();

        Config { engines, punctuation_characters }
        }

    pub fn load_alphabets_from_file(&mut self, file_path: &str) {
        if let Ok(s)=fs::read_to_string(file_path) {
            self.load_alphabets_from_string(&s);
            }
        }
    pub fn load_alphabets_from_string(&mut self, s: &str) {
        let mut engines=Vec::new();
        for line in s.lines() {
            if line.starts_with("#") {
                continue;
                }

            if let Ok(engine)=SpeechEngineConfiguration::load_from_string(line) {
                engines.push(engine);
                }
            }

        //Latin engine is necessary and has to be specified exactly once, so we need to remove duplicates and check, whether it's there at all. We also need to make sure, that it's first in the list.

        let mut latin_position=None;
        for (i, engine) in engines.iter().enumerate() {
            if engine.unicode_ranges.len()==0 {
                latin_position=Some(i);
                }
            }

        if let Some(position)=latin_position {

            let mut engines_to_remove=Vec::new();
            for (i, engine) in engines.iter().enumerate().rev() {
                if engine.unicode_ranges.len()==0 {
                    if i>position {
                        engines_to_remove.push(position);
                        }
                    else {
                        break;
                        }
                    }
                }
            for i in engines_to_remove {
                engines.remove(i);
                }

            if position>0 {
                let latin_engine=engines.remove(position);
                engines.insert(0, latin_engine);
                }
            }
        else {
            engines.insert(0, SpeechEngineConfiguration::new("latin"));
            }

        self.engines=engines;
        }
    pub fn load_configuration_from_file(&mut self, file_path: &str) {
        if let Ok(s)=fs::read_to_string(file_path) {
            self.load_configuration_from_string(&s);
            }
        }
    pub fn load_configuration_from_string(&mut self, s: &str) {
        for line in s.lines() {
            if line.starts_with("#") || line=="\n" {
                continue;
                }

            if let Ok(setting)=Config::split_by_colon(line) {
                let (key, value)=(&setting[0], &setting[1]);
                match &key[..] {
                    "punctuation_characters" => {
                        let mut punctuation_characters: HashSet<char>=['\n', '\r'].iter().cloned().collect();
                        for ch in value.chars() {
                            punctuation_characters.insert(ch);
                            }
                        self.punctuation_characters=punctuation_characters;
                        },
                    _ => {},
                    };
                }
            }
        }
    pub fn generate_alphabets_scheme(&self) -> Vec<u32> {
        let mut preresult: Vec<(u32, u32, usize)>=Vec::new();
        for (id, engine) in self.engines.iter().enumerate() {
            if engine.unicode_ranges.len()!=0 {
                for i in (0..engine.unicode_ranges.len()).step_by(2) {
                    preresult.push((engine.unicode_ranges[i], engine.unicode_ranges[i+1], id));
                    }
                }
            }
        preresult.sort_by_key(|i| i.0);

        let mut result: Vec<u32>=Vec::new();
        for p in preresult {
            result.push(p.0);
            result.push(p.1);
            result.push(p.2 as u32);
            }

        return result;
        }

    fn split_by_colon(line: &str) -> Result<Vec<String>, ()> {
        let mut result: Vec<String>=Vec::new();

        for m in COLON_SEARCHING_REGEX.splitn(line, 2) {
            result.push(m.to_string());
            }

        if result.len()==2 {
            return Ok(result);
            }
        else {
            return Err(());
            }
        }
    }
pub struct SpeechEngineConfiguration {
    pub name: String,
    pub unicode_ranges: Vec<u32>,
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

    pub fn new(name: &str) -> SpeechEngineConfiguration {
        SpeechEngineConfiguration {name: name.to_string(), unicode_ranges: vec![], module: "/usr/lib/speech-dispatcher-modules/sd_espeak-ng".to_string(), arg: "/etc/speech-dispatcher/modules/espeak-ng.conf".to_string(), language: "en".to_string(), voice: "male1".to_string(), punctuation_mode: "some".to_string(), pitch: 10, capitals_pitch: 50, rate: 2, volume: 100, firejailed: false}
        }
    pub fn load_from_string(line: &str) -> Result<SpeechEngineConfiguration, &str> {
        let settings: Vec<String>=line.split(',').map(|i| i.to_string()).collect();

        if settings.len()!=12 {
            return Err("Invalid number of specified options.");
            }

        //Declare individual variables, some with their default values.

        let (name, module, arg, language, voice, punctuation_mode, firejailed);

        let unicode_ranges: Vec<u32>;

        let (mut pitch, mut capitals_pitch, mut rate, mut volume)=(10, 50, 2, 100);

        //Assign declared variables from received settings

        name=settings[0].clone();

        //unicode_ranges
        if &settings[1][..]=="" || &settings[1][..]=="*" {
            unicode_ranges=Vec::new();
            }
        else {
            let mut result: Vec<u32>=Vec::new();

            for range in UNICODE_RANGES_MATCHING_REGEX.find_iter(&settings[1]) {
                let mut errors_found=false;
                let mut values: Vec<u32>=range.as_str().split('-')
                .map(|i| {
                    let i=i[1..].to_lowercase();

                    if let Some(position)=i.find("x") {
                        match u32::from_str_radix(&i[position+1..], 16) {
                            Ok(num) => num,
                            Err(_) => {
                                errors_found=true;
                                0
                                },
                            }
                        }
                    else {

                        match i.parse::<u32>() {
                            Ok(num) => num,
                            Err(_) => {
                                errors_found=true;
                                0
                                },
                            }
                        }
                    })
                .collect();

                if errors_found {
                    return Err("Invalid unicode ranges");
                    }

                if values[1]<values[0] {
                    let temporary=values[0];
                    values[0]=values[1];
                    values[1]=temporary;
                    }

                result.push(values[0]);
                result.push(values[1]);
                }

            if result.len()==0 {
                return Err("No unicode ranges specified");
                }

            unicode_ranges=result;
            };

        module=settings[2].clone();
        arg=settings[3].clone();
        language=settings[4].clone();
        voice=settings[5].clone();
        punctuation_mode=match &settings[6][..] {
            "none" | "some" | "all" => settings[6].clone(),
            _ => "some".to_string(),
            };

        //pitch
        if let Ok(n)=settings[7].parse::<i32>() {
            if n>=-100 && n<=100 {
                pitch=n;
                }
            }
        //capitals_pitch
        if let Ok(n)=settings[8].parse::<i32>() {
            if n>=-100 && n<=100 {
                capitals_pitch=n;
                }
            };
        //rate
        if let Ok(n)=settings[9].parse::<i32>() {
            if n>=-100 && n<=100 {
                rate=n;
                }
            }
        //volume
        if let Ok(n)=settings[10].parse::<i32>() {
            if n>=-100 && n<=100 {
                volume=n;
                }
            }

        firejailed=match &settings[11][..] {
            "yes" | "true" => true,
            _ => false,
            };

        Ok(SpeechEngineConfiguration {name, unicode_ranges, module, arg, language, voice, punctuation_mode, pitch, capitals_pitch, rate, volume, firejailed})
        }
    }

pub fn run(mut config: Config) {
    let mut engines=Vec::new();
    for engine in &config.engines {
        engines.push(Process::new(&engine.module, &engine.arg, engine.firejailed));
        }

    let mut alphabets_scheme=config.generate_alphabets_scheme();
    let mut audio_settings: Option<AudioSettings>=None;
    let mut log_level_settings: Option<LogLevelSettings>=None;

    let mut currently_speaking_engine=0;

    let mut currently_spoken_text: Vec<LanguageChunk>=Vec::new();
    let mut currently_spoken_text_position: usize=0;
    let mut capitalized=false;
    let mut original_pitch=config.engines[0].pitch;
    let mut speaking=false;
    let (sd_input_transmitter, sd_input_receiver)=mpsc::channel::<SdInputCommand>();

    let default_language=config.engines[0].language.clone();
    thread::spawn(move || sd_input_processing_loop(sd_input_transmitter, &default_language));

    let (fs_tx, fs_rx)=mpsc::channel();
    let mut watcher=notify::recommended_watcher(move |res| fs_tx.send(res).unwrap()).unwrap();
    let chinfusor_config_dir=std::env::var("HOME").unwrap()+"/.config/chinfusor";
    watcher.watch(Path::new(&chinfusor_config_dir), RecursiveMode::Recursive).unwrap_or_else(|_| ());

    loop {
        //Catch input from Speech dispatcher
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

        //If nothing is currently being spoken, check, if user didn't change configuration
        if !speaking {
            while let Ok(Ok(event))=fs_rx.try_recv() {
                if event.paths.len()==1 {
                    if let EventKind::Modify(_)=&event.kind {
                        let path=event.paths[0].to_str().unwrap();

                        if path.ends_with("alphabets_settings.csv") {
                            //We need to load again the configuration, recreate list of engines, initialize, set audio, loglevel and properties for each of them.

                            if let Some(audio_settings)=&audio_settings {
                                config.load_alphabets_from_file(path);

                                //First, deinitialize currently running engines;

                                for engine in &mut engines {
                                    engine.write_line("QUIT");
                                    engine.wait_for_exit();
                                    }

                                //Then, start newones

                                engines=Vec::new();
                                for engine in &config.engines {
                                    engines.push(Process::new(&engine.module, &engine.arg, engine.firejailed));
                                    }

                                alphabets_scheme=config.generate_alphabets_scheme();

                                //Now, we need to initialize and configure each of them

                                for (id, engine) in engines.iter_mut().enumerate() {
                                    engine.write_line("INIT");

                                    engine.write(&audio_settings.generate_sd_command());
                                    engine.write(&SpeechSettings::generate_sd_command_from_engine_configuration(&config.engines[id]));

                                    if let Some(log_level_settings)=&log_level_settings {
                                        engine.write(&log_level_settings.generate_sd_command());
                                        }
                                    }

                                }
                            }
                        else if path.ends_with("settings.conf") {
                            config.load_configuration_from_file(path);
                            }
                        }
                    }
                }
            }

        //Check the input from speech-dispatcher
        if let Some(sd_command)=sd_input {

            match sd_command {
                SdInputCommand::Init => {
                    for engine in engines.iter_mut() {
                        engine.write_line("INIT");
                        }
                    },
                SdInputCommand::Audio(settings) => {
                    for (id, engine) in engines.iter_mut().enumerate() {
                        engine.write(&settings.generate_sd_command());
                        engine.write(&SpeechSettings::generate_sd_command_from_engine_configuration(&config.engines[id]));
                        }

                    audio_settings=Some(settings);
                    },
                SdInputCommand::Set(_settings) => {
                    //currently_speaking_engine.write(&settings.generate_sd_command());
                    //another_engine.write(&settings.generate_sd_command());
                    },
                SdInputCommand::LogLevel(settings) => {
                    for engine in engines.iter_mut() {
                        engine.write(&settings.generate_sd_command());
                        }

                    log_level_settings=Some(settings);
                    },
                SdInputCommand::Speak(text) => {
                    if !speaking {

                        currently_spoken_text=text_processor::parse_text(&text.replace("<speak>", "").replace("</speak>", ""), &alphabets_scheme, &config.punctuation_characters, true);

                        if currently_spoken_text.len()>0 {
                            currently_spoken_text_position=0;
                            currently_speaking_engine=currently_spoken_text[0].0;

                            let text=&currently_spoken_text[0].1;

                            engines[currently_speaking_engine].write(&format!("SPEAK\n<speak>{}</speak>\n.\n", text));

                            speaking=true;
                            println!("701 BEGIN");
                            engines[currently_speaking_engine].activate_asynchronous_reading_until_sd_end_signal();
                            }
                        else {
                            println!("701 BEGIN");
                            println!("702 END");
                            }
                        }
                    },
                SdInputCommand::Key(text) => {
                    currently_speaking_engine=0;

                    engines[currently_speaking_engine].write(&format!("KEY\n{}\n.\n", text));

                    speaking=true;

                    println!("701 BEGIN");
                    engines[currently_speaking_engine].activate_asynchronous_reading_until_sd_end_signal();
                    }
                SdInputCommand::Char(ch) => {
                    currently_speaking_engine=text_processor::identify_character(ch, &alphabets_scheme);
                    original_pitch=config.engines[currently_speaking_engine].pitch;
                    let capitalized_pitch=config.engines[currently_speaking_engine].capitals_pitch;

                    if ch.is_uppercase() {
                        engines[currently_speaking_engine].write(&SpeechSettings::generate_sd_command_from_pitch(capitalized_pitch));
                        capitalized=true;
                        }

                    engines[currently_speaking_engine].write(&format!("CHAR\n{}\n.\n", ch));

                    speaking=true;

                    println!("701 BEGIN");
                    engines[currently_speaking_engine].activate_asynchronous_reading_until_sd_end_signal();
                    },
                SdInputCommand::Pause => {
                    if speaking {
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
                        }
                    },
                SdInputCommand::Stop => {
                    if speaking {
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
                        }
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

        //Check whether currently speaking module has finished and update things accordingly.
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

                    currently_speaking_engine=currently_spoken_text[currently_spoken_text_position].0;

                    let text=&currently_spoken_text[currently_spoken_text_position].1;

                    engines[currently_speaking_engine].write(&format!("SPEAK\n<speak>{}</speak>\n.\n", text));
                    engines[currently_speaking_engine].activate_asynchronous_reading_until_sd_end_signal()
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

                println!("202 OK RECEIVING MESSAGE");

                stdin.read_line(&mut key_string).unwrap();
                stdin.read_line(&mut the_rest).unwrap();

                key_string=key_string[..key_string.len()-1].to_string();

                println!("200 OK SPEAKING");

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

