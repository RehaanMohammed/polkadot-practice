# Struct

### The types of structs
1. 
```rust,editable
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn main() {
    let age = 23;
    let p = Person {
        name: String::from("Rehaan"),
        age: age,
        hobby: String::from("sketching"),
    };

    println!("Success!");
} 
```

2. 
```rust,editable
struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}

impl SomeTrait for Unit {  }
fn main() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
} 

fn do_something_with_unit(u: Unit){ }  
// One could also use this 
// fn do_something_with_unit<T>(u: T) where T: SomeTrait{ }
```

3.

```rust,editable
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let v = Point(0, 127, 255);
    check_color(v);

    println!("Success!");
}   

fn check_color(p: Point) {
    let Point(x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
 }
```

### Operating on structs

4.

```rust,editable
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };
// sunface is definitely older than 18
    p.age = 30;

    p.name = String::from("sunfei");

    println!("Success!");
}
```

5.
```rust,editable
struct Person {
    name: String,
    age: u8,
}
fn main() {
    println!("Success!");
} 

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name 
    }
}
```

6.
```rust,editable
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("Success!");
} 

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}
```

### Print the structs
7.

```rust,editable
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("{:#?}", rect1); // Print debug info to stdout
}
```

### Partial move

#### Exercises

8. 
```rust,editable
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn main() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name = f.name;

    println!("{}, {}", _name, f.data);
} 
```
