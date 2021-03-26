use sd_chinfusor_ext::{Config, run};

fn main()
    {
    let mut config=Config::new();
    config.load_alphabets_from_file(&(std::env::var("HOME").unwrap()+"/.config/chinfusor/alphabets_settings.csv"));
    config.load_configuration_from_file(&(std::env::var("HOME").unwrap()+"/.config/chinfusor/settings.conf"));
    run(config);
    }

