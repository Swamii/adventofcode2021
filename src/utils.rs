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
