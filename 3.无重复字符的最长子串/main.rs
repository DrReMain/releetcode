impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;
        let chars: Vec<char> = s.chars().collect();
        let mut m = HashSet::new();
        let mut prev = 0;
        let mut idx = 0;
        let mut len = 0;

        chars.iter().enumerate().for_each(|(_, ch)| {
            while m.contains(ch) {
                m.remove(&chars[prev]);
                prev += 1;
            }

            m.insert(ch);
            len = len.max(idx - prev + 1);
            idx += 1;
        });

        len as i32
    }
}
