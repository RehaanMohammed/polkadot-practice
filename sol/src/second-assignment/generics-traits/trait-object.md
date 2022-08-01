1.
```rust,editable

trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}

fn main() {
    let duck = Duck;
    duck.swim();

    if let Some(bird) = hatch_a_bird(2) {
        assert_eq!(bird.quack(), "duck duck");
    }

    if let Some(bird) = hatch_a_bird(1) {
        assert_eq!(bird.quack(), "swan swan");
    }

    println!("Success!");
}   

fn hatch_a_bird(bird_option: u32) -> Option<Box<dyn Bird>> {
	match bird_option {
		1 => Some(Box::new(Swan)),
		2 => Some(Box::new(Duck)),
		_ => None,
	}
}

// There! this code is much safer now. Kinda defeats the point of assert now that I think about it.
// Nevermind. Let's carry on. 
```
2.
```rust,editable 
trait Bird {
    fn quack(&self);
}

struct Duck;
impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}

fn main() {
    let birds: [Box<dyn Bird>; 3] = [Box::new(Duck), Box::new(Swan), Box::new(Duck)];

    for bird in birds {
        bird.quack();
    }
}
```

3.
```rust,editable
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn main() {
    let x = 1.1f64;
    let y = 8u8;

    draw_with_box(Box::new(x));

    draw_with_ref(&y);

    println!("Success!");
}

fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

fn draw_with_ref(x: &u8) {
    x.draw();
}
```

4.
```rust,editable

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

fn static_dispatch<T: Foo>(x: T) {x.method();}

fn dynamic_dispatch(x: &dyn Foo) {x.method();}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!");
}
```


5.
```rust,editable

trait MyTrait {
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> u32 { 42 }
}

impl MyTrait for String {
    fn f(&self) -> String { self.clone() }
}

fn my_function(x: impl MyTrait) -> impl MyTrait {
    x.f()
}

fn main() {
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));

    println!("Success!");
}
```
