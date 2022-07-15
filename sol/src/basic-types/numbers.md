# Numbers

### Integer

1.  

```rust,editable
fn main() {
    let x: i32 = 5;
    let mut y: i32 = 5; // make U32 to I32 so that the types of x and y can match

    y = x;
    
    let z = 10; // any unconstrained integer will default to type {Integer} which is i32.

    println!("Success!");
}
```

2. 
```rust,editable
fn main() {
    let v: u16 = 38_u8 as u16; // "as" keyword is used for explicit type conversion

    println!("Success!");
}
```
3.   

```rust,editable
fn main() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x)); // Integer is defaulted to i32 when type is not assigned explicitly.

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
```

4.  
```rust,editable
fn main() {
    assert_eq!(i8::MAX, 127); // The range is from -128 to 127 
    assert_eq!(u8::MAX, 255); // The range is from 0 to 255

    println!("Success!");
}
```

5.  
```rust,editable
fn main() {
   let v1 = 251_u16 + 8; // converting to u16 to increase the range
   if let Some(v2) = u8::checked_add(251, 8) {
       println!("{},{}",v1,v2);
   } else {
       println!("The sum will be out of range of \'u8\'.\nHence we cannot add \'8\' to it.");
   }
}
```
6. 
```rust,editable
fn main() {
    let v = 1_024 /* decimal 1024 */ + 0xff /* hexa decimal 255*/ + 0o77 /* octal 63 */ + 0b1111_1111 /* binary 255 */;
    
    assert!(v == 1597);

    println!("Success!");
}
```
### Floating-Point
7. 

```rust,editable
fn main() {
    let x = 1_000.000_1 // f64 is default
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64 

    println!("Success!");
}
```

8. 

I remember seeing this [video](https://www.youtube.com/shorts/s9F8pu5KfyM) on YouTube that explained why computers suck at math because of rounding error.

```rust,editable
// 1) Using rounding function.

fn main() {
    
    assert!(0.1_f32.round() + 0.2_f32.round() == 0.3_f32.round());

    println!("Success!");
}
```

```rust,editable
// 2) Using absolute values

fn main() {
    
    assert!(0.1_f32.abs() + 0.2_f32.abs() == 0.3_f32.abs());

    println!("Success!");
}
```
### Range
9. 

```rust,editable
fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5); // Refer below

    for c in 'a'..='z' {
        println!("{}",c as u8); // explicit conversion to u8
    }
}
```
|Iter |Variables                        |Computation             |
|-----|---------------------------------|------------------------|
|iter1| i =	-3	sum =	00	| 	0 + (-3) 	 |	
|iter2| i =	-2	sum =	-3	| 	-3 - 2 		 |
|iter3| i =	-1	sum =	-5	| 	-5 - 1 		 |
|iter4| i =	0	sum =	-6 	| 	-6 + 0 		 |
|iter5| i =	1	sum =	-6 	| 	-6 + 1 		 |
|iter6| i =	2	sum =	-5 	| 	break; 		 |

10. 🌟🌟 

```rust,editable
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}
```
### Computations

11. 🌟 
```rust,editable
fn main() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1); // change to i8 as negative values are out of range 
    
    assert!(3 * 50 == 150);

    assert!(9.6_f32.abs() / 3.2_f32.abs() == 3.0); // Use absolute values

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
```
