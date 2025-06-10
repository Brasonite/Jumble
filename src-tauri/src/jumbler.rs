use crate::processor::{self, Segment};

#[tauri::command]
pub fn jumble(input: &str) -> String {
    let mut processed: String = String::new();

    let (_, segments) = processor::parse(input).expect("Failed to process input");

    for segment in segments {
        match segment {
            Segment::Fixed(text) => processed.push_str(&text),
            Segment::Flexible(text) => {
                let mut processed_word = text.clone();

                let chars: Vec<char> = text.to_lowercase().chars().collect();

                if chars.len() > 3 {
                    // Word can be jumbled
                    let uppercase_chars: Vec<usize> = text
                        .char_indices()
                        .filter(|(_, c)| c.is_uppercase())
                        .map(|(idx, _)| idx)
                        .collect();

                    let mut attempts = 5usize;
                    while processed_word == text && attempts > 0 {
                        processed_word.clear();

                        let mut available_chars = chars.clone();

                        let first = available_chars.remove(0);
                        let last = available_chars.pop().unwrap();

                        fastrand::shuffle(&mut available_chars);

                        processed_word.push(first);
                        for c in available_chars {
                            processed_word.push(c);
                        }
                        processed_word.push(last);

                        let char_indices: Vec<(usize, char)> =
                            processed_word.char_indices().collect();
                        for (idx, char) in char_indices {
                            if uppercase_chars.contains(&idx) {
                                let uppercase: String = char.to_uppercase().collect();
                                processed_word.remove(idx);
                                processed_word.insert_str(idx, &uppercase);
                            }
                        }

                        attempts -= 1;
                    }
                }

                processed.push_str(&processed_word);
            }
        }
    }

    processed
}
