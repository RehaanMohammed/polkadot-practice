1. 
```rust,editable
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { 
    	self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);

    println!("Success!");
}
```

2. 

```rust,editable
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self)  {
        println!("the current state is {}", self.color);
    }
}
fn main() {
    let light = TrafficLight{
        color: "red".to_owned(),
    };
    light.show_state();
    println!("{:?}", light);
}
```

3. 
```rust,editable
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(Self)  {
        println!("the current state is {}", self.color);
    }

    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}
fn main() {
    println!("Success!");
}
```

4. 

```rust,editable
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn new() -> Self {
    	TrafficLight {
        	color: "red".to_owned(),
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

fn main() {
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");

    println!("Success!");
}
```

5.
```rust,editable

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    println!("Success!");
}
```
6. 

```rust,editable

#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl TrafficLightColor {
	pub fn color(&self) -> &str {
		match *self {
			TrafficLightColor::Red => "red",
			TrafficLightColor::Yellow => "yellow",
			TrafficLightColor::Green => "green",
		}
	}	    
}

fn main() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}
```
