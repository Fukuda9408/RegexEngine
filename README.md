# Rustで正規表エンジン
[正規表現を作ろう](https://codezine.jp/article/corner/237)でPythonで実装されている正規表現エンジンをRustで実装。

### 使いかた
```Rust
use regex::regex::Regex;

let regexp = Regex::new("(p(erl|ython|hp)|ruby)".to_string()).unwrap();
if regexp.matches("python".to_string()) {
    println!("match");
} else {
    println!("not match");
}
// match
```
