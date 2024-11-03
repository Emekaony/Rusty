use crate::houses;
use crate::Color::Green;
use crate::Person::Name;

mod audio;
mod haha;
mod houses;
mod utilities;

#[derive(Debug)]
struct Employee {
    firstname: String,
    company: String,
    age: u32,
}

impl Employee {
    fn details(&self) -> String {
        format!(
            "name: {}, age: {}, company: {}",
            self.firstname, self.age, self.company
        )
    }

    fn mutate_name(&mut self, new_name: String) {
        println!("Changing firstname from {} to {}", self.firstname, new_name);
        self.firstname = new_name;
    }

    fn static_details() -> String {
        String::from("Details of a person")
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum Person {
    Name(String),
}

fn main() {
    let mut emp: Employee = Employee {
        firstname: "Nnaemeka".to_string(),
        company: "Rhombus".to_string(),
        age: 23,
    };
    println!("{}", emp.details());
    println!("{:?}", emp);
    // when using static functions, use the :: thing not the dot notation
    println!("static employee details: {}", Employee::static_details());

    // I would say I have a decent understanding of rust ownsership rules tbh (besides lifetimes).
    emp.mutate_name(String::from("Kamsi"));
    println!("{}", emp.firstname);

    // enums
    let my_color: Color = Color::Red;
    println!("{:?}", my_color);
    let tt = Green;

    println!("{}", utilities::math::sum(10, 20));

    audio::fourier::implement_fourier_transform();

    let person = Name("emeka".to_string());
    println!("{:?}", person);
}
