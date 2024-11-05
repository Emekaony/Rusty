#[derive(Debug)]
struct Person {
    name: String,
}

#[derive(Debug)]
struct Dog<'a> {
    // Struct has lifetime a
    /*
    We need to find a way to tell the rust compiler that owner
    will live just as long as the structure lives
    so we don't have dangling reference situation.
     */
    name: String,
    owner: &'a Person, // owner will live as long as a
}

impl Dog<'_> {
    fn bark(&self) {
        println!(
            "{} is barking. Have {} come get him/her pls!",
            self.name, self.owner.name
        );
    }
}

pub fn run() {
    let person = Person {
        name: String::from("John"),
    };
    let dog = Dog {
        name: String::from("pepe"),
        owner: &person,
    };
    dog.bark();
}
