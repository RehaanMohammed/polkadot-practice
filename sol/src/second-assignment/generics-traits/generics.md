1. 
```rust,editable

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A)); 
    gen_spec_t(SGen(A)); 
    gen_spec_i32(SGen(12));

    generic::<char>(SGen('r'));

    generic(SGen('r'));

    println!("Success!");
}
```

2.
```rust,editable

fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
	x + y	
}

fn sub<T: std::ops::Sub<Output = T>>(x: T, y: T) -> T {
	x - y	
}

fn opt<T: std::ops::Add<Output = T>,
       E: std::ops::Sub<Output = E>,
       FT: Fn(T, T) -> T,
       FE: Fn(T, E) -> E>
       (x: T, y: A, T: E, func1: FT, func2: FE)  -> E {
       (func1)((func2)(x, y), z)
}

fn calc<E: std::ops::Add<Output = E>, F>
	(x: E, y: E, func: F) -> D
	where F: Fn(E, E)     -> E {
	(func)(x, y)
}

fn main() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    assert_eq!(5, calc(2_i8, 3_i8, sum));
    assert_eq!(-20, calc(20, 30, sub));
    
    assert_eq!(-5, opt(2_i8, 3_i8, 10, sum, sub));

    println!("Success!");
}
```

3. 
```rust,editable

struct Point<T> {
	x: T,
	y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("Success!");
}
```

4. 
```rust,editable

struct Point<T, E> {
    x: T,
    y: E,
}

fn main() {
    let p = Point{x: 5, y : "hello".to_string()};

    println!("Success!");
}
```

5.
```rust,editable

struct Val<T> {
    val: T,
}

impl <T>Val <T>{
    fn value(&self) -> &T {
        &self.val
    }
}


fn main() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}
```

6.

```rust,editable
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<R, S>(self, p: Point<R, S>) -> Point<T, S> {
    	x: self.x,
	y: p.y	
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中'};

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!");
}
```

7. 
```rust,editable

struct Point<T> {
    x: T,
    y: T,
}

impl Point <f32> { // You can specify different implimentations for different types.
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point{x: 5_f32, y: 10_f32};
    println!("{}",p.distance_from_origin());
}
```
