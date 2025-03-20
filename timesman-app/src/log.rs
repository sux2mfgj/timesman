use chrono;

pub fn tmlog(text: String) {
    println!("{}: {}", chrono::Local::now().to_string(), text);
}
