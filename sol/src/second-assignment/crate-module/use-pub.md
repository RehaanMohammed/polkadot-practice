1. 

```rust,editable
use std::fmt::Result;
use std::io::Result as IoTypeResult;

fn main() {}
```

2.

```rust,editable
use std::collections::*;
// Not recommended sometimes.
// No need to import modules that are not going to be used.
// Keep your code clean.

fn main() {
    let _c1:HashMap<&str, i32> = HashMap::new();
    let mut c2 = BTreeMap::new();
    c2.insert(1, "a");
    let _c3: HashSet<i32> = HashSet::new();
}
```
```rust
use std::collections::{HashMap, BTreeMap, HashSet};
// This is the recommended way to import.
// Not only does this avoid importing unnecessary modules, it is also understandable.
// This way anyone reading the code can know what was imported from the modules.

fn main() {
    let _c1:HashMap<&str, i32> = HashMap::new();
    let mut c2 = BTreeMap::new();
    c2.insert(1, "a");
    let _c3: HashSet<i32> = HashSet::new();
}

```

3. 
```rust,editable
// Add this line in lib.rs
// pub use crate::front_of_house::hosting;
fn main() {
    assert_eq!(hello_package::hosting::seat_at_table(), "sit down please");
    assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");
}
```
