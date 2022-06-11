pub fn error(line: usize, message: &str) {
    report(line, message, "");
}

pub fn report(line: usize, message: &str, location: &str) {
    eprintln!("[line {}], Error {}: {}", line, location, message);
}
