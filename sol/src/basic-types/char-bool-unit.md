# Char, Bool and Unit

### Char
1. 
```rust, editable
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); // char is 4 bytes 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); // similarly this is also 4 bytes.

    println!("Success!");
} 
```
2. 
```rust, editable
fn main() {
    let c1 = '中';

// Single quotes is for character and double quotes is for strings.
// This ain't Python. We have proper standards and conventions.

    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}
```

### Bool
3. 
```rust, editable
fn main() {
    let f: bool = false;

    let t = f;

// Could've just removed the bang but, I guessed we had to use the unused variable.

    if !t {
        println!("Success!");
    }
} 
```

4. 🌟
```rust, editable
fn main() {
    let f = true;
    let t = true || false;
    assert_eq!(t, f);

    println!("Success!");
}

```
AND truth table:

|X     |Y     |X&Y   |
|------|------|------|
|true  |false |false |
|false |true  |false |
|false |false |false |
|true  |true  |true  |

### Unit type
5. 
```rust,editable
fn main() {
    let w: () = ();

    let v = (2, 3);
    assert_eq!(w, implicitly_ret_unit()); // I can't think of anything but changing the variable name

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
```

6.
```rust,editable
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0); // size of unit type is zero

    println!("Success!");
}
```
