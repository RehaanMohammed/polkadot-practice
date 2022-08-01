1. 
It is not recommended to use unwrap() if you're unsure about the result. Check if pragmatically the expression is not going to give the unwanted option. In which case unwrap() can be used without an issue. Otherwise it is best to avoid using it and instead use pattern-matching.
```rust,editable

use std::num::ParseIntError;

fn multiply(n1_str: &str,
            n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn main() {
    let result = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result = multiply("4", "2");
    assert_eq!(result.unwrap(), 8);
    
    
    println!("Success!");
}
```

2. 
```rust,editable

use std::num::ParseIntError;

fn multiply(n1_str: &str,
            n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>()?;
    let n2 = n2_str.parse::<132>()?;
    
    // Use this in main when you need to STOP the program because you cannot proceed.
    // In other words, if Error result is an error rather than an exception then it is allowed.
    // In that case, it is okay to use "?" in main. 
    // 
    // This is how to use the above function if the error result is an exception:
    // 
    // if let Ok(result) = multiply("10", "2") {
    //      ... to handle result.
    // } else {
    //      ... to handle error.
    // }
    // 
    // This is essentailly try-catch for rust.

    Ok(n1 * n2)
}

fn main() {
    assert_eq!(multiply("3", "4").unwrap(), 12);
    println!("Success!")
}
```

3. 
```rust,editable

use std::fs::File;
use std::io::{self, Read};

fn read_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
    println!("Success!")
}
```

4. 
```rust,editable
use std::num::ParseIntError;

fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
   n_str.parse::<i32>().map(|x| x + 2);
}

fn main() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!")
}
```

5.
```rust,editable
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    match n1_str.parse::<i32>() {
        Ok(n1)  => {
            match n2_str.parse::<i32>() {
                Ok(n2)  => {
                    Ok(n1 * n2)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    n1_str.parse::<i32>().and_then(|n1| {
        n2_str.parse::<i32>()?.map(|n2| n1 * n2)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // This still presents a reasonable answer.
    let twenty = multiply1("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = multiply("t", "2");
    print(tt);

    println!("Success!")
}
```

6. 
```rust,editable
use std::num::ParseIntError;

type Res<T> = Result<T, ParseIntError>;

fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn print(result: Res<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));

    println!("Success!")
}
```
