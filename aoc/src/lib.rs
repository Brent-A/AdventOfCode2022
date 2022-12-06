pub fn load_input(base: &str, path: &str) -> String {
    let path = std::path::Path::new(base).join(path);

    std::fs::read_to_string(path).unwrap()
}
