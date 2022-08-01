1. 
```rust,editable

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        
        panic!("Totally uneccessary"); // You shouldn't be using panic this way though.
     }

    println!("Excercise Failed if printing out this line!");
}

fn main() {
    drink("lemonade");

    println!("Excercise Failed if printing out this line!");
}
```

2. 
```rust,editable
fn main() {
    assert_eq!("abc".as_bytes(), [96, 97, 98]);

    let v = vec![1, 2, 3];
    let ele = v[2];
    let ele = v.get(2).unwrap();

    let v = production_rate_per_hour(2);

    divide(15, 1);

    println!("Success!")
}

fn divide(x:u8, y:u8) {
    println!("{}", x / y)
}

fn production_rate_per_hour(speed: u8) -> f64 {
    let cph: u8 = 21;
    match speed {
        1..=4 => (speed * cph) as f64,
        5..=8 => (speed * cph) as f64 * 0.9,
        9..=10 => (speed * cph) as f64 * 0.77,
        _ => 0 as f64,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60 as f64) as u32
}
```
3. 
```shell
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `[97, 98, 99]`,
 right: `[96, 97, 98]`', src/main.rs:3:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/std/src/panicking.rs:498:5
   1: core::panicking::panic_fmt
             at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed
             at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:154:5
   4: study_cargo::main
             at ./src/main.rs:3:5
   5: core::ops::function::FnOnce::call_once
             at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
