use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct LanguageChunk(pub usize, pub String);

//Alphabets cheme is a vec consisting of triplets of u32s, where first value is unicode range start, the second unicode range end, and the thirdone id of the alphabet for which the range is defined. These triplets are sorted in ascending order with the range start as sorting key, so it is easy to search through them.

pub fn parse_text(text: &str, alphabets_scheme: &Vec<u32>, punctuation_characters: &HashSet<char>, ssml: bool) -> Vec<LanguageChunk> {
    if alphabets_scheme.len()==0 {
        return vec![LanguageChunk(0, text.to_string())];
        }

    let chars: Vec<char>=text.chars().collect();
    if chars.len()==0 {
        return Vec::new();
        }
    let mut result: Vec<LanguageChunk>=Vec::new();

    let lower_threshold=alphabets_scheme[0];
    let upper_threshold=alphabets_scheme[alphabets_scheme.len()-2];
    let get_alphabet_id=|ch| {
        if ch<lower_threshold || ch>upper_threshold {
            return 0;
            }

        if ch<=alphabets_scheme[1] {
            return alphabets_scheme[2] as usize;
            }

        for i in (3..alphabets_scheme.len()).step_by(3) {
            let (lower_boundary, upper_boundary)=(alphabets_scheme[i], alphabets_scheme[i+1]);

            if ch<lower_boundary {
                return 0;
                }
            if ch<=upper_boundary {
                return alphabets_scheme[i+2] as usize;
                }
            }

        return 0;
        };

    let mut last_mark=0;
    let mut chunk_type_set=false;
    let mut current_chunk_type=if ssml && chars[0]=='<' {
        0
        } else {
        chunk_type_set=true;
        get_alphabet_id(chars[0] as u32)
        };
    let mut in_tag=false;

    for (i, ch) in chars.iter().enumerate() {
        if ssml {
            if *ch=='<' && !in_tag {
                in_tag=true;
                }
            if *ch=='>' && in_tag {
                in_tag=false;
                continue;
                }

            if in_tag {
                continue;
                }
            }

        if punctuation_characters.contains(ch) {
            continue;
            }

        if !chunk_type_set {
            current_chunk_type=get_alphabet_id(*ch as u32);
            chunk_type_set=true;
            continue;
            }

        let new_chunk_type=get_alphabet_id(*ch as u32);

        if new_chunk_type!=current_chunk_type {
            let collected_text=(&chars[last_mark..i]).iter().collect::<String>().trim().to_string();
            result.push(LanguageChunk(current_chunk_type, collected_text));

            current_chunk_type=new_chunk_type;
            last_mark=i;
            }

        }

    //Process the last chunk

    let collected_text=(&chars[last_mark..]).iter().collect::<String>().trim().to_string();
    result.push(LanguageChunk(current_chunk_type, collected_text));

    result
    }
pub fn identify_character(character: char, alphabets_scheme: &Vec<u32>) -> usize {
    if alphabets_scheme.len()==0 {
        return 0;
        }

    let ch=character as u32;

    if ch<alphabets_scheme[0] || ch>alphabets_scheme[alphabets_scheme.len()-2] {
        return 0;
        }
    if ch<=alphabets_scheme[1] {
        return alphabets_scheme[2] as usize;
        }

    for i in (3..alphabets_scheme.len()).step_by(3) {
        let (lower_boundary, upper_boundary)=(alphabets_scheme[i], alphabets_scheme[i+1]);

        if ch<lower_boundary {
            return 0;
            }
        if ch<=upper_boundary {
            return alphabets_scheme[i+2] as usize;
            }
        }

    return 0;
    }

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::Config;

    #[test]
    fn chinese_test() {
        let mut config=Config::new();
        config.load_alphabets_from_string("latin,*,/usr/lib/speech-dispatcher-modules/sd_espeak-ng,/etc/speech-dispatcher/modules/espeak-ng.conf,sk,male1,some,10,50,2,100,no\nchinese,u0x4E00-u0x9FA5,/usr/lib/speech-dispatcher-modules/sd_espeak-ng,/etc/speech-dispatcher/modules/espeak-ng.conf,cmn,male1,some,10,50,2,100,no\ncyrillic,u0x400-u0x52F,/usr/lib/speech-dispatcher-modules/sd_espeak-ng,/etc/speech-dispatcher/modules/espeak-ng.conf,ru,male1,some,10,50,2,100,no");
        config.load_configuration_from_string("punctuation_characters: ,.?，。？- :()");

        let text="Hello, this is an experimental test. 你好，我是人。 There is also some mixed 过 text. Hmm, I must refresh my Chinese, as I don't remember a word. :D Here is also a test question: 他又妈妈？";
        let expected_result: Vec<LanguageChunk>=vec![LanguageChunk(0, "Hello, this is an experimental test.".to_string()), LanguageChunk(1, "你好，我是人。".to_string()), LanguageChunk(0, "There is also some mixed".to_string()), LanguageChunk(1, "过".to_string()), LanguageChunk(0, "text. Hmm, I must refresh my Chinese, as I don't remember a word. :D Here is also a test question:".to_string()), LanguageChunk(1, "他又妈妈？".to_string())];

        assert_eq!(expected_result, parse_text(text, &config.generate_alphabets_scheme(), &config.punctuation_characters, false));
        }

    #[test]
    fn cyrillic_test() {
        let mut config=Config::new();
        config.load_alphabets_from_string("latin,*,/usr/lib/speech-dispatcher-modules/sd_espeak-ng,/etc/speech-dispatcher/modules/espeak-ng.conf,sk,male1,some,10,50,2,100,no\nchinese,u0x4E00-u0x9FA5,/usr/lib/speech-dispatcher-modules/sd_espeak-ng,/etc/speech-dispatcher/modules/espeak-ng.conf,cmn,male1,some,10,50,2,100,no\ncyrillic,u0x400-u0x52F,/usr/lib/speech-dispatcher-modules/sd_espeak-ng,/etc/speech-dispatcher/modules/espeak-ng.conf,ru,male1,some,10,50,2,100,no");
        config.load_configuration_from_string("punctuation_characters: ,.?，。？- :()");

        let text="This is a Katyusha test. It consists of writing Катюша and seeing, whether this thing can parse it, as I don't have a cyrillic keyboard.";
        let expected_result: Vec<LanguageChunk>=vec![LanguageChunk(0, "This is a Katyusha test. It consists of writing".to_string()), LanguageChunk(2, "Катюша".to_string()), LanguageChunk(0, "and seeing, whether this thing can parse it, as I don't have a cyrillic keyboard.".to_string())];

        assert_eq!(expected_result, parse_text(text, &config.generate_alphabets_scheme(), &config.punctuation_characters, false));
        }

    }
