# Statements and Expressions

1. 
```rust,editable
// 1) 
fn main() {
   let v = {
       let mut x = 1;
       x += 2;
       x
   };

   assert_eq!(v, 3);

   println!("Success!");
}
```
```rust,editable
// 2)
fn main() {
   let v = {
       let mut x = 1;
       x + 2
   };

   assert_eq!(v, 3);

   println!("Success!");
}
```

2.
```rust,editable
fn main() {
   let v = {let x = 3; x}; // Paranthesis can only be used for tuples.
// There should also be a return statement otherwise the expression will return ()
   assert!(v == 3);

   println!("Success!");
}
```

3.
```rust,editable
fn main() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

// semicolon means the statement is not returning instead () is returned
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
```
