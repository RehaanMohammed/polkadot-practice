# Patterns

1.
```rust
# fn main() {
fn match_number(n: i32) {
    match n {
	1 => println!("One!"),
	
	2 | 3 | 4 | 5 => println!("match 2 -> 5"),
	
	6..=10 => println!("match 6 -> 10"),

	_ => println!("match 11 -> +infinite")
    }
}
# match_number(4);
# } 
```

2.
```rust,editable
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 5, y: 10 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
```

3.

```rust,editable
enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id:  id@3..=7,
        } => println!("Found an id in range [3, 7]: {}", id),
        Message::Hello { id: newid@(10 | 11 | 12) } => {
            println!("Found an id in another range [10, 12]: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
```

4.
```rust,editable
fn main() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x != 0 => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}
```

5.
```rust,editable
fn main() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first,..,last) => {
           assert_eq!(first, 2);
           assert_eq!(last, 2048);
        }
    }

    println!("Success!");
}
```

6.
```rust,editable
fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        value => value.push_str(" world!"), // r is already a mutable borrow
    }

	// why can't we just do 
	// r.push_str(" world!") 😆
	println!("{}", v);
}
```
