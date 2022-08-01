1、
```rust,editable
fn main() {
    let color = String::from("green");

    let print = || println!("`color`: {}", color); // no need to move

    print();
    print();

    println!("{}",color);
}
```

2、
```rust,editable
fn main() {
    let mut count = 0;

    let mut inc = move || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();


    let _reborrow = &count; 

    inc();

    let _count_reborrowed = &mut count; 

    assert_eq!(count, 0);
}
```

3、
```rust,editable
// 1.
fn main() {
     let movable = Box::new(3);

     let consume = || {
         println!("`movable`: {:?}", movable);
         take(movable);
     };

     consume();
    // consume(); already moved
}

fn take<T>(_v: T) {}
```
```rust
// 2.
fn main() {
     let movable = Box::new(3);

     let consume = move || {
         println!("`movable`: {:?}", movable);
         take(&movable); // by reference only.
     };

     consume();
     consume();
}

fn take<T>(_v:&T) {}
```
4、
```rust,editable
fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));

    let n = example_closure(String::from("5".to_owned()));
}
```
5、
```rust,editable
fn fn_once<F>(func: F)
where
    F: Fn /*Once*/ (usize) -> bool /* + Copy  also another way*/,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}
```

6、
```rust,editable
fn main() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}",s);
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F)  {
    f("hello")
}
```
7、
```rust,editable

fn apply<F>(f: F) where
    F: FnOnce() {

    f();
}

fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);

        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
```
```rust
fn main() {
    let s = String::new();

    let update_string = |str| -> String { s.push_str(str); s };

    exec(update_string);
}

fn exec<'a, F: FnOnce(&'a str) -> String>(mut f: F)  {
    f("hello");
}
```

8、
```rust,editable
fn main() {
    let mut s = String::new();

    let update_string = |str| -> String {s.push_str(str); s };

    exec(update_string);
}

fn exec<'a, F: FnOnce(&'a str) -> String>(mut f: F) {
    f("hello");
}
```

9、
```rust,editable

fn call_me<F: Fn()>(f: F) { // Using generics
    f();
}

fn function() {
    println!("I'm a function!");
}

fn main() {
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
```

10、
The reason we use a Box here is because, we do not know the size of a trait at compile time. Hence, we use Box which is a Heap allocation. Also, the useof "dyn" keyword. Basically, it is used to distinguish and address dynamic dispatch when you're using traits as a type rather than interface. 
```rust,editable
fn create_fn() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    Box::new(move |x| x + num)
}


fn main() {
    let fn_plain = create_fn();
    fn_plain(1);
}
```

11、
```rust,editable
fn factory(x:i32) -> Box<dyn Fn(i32) -> i32> {

    let num = 5;

    if x > 1{
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x + num) // Same here I guess
    }
}
```
