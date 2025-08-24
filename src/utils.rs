fn read_file(file_path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(file_path)
}

fn write_file(file_path: &str, contents: &str) -> std::io::Result<()> {
    std::fs::write(file_path, contents)
}

fn validate_input(input: &str) -> bool {
    !input.trim().is_empty()
}