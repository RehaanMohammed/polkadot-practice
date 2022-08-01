1. 
```rust,editable
struct Structure(i32);

fn main() {
    println!("{} months in a year.", 12);

    println!("Now {:?} will print!", Structure(3));
}
```

2. 
```rust,editable
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

fn main() {
    let person = Person { name:  "Sunface".to_string(), age: 18 };
    println!("{:#?}", person);
}
```

3. 
```rust,editable
use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

impl fmt::Debug for Structure {
    fn fmt(&self, f: fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl fmt::Debug for Deep {
    fn fmt(&self, f: fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{:?}", self.0) 
    }
}

fn main() {    
    println!("Now {:?} will print!", Deep(Structure(7)));
}

```
4. 
```rust,editable

use std::fmt;

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Display: {} + {}i", self.0, self.1)
    }
}

impl fmt::Debug for Point2D {
    fn fmt(&self, f: fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Debug: Complex{ real: {}, imag: {} }", self.0, self.1)
}

fn main() {
    let point = Point2D { x: 3.3, y: 7.2 };
    assert_eq!(format!("{}",point), "Display: 3.3 + 7.2i");
    assert_eq!(format!("{:?}",point), "Debug: Complex { real: 3.3, imag: 7.2 }");
    
    println!("Success!")
}
```
5. 
```rust,editable
use std::fmt; 

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    assert_eq!(format!("{}",v), "[0: 1, 1: 2, 2: 3]");
    println!("Success!")
}
```

