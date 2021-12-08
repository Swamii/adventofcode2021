const INPUT_DIR: &str = "input/";

pub fn read_input(name: &str) -> String {
    let path: &String = &format!("{}{}", INPUT_DIR, name);
    return std::fs::read_to_string(path).expect(format!("File {} not found", path).as_str());
}

pub fn capitalize_string(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn sort_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    return chars.iter().collect::<String>();
}

pub fn characters_overlap(s1: &str, s2: &str) -> bool {
    for char in s1.chars() {
        if !s2.contains(char) {
            return false;
        }
    }
    return true;
}
