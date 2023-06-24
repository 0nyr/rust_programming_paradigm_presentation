use std::cmp::Ordering;

struct Position {
    x: f32,
    y: f32,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        if self.x == other.x && self.y == other.y {
            Some(Ordering::Equal)
        } else if self.x <= other.x && self.y <= other.y {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// create a class Boxer that contain the weight and name
// and implement the trait PartialOrd and PartialEq
struct Boxer {
    weight: f32,
    name: String,
}

impl PartialOrd for Boxer {
    fn partial_cmp(&self, other: &Boxer) -> Option<Ordering> {
        if self.weight == other.weight {
            Some(Ordering::Equal)
        } else if self.weight <= other.weight {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl PartialEq for Boxer {
    fn eq(&self, other: &Boxer) -> bool {
        self.weight == other.weight
    }
}


fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    println!("Hello, world!");

    // test largest function on Position
    let position1 = Position { x: 1.0, y: 2.0 };
    let position2 = Position { x: 2.0, y: 10.0 };
    let position3 = Position { x: 100.0, y: 200.0 };
    
    let positions = vec![position1, position2, position3];

    let largest_position = &largest(&positions);
    println!("largest position {{ x:{}, y:{} }}", largest_position.x, largest_position.y);

    // test largest function on boxer
    let boxer1 = Boxer { weight: 110.0, name: String::from("Mike Tyson") };
    let boxer2 = Boxer { weight: 80.0, name: String::from("Muhammad Ali") };
    let boxer3 = Boxer { weight: 100.0, name: String::from("George Foreman") };
    let boxers = vec![boxer1, boxer2, boxer3];

    let largest_boxer = &largest(&boxers);
    println!("largest boxer {{ weight:{}, name:{} }}", largest_boxer.weight, largest_boxer.name);

}
