fn main() {
    let list1: Vec<u8> = vec![1, 2, 20, 99];
    list1
        .iter()
        .map(|&x| get_animal(x))
        .for_each(|an| an.move_body())
}

trait Animal {
    fn move_body(&self);
}

enum Bird {
    Goose(u8),
    Cuckoo,
}

struct Dog {}
struct Cat {}

impl Animal for Bird {
    fn move_body(&self) {
        match *self {
            Bird::Goose(x) => println!("goose move {}", x),
            Bird::Cuckoo => println!("cuckoo move"),
        }
    }
}

impl Animal for Dog {
    fn move_body(&self) {
        println!("dog move");
    }
}

impl Animal for Cat {
    fn move_body(&self) {
        println!("cat move");
    }
}

fn get_animal(n: u8) -> Box<dyn Animal> {
    match n {
        1 => Box::new(Bird::Goose(22)),
        2 => Box::new(Bird::Cuckoo),
        20 => Box::new(Dog {}),
        _ => Box::new(Cat {}),
    }
}
