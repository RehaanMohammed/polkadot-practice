1. 
```shell
library crate root
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         ├── take_payment
         └── complain
```

> current file: lib.rs
```rust,editable
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        
        fn seat_at_table() {}
    }
    
    mod serving {
        fn take_order() {}
        
        fn serve_order() {}

        fn take_payment() {}

        fn complain() {}
    }
}
```

2. 
> current file: lib.rs
```rust
mod front_of_house {
    mod hosting {
        fn add_to_watlist() {}
        
        fn seat_at_table() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}
```

3. 
> current file: lib.rs
```rust,editable
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
        // crate::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}
```

4. 
```shell
.
├── Cargo.toml
├── src
│   ├── back_of_house.rs
│   ├── front_of_house
│   │   ├── hosting.rs
│   │   ├── mod.rs
│   │   └── serving.rs
│   ├── lib.rs
│   └── main.rs
```

> current file: lib.rs
```rust,editable
mod front_of_house;
mod back_of_house;
pub fn eat_at_restaurant() -> String {
    front_of_house::hosting::add_to_waitlist();

    back_of_house::cook_order();

    String::from("mamma mia!")
}
```

> current file: src/back_of_house.rs
```rust,editable
use crate::front_of_house;

pub fn fix_incorrect_order() {
    cook_order();
    crate::front_of_house::serving::serve_order();
}

fn cook_order() {}
```

> current file: src/back_of_house/mod.rs
```rust,editable
pub mod hosting;
pub mod serving;
```

> current file: src/front_of_house/hosting.rs
```rust,editable
fn add_to_waitlist() {
    // add to a vector/struct with vector of waitilist.
}

fn seat_at_table() {
    String::from("sit down please")
}

```

> current file: src/front_of_house/serving.rs
```rust,editable
pub fn take_order() {}

pub fn serve_order() {}

pub fn take_payment() {}

fn complain() -> String {
    String::from("I spat on his food")
}

```

5. 

> current file: src/main.rs
```rust,editable
mod front_of_house;

fn main() {
    assert_eq!(front_of_house::hosting::seat_at_table(), "sit down please");
    assert_eq!(hello_package::eat_at_restaurant(), "yummy yummy!");
    
    // Oops, a customer snuck to the back and overheard!
    assert_eq!(front_of_house::serving::complain(), "I spat on his food");
}
```
