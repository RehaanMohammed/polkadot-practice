# Reference and Borrowing

### Reference
1. 
```rust,editable
fn main() {
   let x = 5;
   let p = &x;

   println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}
```

2.
```rust,editable
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, *y);

    println!("Success!");
}
```

3. 
```rust,editable
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s); // argument expects &string

    println!("Success!");
}

fn borrow_object(s: &String) {}
```

4.
```rust,editable
fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s); // Sending mutable reference

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}
```

5.
```rust,editable
fn main() {
    let mut s = String::from("hello, ");

    let p = &mut s;
    
    p.push_str("world");

    println!("Success!");
}
```

#### Ref
`ref` can be used to take references to a value, similar to `&`.

6. 
```rust,editable
fn main() {
    let c = '中';

    let r1 = &c;
    let ref r2 = c; // ref type makes working with addresses and pointers a little safer.

    assert_eq!(*r1, *r2);
    
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
```

### Borrowing rules
7.
```rust,editable
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s; // no need to mutate

    println!("{}, {}", r1, r2);

    println!("Success!");
}
```

#### Mutability
8.
```rust,editable
fn main() {
    let mut s = String::from("hello, "); //  cannot be mutated if the original variable ain't mutable.

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}
```

9.
```rust,editable
    let mut s = String::from("hello, ");

    borrow_object(&s);
    
    s.push_str("world");

    println!("Success!");
}

fn borrow_object(s: &String) {}
```
### NLL
10. 
```rust,editable
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    
    // println!("{}",r1); cause s is borrowed to r2 now
}
```

11.
```rust,editable
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // You can't use r1 and r2 at the same time
	
    r1.push_str(r2);
}
```
