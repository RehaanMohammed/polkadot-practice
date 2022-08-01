# Match, if let

### Match
1. 
```rust,editable
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North  => {
            println!("South or North");
        },
        _ => println!("YOU SHALL NOT BE HERE!"),
    };
}
```

2.
```rust,editable
fn main() {
    let boolean = true;

    let binary = match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);

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
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }

    println!("Success!");
} 

fn show_message(msg: Message) {
    match msg {
        Message::Move{x: a, y: b} => {
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        _ => println!("no data in these variants")
    }
}
```

### matches!

4. 
```rust,editable
fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9')); 
        // checking with regular expression?
        // "matches" macro is new to me. Very interesting though.
        // I can already imagine it being used for validating stuff.
    }

    println!("Success!");
} 
```

5. 
```rust,editable
enum MyEnum {
    Foo,
    Bar
}

fn main() {
    let mut count = 0;

    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) {
            // "matches" macro seems quite handy.
            // NOICE
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}
```

### If let

6. 
```rust,editable
fn main() {
    let o = Some(7);

    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
        println!("Success!");
    };
}
```

7.
```rust,editable
enum Foo {
    Bar(u8)
}

fn main() {
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);
        println!("Success!");
    }
}
```

8.
```rust,editable
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let a = Foo::Qux(10);

    match a {
       Foo::Bar =>  println!("match foo::bar"),
       Foo::Baz =>  println!("match foo::baz"),
       _ => println!("match others"),
    }
}
```

### Shadowing
9.
```rust,editable
fn main() {
    let age = Some(30);
    if let Some(age) = age {
       assert_eq!(age, 30);
    }
    
    match age {
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
 }
 ```
