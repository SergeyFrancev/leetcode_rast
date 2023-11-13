pub struct Solution {

}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut out: String = String::from("");
        let mut stack = String::from("");

        fn save_part(text: &mut String, part: String) {
            if part.len() > 0 {
                text.push(' ');
                text.push_str(&part);
            }
        }

        for ch in s.chars().into_iter() {
            match ch {
                ' ' => {
                    save_part(&mut out, stack);
                    stack = String::from("");
                },
                _ => {stack.insert(0, ch)},
            }
        }
        save_part(&mut out, stack);
        out
    }
}
