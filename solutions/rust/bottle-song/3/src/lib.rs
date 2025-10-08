use std::collections::HashMap;

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut words = HashMap::new();
    words.insert(0, "no".to_string());
    words.insert(1, "one".to_string());
    words.insert(2, "two".to_string());
    words.insert(3, "three".to_string());
    words.insert(4, "four".to_string());
    words.insert(5, "five".to_string());
    words.insert(6, "six".to_string());
    words.insert(7, "seven".to_string());
    words.insert(8, "eight".to_string());
    words.insert(9, "nine".to_string());
    words.insert(10, "ten".to_string());

    fn bottle_word(n: u32) -> &'static str {
        if n == 1 { "bottle" } else { "bottles" }
    }

    fn capitalize(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
    fn num_word(n: u32, dict: &HashMap<u32, String>) -> String {
        dict.get(&n).cloned().unwrap_or_else(|| n.to_string())
    }

    let mut song = String::new();


    for n in (start_bottles - take_down + 1..=start_bottles).rev() {
        let next = n.saturating_sub(1);

        let current_word = num_word(n, &words);     // String
        let next_word = num_word(next, &words);     // String

        let current_bottle = bottle_word(n);        // &str
        let next_bottle = bottle_word(next);        // &str

        let verse = format!(
            "{cw} green {cb} hanging on the wall,\n\
             {cw} green {cb} hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be {nw} green {nb} hanging on the wall.\n",
            cw = capitalize(&current_word),
            cb = current_bottle,
            nw = next_word,
            nb = next_bottle
        );

        song.push_str(&verse);

        if n != start_bottles - take_down + 1 {
            song.push('\n'); 
        }
    }

    song
}