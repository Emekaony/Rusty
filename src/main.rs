use crate::house::house::House;
use crate::Color::Green;
use crate::Person::Name;

mod audio;
mod haha;
mod house;
mod player;
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

enum DayOfTheWeek {
    Monday,
    Tuesday,
    Wednesday,
}

static mut x: i32 = 10;

fn random_stuff() {
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

    let h1: House = House {
        number_of_floors: 1,
        number_of_rooms: 3,
    };
    println!("It is {} that h1 is a duplex", h1.is_duplex());
    println!(
        "It is {} that h1 can contain Legion",
        h1.can_contain_legion()
    );

    let today: DayOfTheWeek = DayOfTheWeek::Tuesday;
    match today {
        DayOfTheWeek::Monday => println!("It is Monday"),
        DayOfTheWeek::Tuesday => println!("It is Tuesday"),
        DayOfTheWeek::Wednesday => println!("It is Wednesday"),
    }
    unsafe {
        println!("x is unsafe but its value is: {}", x);
    }
}

fn function_module() {
    // get back to this and fix the lifetime later!
    // let append_to_string = |original: &mut String, to_add: &str| {
    //     original.push_str(to_add);
    //     original
    // };
    let sum = |a: i32, b: i32| a + b;
    let nums: (i32, i32) = (1, 2);
    println!(
        "Sum of {} and {} is {}",
        nums.0,
        nums.1,
        sum(nums.0, nums.1)
    );
}

fn main() {
    function_module();
}
