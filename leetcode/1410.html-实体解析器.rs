/*
 * @lc app=leetcode.cn id=1410 lang=rust
 *
 * [1410] HTML 实体解析器
 */

// @lc code=start
impl Solution {
    pub fn entity_parser(text: String) -> String {
        use regex::Regex;
        use std::collections::HashMap;

        let m: HashMap<&str, &str> = [
            ("quot", "\""),
            ("apos", "'"),
            ("amp", "&"),
            ("gt", ">"),
            ("lt", "<"),
            ("frasl", "/"),
        ]
        .into_iter()
        .collect();

        let re = Regex::new(r#"&((quot)|(apos)|(amp)|(gt)|(lt)|(frasl));"#).unwrap();
        let replaced_text = re.replace_all(&text, |caps: &regex::Captures| {
            let group = caps.get(1).unwrap().as_str();
            if let Some(replacement) = m.get(group) {
                replacement.to_string()
            } else {
                caps.get(0).unwrap().as_str().to_string()
            }
        });

        replaced_text.to_string()
    }
}
// @lc code=end
