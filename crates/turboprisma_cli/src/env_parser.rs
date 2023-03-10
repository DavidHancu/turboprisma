use lazy_static::__Deref;
use regex::{RegexBuilder, Regex};
use std::collections::HashMap;

pub fn parse_env(content: &str) -> Result<HashMap<String, String>, Option<String>> {
    let line = RegexBuilder::new(r#"(?:^|^)\s*(?:export\s+)?([\w.-]+)(?:\s*=\s*?|:\s+?)(\s*'(?:\\'|[^'])*'|\s*"(?:\\"|[^"])*"|\s*`(?:\\`|[^`])*`|[^#\r\n]+)?\s*(?:#.*)?(?:$|$)"#).multi_line(true).build().unwrap();
    let new_line = RegexBuilder::new(r"\r\n?").multi_line(true).build().unwrap();
    let quote_replace = RegexBuilder::new(r#"^(?:(')([\s\S]*)')|(?:(")([\s\S]*)")|(?:(`)([\s\S]*)`)$"#).multi_line(true).build().unwrap();
    let new_line_replace = Regex::new(r#"\\n"#).unwrap();
    let carriage_replace = Regex::new(r#"\\r"#).unwrap();
    let expand_regex = Regex::new(r#"(\\*?)\$\{?([\w]+)(?::-([^}\\]*))?\}?"#).unwrap();

    let mut map = HashMap::new();
    let content = new_line.replace_all(content, "\n");

    for capture in line.captures_iter(&content) {
        let key = &capture[1];
        let value = capture[2].trim();

        let maybe_quote = value.starts_with("\"");

        match maybe_quote {
            true => {
                let quote_replaced = quote_replace.replace_all(&value, "$2$4$6").to_string();
                let new_line_replaced = new_line_replace.replace_all(&quote_replaced, "\n");
                let carriage_replaced = carriage_replace.replace_all(&new_line_replaced, "\n");

                map.insert(key.to_owned(), carriage_replaced.to_string());
            },
            false => {
                map.insert(key.to_owned(), quote_replace.replace_all(&value, "$2$4$6").to_string());
            },
        };
    }

    let mut cached: HashMap<String, String> = HashMap::new();
    for (key, value) in &map {
        let mut new_value = value.deref().to_string();
        for capture in expand_regex.captures_iter(&value) {
            let escaped = &capture[1];
            let replaced = &capture[2];

            if escaped.chars().count() % 2 == 1 {
                continue;
            }
            
            if map.contains_key(replaced)
            {
                new_value = new_value.replace(&capture[0], &map[replaced]);
            }
        }
        cached.insert(key.to_string(), new_value);
    }

    for (key, value) in cached {
        map.insert(key, value);
    }

    Ok(map)
}
