# Enum
1. 

```rust,editable
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// Enum cannot be floats. They can however hold float values.
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}


fn main() {
    assert_eq!(Number::One as isize, Number1::One as isize);
    assert_eq!(Number1::One as isize, Number2::One as isize);

    println!("Success!");
} 
```

2. 
```rust,editable
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg1 = Message::Move{x: 1, y: 2}; // Instantiating with x = 1, y = 2 
    let msg2 = Message::Write(String::from("hello, world!")); // Instantiating with "hello, world!"

    println!("Success!");
} 
```

3.
```rust,editable
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Move{x: 1, y: 1};

    if let Message::Move{x: a, y: b} = msg {
        assert_eq!(a, b);
    } else {
        panic!("WHY ARE YOU RUNNING! ");
    }

    println!("Success!");
} 
```

4.

```rust,editable
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
} 

fn show_message(msg: Message) {
    println!("{:#?}", msg);
}
```

5.
```rust,editable
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);
        println!("Success!");
        return
    } 
        
    panic!("WHY ARE YOU RUNNING！");
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```
6.

```rust,editable
use crate::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn prepend(self, elem: u32) -> Self {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(), // recurssion
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.len())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
```
