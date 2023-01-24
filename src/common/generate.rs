use std::fs;

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn format_string(input: String) -> String {
    let input = input.replace("_", " ");
    let input = input.split(".").next().unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();
    let input: Vec<String> = input.iter().map(|s| capitalize(s)).collect();
    let input = input.join(" ");

    input
}

pub fn name_vector(path: &str, blacklist: &Vec<&str>) -> Vec<String> {
    let mut items = vec![];
    for entry in fs::read_dir(path).unwrap() {
        let file_name = entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .into_owned()
            .trim()
            .to_string();

        if blacklist.contains(&file_name.as_ref()) {
            continue;
        }
        items.push(format_string(file_name));
    }
    items.reverse();
    items
}

pub fn input() -> String {
    let mut input = String::new();
    let result = match std::io::stdin().read_line(&mut input) {
        Ok(_) => input
            .to_string()
            .trim()
            .to_string(),
        Err(error) => {
            if error.kind() == std::io::ErrorKind::UnexpectedEof {
                std::process::exit(0);
            } else {
                panic!("Failed to read your input! D: ({})", error);
            }
        }
    };
        
    result
}