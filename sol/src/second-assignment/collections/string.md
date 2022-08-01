### Basic operations
1. 
```rust,editable

fn main() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    move_ownership(s.clone()); // ownership not moved instead cloned.

    assert_eq!(s, "hello, world!");

    println!("Success!")
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}
```

2.
```rust,editable
fn main() {  
   let mut s = String::from("hello, world");

   let slice1: &str = &s[..];
// let slice1: &str = s.as_str();
   assert_eq!(slice1, "hello, world");

   let slice2 = &s[0..5];
   assert_eq!(slice2, "hello");

   let mut slice3: String = s; 
   slice3.push('!');
   assert_eq!(slice3, "hello, world!");

   println!("Success!")
}
```

3. 

Two Heap allocations have happned here. "s" is one then another is created for after using "slice" to create one.

```rust,editable
 fn main() {  
   let s: String = String::from("hello, world!");

   let slice: &str = &s;

   let s: String = slice.to_string();

   assert_eq!(s, "hello, world!");

   println!("Success!")
}
```

4. 

```rust,editable
fn main() {
    let s = String::from("hello, 世界");
    let slice1 = s[0..1];
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10];
    assert_eq!(slice2, "世");
    
    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }

    println!("Success!")
}
```

5. 

```rust,editable

fn main() {
    let mut s = String::new();
    s.push_str();

    let v = vec![104, 101, 108, 108, 111];

    let s1 = String::from_uft8(v).unwrap(); 
// Cause string is essentially a wrapper for vector of u8 numbers.
    
    assert_eq!(s, s1);

    println!("Success!")
}
```

6.
```rust,editable
fn main() {
    let mut s = String::with_capacity(25);

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!")
}
```
Essentially structure of a string from source
```

struct String {
	vec: Vec<u8>
}

impl String {
	fn with_capacity(capacity: usize) -> String {
		String {vec: Vec::with_capacity(capacity)}
	}
}
```
## From the std crate

The capacity of a vector is the amount of space allocated for any future
elements that will be added onto the vector. This is not to be confused with
the *length* of a vector, which specifies the number of actual elements
within the vector. If a vector's length exceeds its capacity, its capacity
will automatically be increased, but its elements will have to be
reallocated.

For example, a vector with capacity 10 and length 0 would be an empty vector
with space for 10 more elements. Pushing 10 or fewer elements onto the
vector will not change its capacity or cause reallocation to occur. However,
if the vector's length is increased to 11, it will have to reallocate, which
can be slow. For this reason, it is recommended to use [`Vec::with_capacity`]
whenever possible to specify how big the vector is expected to get.

Essentially a string with capacity is just capacity is the reserved space that
a string expects to get so it allocates it just in case and increases when 
more space is demanded. Makes sense. Also, I can use "with_capacity" now to reduce
useless memory allocation. Yay, to more efficient code!

7.
```rust,editable

use std::mem;

fn main() {
    let story = String::from("Rust By Practice");

    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr(); // mutable pointer like in C/C++.
    let len = story.len();
    let capacity = story.capacity();

    assert_eq!(16, len);

    let s = unsafe { String::from_raw_parts(ptr, len, capacity) }; // lo here come the dreaded unsafe block

    assert_eq!(*story, s);

    println!("Success!")
}
```
