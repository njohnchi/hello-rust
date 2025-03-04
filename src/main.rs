fn main() {
    let my_dog = Dog::from(
        String::from("Bingo"),
        2.45,
        String::from("Tommy")
    );
    my_dog.intro();

    let mut your_dog = Dog::new();
    your_dog.name = String::from("Pluto");
    your_dog.age = 4.72;
    your_dog.owner = String::from("Jane");
    your_dog.intro();

    let another_dog = Dog::new();
    another_dog.intro();
}

struct Dog {
    name: String,
    age: f64,
    owner: String,
}

impl Dog {
    fn new() -> Self {
        Self {
            name: String::new(),
            age: 0.0,
            owner: String::new(),
        }
    }

    fn from(name: String, age: f64, owner: String) -> Self {
        Self { name, age, owner }
    }

    fn intro(&self) {
        println!("My name is {}, I am {}, and my owner is {}",
                 self.name, self.age, self.owner);
    }
}
