1.
   
```rust,editable
struct Array<T, const N: usize> {
    data : [T; N]
}

fn main() {
    let arrays = [
        Array{
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3], // They all should be the same type or else too ambigious.
        },
        Array {
            data: [1, 2]
        }
    ];

    println!("Success!");
}
```

2. 
```rust,editable

fn print_array<T: std::fmt::Debug, const N: usize>(arr: &[T; N]) {
    println!("{:?}", *arr);
}
fn main() {
    let arr = [1, 2, 3];
    print_array(&arr);

    let arr = ["hello", "world"];
    print_array(&arr);
}
```

3.
   
```rust,editable
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    //...
}

fn main() {
    check_size([0u8; 767]); 
    check_size([0i32; 191]);
    check_size(["hello你好"; 47]);
    check_size([(); 31].map(|_| "hello你好".to_string())); 
    check_size(['中'; 191]); 

    println!("Success!");
}



pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}
```

