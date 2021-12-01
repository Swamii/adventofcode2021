const INPUT_DIR: &str = "input/";

pub fn read_input(name: &str) -> String {
    let mut path: String = INPUT_DIR.to_owned();
    path.push_str(name);
    return std::fs::read_to_string(path).expect("File not found");
}
