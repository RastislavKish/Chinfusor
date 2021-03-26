use chinfusor::{Config, run};

fn main()
    {
    let mut config=Config::new();
    config.load_from_file(&(std::env::var("HOME").unwrap()+"/.config/chinfusor/settings.csv"));
    run(config);
    }

