//! Doing some random arithmetic because, Mafs.
//! I just want to sleep bruh...


/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// # // panics on division by zero
/// comments_docs::compute::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> Result<i32, !> {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    Ok(a / b)
}

/// # Results
/// 
/// Sends Result back when divisor is less zero.
/// ```
///
/// fn try_main() -> Result<(), String> {
///        let res = comments_docs::compute::try_div(10, 0)?;
/// #      Ok(()) // returning from try_main
/// }
/// 
/// # // Handling the Result so that my compiler doesn't go berserk.
/// # // Who knew that even doc code needs to be compiled.
/// fn main() { 
///      if let Ok(x) = try_main() {
///          println!("The div output is {:?}", x);
///      } else {
///          println!("to infinity and beyond");
///      }
/// }
/// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}

/// Add one to the given value and return the value.
///
/// # Examples
///
/// ```
/// #[doc(inline)]
/// use comments_docs::compute::add_one;
///
/// let arg = 5;
/// let answer = add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/** Add two to the given value and return a new value

# Examples
```
let arg = 5;
let answer = comments_docs::compute::add_two(arg);

assert_eq!(7, answer);

```
*/
pub fn add_two(x: i32) -> i32 {
    x + 2
}
