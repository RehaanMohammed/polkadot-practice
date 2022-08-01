1.
```rust,edtiable
fn main() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    assert_eq!(format!("{1}{0}", 1, 2), "21");
    assert_eq!(format!("{1}{0}{0}{1}", 1, 2), "2112");
    println!("Success!");
}
```
2.
Named argument must be placed after other arguments
```rust,editable
fn main() {
    println!("{argument}", argument = "test");

    assert_eq!(format!("{name}{}", 1, name = 2), "21");
    assert_eq!(format!("{a} {c} {b}",a = "a", b = 'b', c = 3 ), "a 3 b");

    println!("{abc} {0}", 2, abc = "def");

    println!("Success!")
}
```

3.
```rust,editable
fn main() {
    println!("Hello {:5}!", "x"); 
    println!("Hello {:1$}!", "x", 5);

    assert_eq!(format!("Hello {1:0$}!", 5, "x"), "Hello x    !");
    assert_eq!(format!("Hello {:width$}!", "x", width = 5), "Hello x    !");

    println!("Success!")
}
```

4.
```rust,editable
fn main() {
    println!("Hello {:<5}!", "x");
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");

    assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&&!");

    println!("Success!")
}
```

5.
```rust,editable
fn main() {
    println!("Hello {:5}!", 5);
    println!("Hello {:+}!", 5);
    println!("Hello {:05}!", 5);
    println!("Hello {:05}!", -5);

    assert!(format!("{number:0>width$}", number=1, width=6) == "000001");
    
    println!("Success!")
}
```

6.
```rust,editable

fn main() {
    let v = 3.1415926;

    println!("{:.1$}", v, 4); 

    assert_eq!(format!(":.2", v), "3.14");
    assert_eq!(format!(":+2", v), "+3.14");
    assert_eq!(format!(":.0", v), "3");

    println!("Success!")
}
```

7.
```rust,editable
fn main() {
    let s = "Hello, world!";

    println!("{0:.5}", s);

    assert_eq!(format!("Hello {1:.0$}!", 3, "abcdefg"), "Hello abc!");

    println!("Success!")
}
```   

8.
```rust,editable
fn main() {
    assert_eq!(format!("#b", 27), "0b11011"); // binary
    assert_eq!(format!("#o", 27), "0o33"); // octal
    assert_eq!(format!("#x", 27), "0x1b"); // Hex
    assert_eq!(format!("#X", 27), "0x1B"); // Hex but uppercase

    println!("{:x}!", 27);

    println!("{:#010b}", 27);

    println!("Success!")
}
```

9.
```rust,editable
fn get_person() -> String {
    String::from("sunface")
}

fn get_format() -> (usize, usize) {
    (4, 1)
}


fn main() {
    let person = get_person();
    println!("Hello, {person}!");

    let (width, precision) = get_format();
    let scores = [("sunface", 99.12), ("jack", 60.34)];
    for (name, score) in scores {
        println!("{name}: {score:width$.prescision$}");
    }
}
```
