use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

static KV_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"("(?:[^"\\]|\\.)*")\s*("(?:[^"\\]|\\.)*")(?:\s*//\s*(.*))?"#).unwrap()
});

pub fn replace_kv_pairs(original: &str, replacements: &HashMap<String, String>) -> String {
    if replacements.is_empty() {
        return original.to_string();
    }
    let mut result = String::with_capacity(original.len());
    for line in original.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with("//") {
            if let Some(caps) = KV_PATTERN.captures(trimmed) {
                let key = &caps[1];
                let unescaped_key = key[1..key.len() - 1].replace("\\\"", "\"");
                if let Some(new_value) = replacements.get(&unescaped_key) {
                    let escaped_value = new_value.replace('"', "\\\"");
                    let indent_len = line.len() - line.trim_start().len();
                    let indent = &line[..indent_len];
                    result.push_str(indent);
                    result.push('"');
                    result.push_str(key);
                    result.push_str("\" \"");
                    result.push_str(&escaped_value);
                    result.push('"');
                    result.push('\n');
                    continue;
                }
            }
        }
        result.push_str(line);
        result.push('\n');
    }
    result
}
