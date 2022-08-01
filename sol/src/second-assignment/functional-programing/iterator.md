1、
```rust,editable
fn main() {
    let arr = [0; 10];
    for i in arr {
        println!("{}",i)
    }
}
```

2、
```rust,editble
fn main() {
    let mut v = Vec::new();
    for n in 1..101 {
       v.push(n);
    }

    assert_eq!(v.len(), 100);
}
```
3、
```rust,editable
// 1.
fn main() {
    let v = vec![1, 2];
    
    let v1 = v.into_iter(); // Moves the borrow.

    assert_eq!(v1.next(), Some(1));
    assert_eq!(v1.next(), Some(2));
    assert_eq!(v1.next(), None);
}
```
```rust,editable
// 2.
fn main() {
    let v = vec![1, 2];
    
    let v1 = v.iter(); // Doesn't move the borrow.

    assert_eq!(v1.next(), Some(1));
    assert_eq!(v1.next(), Some(2));
    assert_eq!(v1.next(), None);
}
```

4、
```rust,editable
fn main() {
    let arr = vec![0; 10];
    for i in arr.iter() { // Create iterator
        println!("{}", i)
    }

    println!("{:?}",arr);
}
```

5、
```rust,editable
fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() { // Mutable iterator
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
```

6、
```rust,editable
fn main() {
    let mut values = vec![1, 2, 3];
    let mut values_iter = values.iter_mut();

    if let Some(v) = values_iter.into_next() {
       *v = 0;
    }

    assert_eq!(values, vec![0, 2, 3]);
}
```

7、
```rust,editable
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.curr + self.next;

        self.curr = self.next;
        self.next = next;

        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    let mut fib = fibonacci();
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(2));
    assert_eq!(fib.next(), Some(3));
    assert_eq!(fib.next(), Some(5));
}
```

8、
```rust,edtiable
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total = v1_iter.sum();

    assert_eq!(total, 6);

    println!("{:?}, {:?}",v1, v1_iter);
}
```

9、
```rust,editable
use std::collections::HashMap;
fn main() {
    let names = [("sunface",18), ("sunfei",18)];
    let folks: HashMap<_, _> = names.into_iter().collect();

    println!("{:?}",folks);

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<i32> = v1.into_iter().collect(); // Moved borrow

    assert_eq!(v2, vec![1, 2, 3]);
}
```

10、
```rust,editable
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2 = v1.iter().map(|x| x + 1).collect::<i32>();

    assert_eq!(v2, vec![2, 3, 4]);
}
```

11、
```rust
use std::collections::HashMap;
fn main() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();

    println!("{:?}",folks);
}
```

12、 
```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| is_shoe_size(&x, &shoe_size)).collect()
}

// Being extra for no reason.
fn is_shoe_size(x: &Vec<Shoe>, shoe_size: &u32) -> bool {
    *x.shoe == *shoe_size 
}

fn main() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}
```
