pub fn load_input(base: &str, path: &str) -> String {
    let path = std::path::Path::new(base).join(path);

    println!("path: {}", path.to_str().unwrap());
    std::fs::read_to_string(path).unwrap()
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
