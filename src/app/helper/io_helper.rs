pub fn confirm(msg: &str) -> bool {
    let mut ans = String::new();
    println!("{} (Y/n):  ", msg);
    if std::io::stdin().read_line(&mut ans).is_err() {
        println!("cannot read the line");
        return false;
    }
    println!("{}", ans);
    // ans has newline character
    if ans.trim_end().to_lowercase() == "y".to_string() {
        return true;
    }
    return false;
}
