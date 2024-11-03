#[allow(dead_code)]
struct RustDev {
    awesome: bool,
}

#[allow(dead_code)]
struct JavaDev {
    awesome: bool,
}

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello() {
        println!("Hello world!");
    }
}

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome }
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello() {
        println!("println!(\"Hello world\");");
    }
}

impl Developer for JavaDev {
    fn new(awesome: bool) -> Self {
        JavaDev { awesome }
    }

    fn language(&self) -> &str {
        "Java"
    }

    fn say_hello() {
        println!("System.Out.Println(\"Hello, World!\");");
    }
}

pub fn run() {
    let better_rust_dev: RustDev = RustDev::new(true);
    let better_java_dev: JavaDev = JavaDev::new(false);

    // use "." for methods that have self inside them
    better_java_dev.language();
    better_rust_dev.language();

    // use the methods associated with the type
    RustDev::say_hello();
    JavaDev::say_hello();
}

pub fn returning_traits() {
    struct Dog {}
    struct Cat {}

    trait Animal {
        fn speak(&self) -> String;
    }

    impl Animal for Dog {
        fn speak(&self) -> String {
            String::from("Dog is speaking.")
        }
    }

    impl Animal for Cat {
        fn speak(&self) -> String {
            String::from("Cat is speaking.")
        }
    }

    /*
    similar to how we return "some View" in swift for opaque typing and protocols
    rust can allow us return Traits butt it's a litt more complicated than that.

    Rust likes to know the size of things at compile time for memory safety guarantees.
    We work around this byb using a Box<dyn <trait_name>)
     */

    fn get_animal(random_integer: i32) -> Box<dyn Animal> {
        if random_integer > 5 {
            // make sure to keep the stuff inside a Box
            Box::new(Dog {})
        } else {
            // again, make sure to create a new Box
            Box::new(Cat {})
        }
    }
    //dog
    println!("{}", get_animal(3).speak());
    // cat
    println!("{}", get_animal(7).speak());
}
