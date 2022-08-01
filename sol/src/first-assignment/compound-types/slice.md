# Slice

1.
```rust,editable
fn main() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world";

    println!("Success!");
}
```
2.
```rust,editable
fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];
    
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");
}
```

3. 
```rust,editable
fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}
```

### String slices
4.  
```rust,editable
fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);

    println!("Success!");
}
```

5. 
```rust,editable
fn main() {
    let s = "你好，世界";
    let slice = &s[0..3];

    assert!(slice == "你");

    println!("Success!");
}
```

6.
```rust,editable
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {}", word);
    
    s.clear();

}
fn first_word(s: &str) -> &str {
    &s[..1]
}
```
