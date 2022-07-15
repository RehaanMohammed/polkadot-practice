# Functions
1.
```rust,editable
fn main() {
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

// Each argument type must be declared because, rust hates ambiguity at run time.
fn sum(x: i32, y: i32) -> i32 { // add a return type and remove semicolons to return expression
    x + y
}
```


2. 🌟
```rust,editable
fn main() {
   print();
}

fn print() -> () { // No need to specify () just keeping it blank will suffice
   println!("Success!");
}
```


3.

```rust,editable
// 1) Using panic to stop a program abruptly
#[allow(unreachable_code)]
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
   panic!(); 
}
```
```rust,editable
// 2) Using never ending loop.
// This should never be used in real code.
#[allow(unreachable_code)]
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
   loop{}
}
```

### Diverging functions 
Diverging functions never return to the caller, so they may be used in places where a value of any type is expected.

4. 
```rust,editable
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
// 1)
fn never_return_fn() -> ! {
    panic!();
}

// 2)

// fn never_return_fn() -> ! {
//     loop{}
// }

// 3)

// fn never_return_fn() -> ! {
//     std::process::exit(1);
// }
```

5. 🌟🌟
```rust,editable
fn main() {
    let b = false;

    let v = match b {
        true => 1,
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}
```
