pub fn to_head_larges<F>(s: &str, splitter: F) -> String
    where F: Fn(&str) -> Vec<String> {
    let mut s_vec: Vec<String> = splitter(s);
    return s_vec.join(" ");
}

pub fn to_head_large(s: &str) -> String {
    if s.is_empty() { return "".to_string(); }
    let mut c_vec = s.chars();
    let head = c_vec.next().unwrap().to_uppercase().to_string();
    let tail = c_vec.as_str().to_lowercase();
    return format!("{}{}", head, tail);
}