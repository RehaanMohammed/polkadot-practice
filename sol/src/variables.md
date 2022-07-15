# Variables

### Binding and mutability
1. 
```rust,editable
fn main() {
    let x: i32 = 5; // Uninitialized but used, ERROR !

//Every variable must be initialized before using. duh... 

    let _y: i32; // Uninitialized but also unused, only a Warning !

// The convention is to add '_' before identifiers if you are not going to use them

    assert_eq!(x, 5);
    println!("Success!");
}
```

2. 
```rust,editable
fn main() {
    let mut x = 1;
    x += 2; 

// Variables are immutable by default
    
    assert_eq!(x, 3);
    println!("Success!");
}
```

### Scope

3.  
```rust,editable
fn main() {
    let x: i32 = 10;
//  {

    let y: i32 = 5;
    println!("The value of x is {} and value of y is {}", x, y);
    
//  } if braces were enabled then the variable 'y' will be destroyed here 

// Easiest way would be to change the scope of the variable.

    println!("The value of x is {} and value of y is {}", x, y); 
}
```

4. 
```rust,editable
fn main() {
// add function call to "define_x"
    println!("{}, world", define_x()); 
}

// Here "'a" is the lifetime parameter defined for the string that is returned
// This helps rust to identify the lifetime of a string variable so that the 
// compiler can safely destroy the string after it's lifetime is completed.
// In this example the lifetime "'a" is bound to the function call "define_x"
// after the function call, the string will be destroyed.

fn define_x<'a>() -> &'a str {
// Returning &str
    "hello"
}
```

### Shadowing

5. 
```rust,editable
fn main() {
    let x: i32 = 5;
    {
// Here the variable x holds the value "12" in this scope
        let x = 12;
        assert_eq!(x, 12);
    }

// Here the value is "5" in this scope
    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}
```

6. 🌟🌟 
```rust,editable
fn main() {
    let mut _x: i32 = 1;
    _x = 7;
    // Shadowing and re-binding
    {
    // Shadowing must be done in a new scope
    let mut _x = _x; // add "mut" if you're going to mutate
    _x += 3;
    
    }

    let _y = 4;
    // Shadowing
    {
    
    let _y = "I can also be bound to text!"; 
    
    }
    
    println!("Success!");
}
```

### Unused variables
1.

```rust,editable
#[allow(unused_variables)] // 1) Allow unused variable
fn main() {
    let _x = 1; // 2) The convention is to add "_" before an unused variable 
}

// Warning: unused variable: `x`
```

### Destructuring
8.

```rust,editable
fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

// Make 'x' mutable

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
```

### Destructuring assignments

9.

```rust,editable
fn main() {
    let (x, y);
    (x,..) = (3, 4);// 3 is assigned to x
    [.., y] = [1, 2];// 2 is assigned to y
// (x,...) is checked with (3, 4)

    assert_eq!([x,y], [3, 2]);

    println!("Success!");
} 
```
```ignore
    ----------
   |          |
   |    ---------
   |   |      |  |
   |   |      v  v
  (x, ...) = (3, 4); The "..." is ignored.

 Similarly,

    ---------
   |         |
   |    --------
   |   |     |  |
   |   |     v  v 
 [..., y] = [1, 2];

Hence, x = 3 and y = 2

```
