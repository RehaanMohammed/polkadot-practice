### Basic Operations
1. 

Types inside HashMap need to be homogenous
```rust,editable
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58); 
    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98)); 
    // This returns an Option with reference because, we do not
    // want to get clone or remove the value present at that index.
    // Use get_mut() if you want that or just do last() to get element at the last index.

    if scores.contains_key("Daniel") {
        let score = scores["Daniel"];
        assert_eq!(score, Some(95));
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score)
    }
}
```

2.
```rust,editable

use std::collections::HashMap;
fn main() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    let mut teams_map2: HashMap<_,_> = teams.into_iter().collect();

    assert_eq!(teams_map1, teams_map2);

    println!("Success!")
}
```

3. 
```rust,editable

use std::collections::HashMap;
fn main() {
    let mut player_stats = HashMap::new();

    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"], 100);

    player_stats.entry("health").or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100);

    let health = player_stats.entry("health").or_insert(50);
    assert_eq!(health, &100);
    *health -= 50;
    assert_eq!(*health, 50);

    println!("Success!")
}

fn random_stat_buff() -> u8 { 42 }
```
4. 
```rust,editable

use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}
```

5. 
```rust,editable
use std::collections::HashMap;
fn main() {
  let v1 = 10;
  let mut m1 = HashMap::new();
  m1.insert(v1, v1);
  println!("v1 is still usable after inserting to hashmap : {}", v1);

  let v2 = "hello";
  let mut m2 = HashMap::new();
  m2.insert(v2, v1);
    
  assert_eq!(v2, "hello");

  println!("Success!")
}
```
