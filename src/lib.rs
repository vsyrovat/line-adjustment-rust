pub fn transform(input: &str, line_width: u32) -> String {
    let line_width = line_width as usize;
    let words = input.split(" ").filter(|w| !w.is_empty());
    let mut lines = vec![];
    let mut line_candidate_words = vec![];
    let mut line_chars_count = 0;
    let mut is_first_word = true;
    for word in words {
        if line_chars_count + 1 + word.chars().count() > line_width {
            lines.push(align_line(line_candidate_words, line_width));
            line_candidate_words = vec![];
            line_candidate_words.push(word);
            line_chars_count = word.chars().count();
            is_first_word = false;
        } else {
            line_candidate_words.push(word);
            line_chars_count += word.chars().count();
            if !is_first_word {
                line_chars_count += 1;
                is_first_word = false;
            }
        }
    }
    lines.push(align_line(line_candidate_words, line_width));
    lines.join("\n")
}

fn align_line(words: Vec<&str>, line_width: usize) -> String {
    match words.len() {
        0 => "".to_string(),
        1 => format!(
            "{}{}",
            words[0],
            " ".repeat(line_width - words[0].chars().count())
        ),
        _ => {
            let chars_count = words.iter().fold(0, |acc, w| acc + w.chars().count());
            if chars_count >= line_width {
                return words.join(" ");
            }
            let gaps_count = words.len() - 1;
            let spaces_count = line_width - chars_count;
            let (gap_size, extra_gaps) = (spaces_count / gaps_count, spaces_count % gaps_count);
            let mut segments: Vec<String> = vec![];
            for i in 0..words.len() {
                segments.push(words[i].to_string());
                if i < words.len() - 1 {
                    let gap = " ".repeat(if i < extra_gaps {
                        gap_size + 1
                    } else {
                        gap_size
                    });
                    segments.push(gap);
                }
            }
            segments.join("")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::align_line;
    use super::transform;

    #[test]
    fn test_align_line() {
        #[rustfmt::skip]
        let test_cases = [
            (vec![], 5, ""),
            (vec!["test"], 5, "test "),
            (vec!["dolor", "sit"], 12, "dolor    sit"),
            (vec!["6chars", "7charss"], 12, "6chars 7charss"),
            (vec!["3", "words", "yet"], 19, "3     words     yet"),
            (vec!["3", "words", "yet"], 20, "3      words     yet"),
            (vec!["Bảo", "tàng", "Lâm", "Đồng"], 18, "Bảo  tàng Lâm Đồng"),
        ];

        for (words, line_width, expected) in test_cases {
            let output = align_line(words, line_width);
            assert_eq!(output, expected);
        }
    }

    #[test]
    fn test_transform() {
        #[rustfmt::skip]
        let test_cases = [
            ("", 5, ""),
            ("test", 5, "test "),
            ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
             "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      "),
            ("Lorem     ipsum    dolor", 17, "Lorem ipsum dolor"),
            ("Мама мыла раму, раму мыла мама.", 7, "Мама   \nмыла   \nраму,  \nраму   \nмыла   \nмама.  "),
            ("Bảo tàng Lâm Đồng Minh Nguyệt Cư Sĩ Lâm", 10, "Bảo   tàng\nLâm   Đồng\nMinh      \nNguyệt  Cư\nSĩ     Lâm")
        ];

        for &(input, line_width, expected) in &test_cases {
            println!("input: '{}'", input);
            let output = transform(input, line_width);
            println!("output: '{}'", output);
            assert_eq!(output, expected);
        }
    }
}
