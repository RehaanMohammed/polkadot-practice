# Ownership

1. 

```rust,editable
// 1)
fn main() {
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);
}
```
```rust,editable
// 2)
fn main() {
    // using string slices.
    let x = String::from("hello, world");
    let y = &x[..];
    println!("{},{}",x,y);
}
```
```rust,editable
// 3)
fn main() {
    // using &String
    let x = &String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}
```
```rust,editable
// 4)
fn main() {
    // Storing &str and copying that
    let x = "hello, world";
    let y = x;
    println!("{},{}",x,y);
}
```
2. 
```rust,editable
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s // Temporarily s owns it but the owership is returned to s2
}
```


3. 
```rust,editable
fn main() {
    let s = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s = String::from("hello, world");
    // Convert String to Vec
    let _s = s.into_bytes(); // _s owns the string now
    String::from_utf8_lossy(&_s).to_string() // Converting bytes to String

// Here to_string() is required as the type is actually a Cow
// Which basically is a type that holds a smart pointer that provides cloning when borrowed.
}
```

4.
```rust,editable
fn main() {
    let s = String::from("hello, world");

    print_str(&s[..]);

    println!("{}", s);
}

fn print_str(s: &str)  {
    println!("{}",s)
}

// You can also do this: 
// fn main() {
//     let mut s = String::from("hello, world");
// 
//     s = print_str(s);
// 
//     println!("{}", s);
// }
// 
// fn print_str(s: String) -> String {
//     println!("{}",s);
//     s
// }
```

5. 
```rust, editable
fn main() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}
```

```ignore
Why ownership in rust is an important concept:

* Rust doesn't have a garbage collector.
* Instead all the variables are dropped when they leave the scope much like c.
* This then poses a problem with pointers. First we have to understand how strings work.

What is a string?
=================

A string is a series of char with a pointer to the first char
stored in variable and a '\0' delimiter to denote end of the string.

example in c:
       ----                 -----
      |  & |               |     |
      |    v               |     v
char* s = "Rust is awesome!";   '\0' is appended

s holds the address of the char 'R'. Hence, s++ would be 'u'.

So when you something like

char* s1 = s; Here s1 is also a pointer that holds the address of 'R'.

In rust, because of the drop trait that is applied to the variables, at the end of the variable scope,

Now you have 2 pointers pointing to the same data.
If the rust compiler follows through and deletes the pointer and frees the memory it was pointing to,
Then the compiler will delete whatever else memory that the dangling pointer is pointing to.

This all can be solved by using ownership which basically just allows one pointer at a time to memory. Rust borrow-checker will
check the code and keep track of who the owner is at a given point in the code.
```
#### Handling assignments and a full time job is DIFFICULT. I haven't slept since 2 days ... 

<iframe src="https://giphy.com/embed/D12CsrRNv7gL6" width="480" height="360" frameBorder="0" class="giphy-embed"></iframe><p><a href="https://giphy.com/gifs/crazy-family-guy-lol-D12CsrRNv7gL6"></a></p>

#### Mutability

6.
```rust,editable
fn main() {
    let s = String::from("hello, ");
    
    let mut s1 = s;

    s1.push_str("world");

    println!("Success!");
}
```

7.
```rust,editable
fn main() {
    let x = Box::new(5);
    
    let mut y = Box::new(420);
    
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}
```

### Partial move

#### Exercises

8.
```rust,editable
fn main() {
   let t = (String::from("hello"), String::from("world"));

   let _s = t.0;

   println!("{:?}", t.1); // since t.0 is borrowed we can only use t.1
}
```

9.
```rust,editable
fn main() {
   let t = (String::from("hello"), String::from("world"));

    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
```
