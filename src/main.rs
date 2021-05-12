use regex::regex::Regex;

fn main() {
    let regexp = Regex::new("(p(erl|ython|hp)|ruby)".to_string()).unwrap();
    if regexp.matches("phps".to_string()) {
        println!("match");
    } else {
        println!("not match");
    }
}
