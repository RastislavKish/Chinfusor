
#[derive(Debug, PartialEq)]
pub enum LanguageChunk {
    Latin(String),
    Chinese(String),
    Cyrillic(String),
    }

#[derive(PartialEq)]
pub enum Alphabet {
    Latin,
    Chinese,
    Cyrillic,
    }

pub fn parse_text(text: &str, ssml: bool) -> Vec<LanguageChunk>
    {
    let chars: Vec<char>=text.chars().collect();
    if chars.len()==0 {
        return Vec::new();
        }
    let mut result: Vec<LanguageChunk>=Vec::new();

    let mut last_mark=0;
    let mut current_chunk_type=match chars[0] as u32 {
        0x4E00..=0x9FA5 => Alphabet::Chinese,
        0x400..=0x52F => Alphabet::Cyrillic,
        _ => Alphabet::Latin,
        };
    let mut in_tag=false;

    for (i, ch) in chars.iter().enumerate() {
        if ssml {
            if *ch=='<' && !in_tag {
                in_tag=true;
                }
            if *ch=='>' && in_tag {
                in_tag=false;
                }

            if in_tag {
                continue;
                }
            }

        match ch {
            ',' | '.' | '?' | '，' | '。' | '？' | '-' | ' ' | ':' | '\r' | '\n' => continue,
            _ => {},
            };

        let new_chunk_type=match *ch as u32 {
            0x4E00..=0x9FA5 => Alphabet::Chinese,
            0x400..=0x52F => Alphabet::Cyrillic,
            _ => Alphabet::Latin,
            };

        if new_chunk_type!=current_chunk_type {
            let collected_text=(&chars[last_mark..i]).iter().collect::<String>().trim().to_string();
            result.push(match current_chunk_type {
                Alphabet::Latin => LanguageChunk::Latin(collected_text),
                Alphabet::Chinese => LanguageChunk::Chinese(collected_text),
                Alphabet::Cyrillic => LanguageChunk::Cyrillic(collected_text),
                });

            current_chunk_type=new_chunk_type;
            last_mark=i;
            }

        }

    //Process the last chunk

    let collected_text=(&chars[last_mark..]).iter().collect::<String>().trim().to_string();
    result.push(match current_chunk_type {
        Alphabet::Latin => LanguageChunk::Latin(collected_text),
        Alphabet::Chinese => LanguageChunk::Chinese(collected_text),
        Alphabet::Cyrillic => LanguageChunk::Cyrillic(collected_text),
        });

    result
    }
pub fn identify_character(character: char) -> Alphabet {
    match character as u32 {
        0x4E00..=0x9FA5 => Alphabet::Chinese,
        0x400..=0x52F => Alphabet::Cyrillic,
        _ => Alphabet::Latin,
        }
    }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn chinese_test() {
        let text="Hello, this is an experimental test. 你好，我是人。 There is also some mixed 过 text. Hmm, I must refresh my Chinese, as I don't remember a word. :D Here is also a test question: 他又妈妈？";
        let expected_result: Vec<LanguageChunk>=vec![LanguageChunk::Latin("Hello, this is an experimental test.".to_string()), LanguageChunk::Chinese("你好，我是人。".to_string()), LanguageChunk::Latin("There is also some mixed".to_string()), LanguageChunk::Chinese("过".to_string()), LanguageChunk::Latin("text. Hmm, I must refresh my Chinese, as I don't remember a word. :D Here is also a test question:".to_string()), LanguageChunk::Chinese("他又妈妈？".to_string())];

        assert_eq!(expected_result, parse_text(text, false));
        }

    #[test]
    fn cyrillic_test() {
        let text="This is a Katyusha test. It consists of writing Катюша and seeing, whether this thing can parse it, as I don't have a cyrillic keyboard.";
        let expected_result: Vec<LanguageChunk>=vec![LanguageChunk::Latin("This is a Katyusha test. It consists of writing".to_string()), LanguageChunk::Cyrillic("Катюша".to_string()), LanguageChunk::Latin("and seeing, whether this thing can parse it, as I don't have a cyrillic keyboard.".to_string())];

        assert_eq!(expected_result, parse_text(text, false));
        }
    }
