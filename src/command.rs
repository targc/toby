use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub short_args: Vec<String>,
    pub kv: HashMap<String, String>,
}

pub fn parse_command(input: &str) -> Option<Command> {
    // tolerate leading spaces before /cmd; tolerate CRLF; capture body lines starting with '-'
    let re = Regex::new(
        r"(?m)^\s*/(?P<cmd>[A-Za-z][\w-]*)(?P<shorts>[^\r\n]*)\r?\n(?P<body>(?:[ \t]*-[ \t]+.*(?:\r?\n|$))*)"
    ).unwrap();

    let mut it = re.captures_iter(input);
    let cap = it.next()?;
    if it.next().is_some() {
        return None;
    } // fail if multiple commands present

    let name = cap.name("cmd").unwrap().as_str().to_string();

    let short_args = cap
        .name("shorts")
        .map(|m| {
            m.as_str()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_else(Vec::new);

    let mut kv = HashMap::new();
    if let Some(body) = cap.name("body") {
        for line in body.as_str().lines() {
            let line = line.trim(); // trims both ends, handles \r
            if line.is_empty() {
                continue;
            }
            // accept "- k: v", "-k: v", "-   k :  v"
            let line = line.trim_start_matches('-').trim_start();
            let mut parts = line.splitn(2, ':');
            match (parts.next(), parts.next()) {
                (Some(k), Some(v)) => {
                    let k = k.trim();
                    let v = v.trim();
                    if !k.is_empty() {
                        kv.insert(k.to_string(), v.to_string());
                    }
                }
                _ => {} // ignore malformed lines
            }
        }
    }

    Some(Command {
        name,
        short_args,
        kv,
    })
}
