# String

### Str and &str
1.

```rust,editable
fn main() {
    let s: &str = "hello, world"; // it is &str not str

    println!("Success!");
}
```


2. 

```rust,editable
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(&s);
}

fn greetings(s: &str) {
    println!("{}",s)
}
```

### String

3.
```rust,editable

fn main() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}
```

4.
```rust,editable
fn main() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}
```

5.
```rust,editable
fn main() {
    let s = String::from("I like dogs");
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}
```

6.

```rust,editable
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2; 
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}
```

### &str and String

7.
```rust,editable
// 1)
fn main() {
    let s = String::from("hello, world");
    greetings(s);
}

fn greetings(s: String) {
    println!("{}",s)
}
```
```rust,editable
// 2)
fn main() {
    let s = "hello, world".to_string();
    greetings(s);
}

fn greetings(s: String) {
    println!("{}",s)
}
```
8.

```rust,editable
fn main() {
    let s = "hello, world".to_string();
    let s1: &str = &s[..];

    println!("Success!");
}
```

### String escapes
9. 
```rust,editable
fn main() {
    let byte_escape = "I'm writing Ru\x73__!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
                unicode_codepoint, character_name );

   let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}
```

10.

```rust,editable
fn main() {
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    let  delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}
```

### String index
11.

```rust,editable
fn main() {
    let s1 = String::from("hi,中国");
    let h = s1[0..1]; // h is only one byte 
    assert_eq!(h, "h");

    let h1 = &s1[3..6]; // the symbol '中' is 2 bytes long
    assert_eq!(h1, "中");

    println!("Success!");
}
```

### Operate on UTF8 string
12. 🌟
```rust,editable
fn main() {
    for c in "你好，世界".chars() {
// chars() creates iterator of chars
        println!("{}", c)
    }
}
```
