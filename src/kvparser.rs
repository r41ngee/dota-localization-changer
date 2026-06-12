use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

static KV_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"("(?:[^"\\]|\\.)*")\s*("(?:[^"\\]|\\.)*")(?:\s*//\s*(.*))?"#).unwrap()
});

const HEADER_LINES: usize = 5;
const FOOTER_LINES: usize = 3;

pub fn parse(text: &str) -> HashMap<String, String> {
    let mut data = HashMap::new();
    let lines: Vec<&str> = text.lines().collect();
    let total_lines = lines.len();

    for (i, line) in lines.iter().enumerate() {
        if i < HEADER_LINES || i >= total_lines - FOOTER_LINES {
            continue;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        if let Some(caps) = KV_PATTERN.captures(trimmed) {
            let key = &caps[1];
            let value = &caps[2];
            let unescaped_key = key[1..key.len() - 1].replace("\\\"", "\"");
            let unescaped_value = value[1..value.len() - 1].replace("\\\"", "\"");
            data.insert(unescaped_key, unescaped_value);
        }
    }

    data
}

pub fn unparse(data: &HashMap<String, String>, lang: &str) -> String {
    let mut lines = Vec::new();
    lines.push("\"lang\"".to_string());
    lines.push("{".to_string());
    lines.push(format!("\t\"Language\" \"{}\"", lang));
    lines.push("\t\"Tokens\"".to_string());
    lines.push("\t{".to_string());
    for (key, value) in data {
        let escaped_key = key.replace('"', "\\\"");
        let escaped_value = value.replace('"', "\\\"");
        lines.push(format!("\t\t\"{}\" \"{}\"", escaped_key, escaped_value));
    }
    lines.push("\t}".to_string());
    lines.push("}".to_string());
    lines.join("\n")
}
