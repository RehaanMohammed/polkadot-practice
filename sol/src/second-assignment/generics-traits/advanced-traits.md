3. 
```rust,editable
trait Pilot {
    fn fly(&self) -> String;
}

trait Wizard {
    fn fly(&self) -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("Up!")
    }
}

impl Human {
    fn fly(&self) -> String {
        String::from("*waving arms furiously*")
    }
}

fn main() {
    let person = Human;
    
    Pilot::fly(&person);
    assert_eq!(person.fly, "This is your captain speaking.");
    Wizard::fly(&person);
    assert_eq!(person.fly, "Up!");
    Human::fly(&person);
    assert_eq!(person.fly, "*waving arms furiously*");

    println!("Success!");
}
```


4. 
```rust,editable

trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

struct CSStudent {
    name: String,
    university: String,
    fav_language: String,
    git_username: String
}

impl Person for CSStudent {
	fn name(&self) -> String {
		self.name.clone()
	}	
}

impl Student for CSStudent {
	fn university(&self) -> String {
		self.university.clone()
	}
}

impl Programmer for CSStudent {
	fn fav_language(&self) -> String {
		self.fav_language.clone()
	}
}

impl CompSciStudent CSStudent {
	fn git_username(&self) -> String {
		self.git_username.clone()
	}
}

fn main() {
    let student = CSStudent {
        name: "Sunfei".to_string(),
        university: "XXX".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "sunface".to_string()
    };

    println!("{}", comp_sci_student_greeting(__));
}
```

5.
```rust,editable
use std::fmt;

// DEFINE a newtype `Pretty` here
struct Pretty(String)

impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}

fn main() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}
```
