use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sd_chinfusor::{Config, text_processor::*};

fn purely_latin_benchmark(c: &mut Criterion) {
    let mut config=Config::new();
    config.load_alphabets_from_string("latin,*,/usr/lib/speech-dispatcher-modules/sd_espeak-ng,/etc/speech-dispatcher/modules/espeak-ng.conf,sk,male1,some,10,50,2,100,no\nchinese,u0x4E00-u0x9FA5,/usr/lib/speech-dispatcher-modules/sd_espeak-ng,/etc/speech-dispatcher/modules/espeak-ng.conf,cmn,male1,some,10,50,2,100,no\ncyrillic,u0x400-u0x52F,/usr/lib/speech-dispatcher-modules/sd_espeak-ng,/etc/speech-dispatcher/modules/espeak-ng.conf,ru,male1,some,10,50,2,100,no");
    config.load_configuration_from_string("punctuation_characters: ,.?，。？- :()");

    let alphabets_scheme=config.generate_alphabets_scheme();
    let punctuation_characters=&config.punctuation_characters;

    //Prepare a text consisting purely of latin characters.

    let mut text=String::new();
    for _ in 0..100000 {
        text+="aaaaaaaaaa";
        }

    c.bench_function("pure latin", |b| b.iter(|| parse_text(black_box(&text), black_box(&alphabets_scheme), black_box(punctuation_characters), black_box(true))));

    }
fn purely_chinese_benchmark(c: &mut Criterion) {
    let mut config=Config::new();
    config.load_alphabets_from_string("latin,*,/usr/lib/speech-dispatcher-modules/sd_espeak-ng,/etc/speech-dispatcher/modules/espeak-ng.conf,sk,male1,some,10,50,2,100,no\nchinese,u0x4E00-u0x9FA5,/usr/lib/speech-dispatcher-modules/sd_espeak-ng,/etc/speech-dispatcher/modules/espeak-ng.conf,cmn,male1,some,10,50,2,100,no\ncyrillic,u0x400-u0x52F,/usr/lib/speech-dispatcher-modules/sd_espeak-ng,/etc/speech-dispatcher/modules/espeak-ng.conf,ru,male1,some,10,50,2,100,no");
    config.load_configuration_from_string("punctuation_characters: ,.?，。？- :()");

    let alphabets_scheme=config.generate_alphabets_scheme();
    let punctuation_characters=&config.punctuation_characters;

    //Prepare a text consisting purely of latin characters.

    let mut text=String::new();
    for _ in 0..100000 {
        text+="你好我是人嘛你又差嘛";
        }

    c.bench_function("pure chinese", |b| b.iter(|| parse_text(black_box(&text), black_box(&alphabets_scheme), black_box(punctuation_characters), black_box(true))));

    }
fn mixed_latin_and_chinese_benchmark(c: &mut Criterion) {
    let mut config=Config::new();
    config.load_alphabets_from_string("latin,*,/usr/lib/speech-dispatcher-modules/sd_espeak-ng,/etc/speech-dispatcher/modules/espeak-ng.conf,sk,male1,some,10,50,2,100,no\nchinese,u0x4E00-u0x9FA5,/usr/lib/speech-dispatcher-modules/sd_espeak-ng,/etc/speech-dispatcher/modules/espeak-ng.conf,cmn,male1,some,10,50,2,100,no\ncyrillic,u0x400-u0x52F,/usr/lib/speech-dispatcher-modules/sd_espeak-ng,/etc/speech-dispatcher/modules/espeak-ng.conf,ru,male1,some,10,50,2,100,no");
    config.load_configuration_from_string("punctuation_characters: ,.?，。？- :()");

    let alphabets_scheme=config.generate_alphabets_scheme();
    let punctuation_characters=&config.punctuation_characters;

    //Prepare a text consisting purely of latin characters.

    let mut text=String::new();
    for _ in 0..50000 {
        text+="hellothere你好我是人嘛你又差嘛";
        }

    c.bench_function("mixed latin chinese", |b| b.iter(|| parse_text(black_box(&text), black_box(&alphabets_scheme), black_box(punctuation_characters), black_box(true))));

    }

criterion_group!(benches, purely_latin_benchmark, purely_chinese_benchmark, mixed_latin_and_chinese_benchmark);
criterion_main!(benches);

