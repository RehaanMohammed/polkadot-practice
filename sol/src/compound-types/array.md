# Array
1.  
```rust,editable
fn main() {
    let arr: [u8; 5] = [1, 2, 3, 4, 5];

    assert!(arr.len() == 5);

    println!("Success!");
}
```
2. 
```rust,editable
fn main() {
    let arr0 = [1, 2, 3];
    let arr: [char; 3] = ['a', 'b', 'c'];
    
    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12); // cause 4 x 3 = 12 quik mafs

    println!("Success!");
}
```

3.

```rust,editable
fn main() {
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}
```
4.
```rust,editable
fn main() {
    let _arr = [1, 2, 3];// array are homogenous unlike tuples

    println!("Success!");
}
```
5.
```rust,editable
fn main() {
    let arr = ['a', 'b', 'c'];
    
    let ele = arr[0];

    assert!(ele == 'a');

    println!("Success!");
}
```
6. 
```rust,editable
fn main() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
    let name0 = names.get(0).unwrap();

    let _name1 = &names[1];

    println!("Success!");
}

```
