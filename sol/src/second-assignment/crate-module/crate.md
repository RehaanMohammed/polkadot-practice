
1. cargo new --bin hello-package
```shell
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```

```toml
# in Cargo.toml
[package]
name = "hello-package"
version = "0.1.0"
edition = "2021"
```

2. cargo new --lib hello-package1
```shell
.
├── Cargo.toml
└── src
    └── lib.rs

1 directory, 2 files
```

```toml
# in Cargo.toml
[package]
name = "hello-package1"
version = "0.1.0"
edition = "2021"
```

3.  
```rust,editable
// Q: Whats the difference between package 1# and 2# ?
// A: crate 1 is a binary. It is used for binary executable code.
// The entry point for the crate is from main.
// crate 2 is a library crate. It is used for libraries.
// lib.rs acts as an entry point ABI for the library. 
```

4. 
```rust,editable
// Q: Whats the name of the library crate in package `hello-package1`?
// A: hello-package1
```

5. 
I've been losing my mind over this. How the hell do you type '─', '├' and '│'? Why is this so hard to do?
```shell,editable
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── main.rs
│   └── lib.rs

```

6. 

```shell,editable
.
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── main1.rs
│       └── main2.rs
├── tests 
│   └── some_integration_tests.rs
├── benches 
│   └── simple_bench.rs
└── examples 
    └── simple_example.rs
```
