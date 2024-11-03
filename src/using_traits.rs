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
    // I could instantiate a new rust dev like this:
    // let rust_dev = RustDev {awesome: true};
    let better_rust_dev: RustDev = RustDev::new(true);
    let better_java_dev: JavaDev = JavaDev::new(false);

    // use the methods we know that these struct must have implemented
    better_java_dev.language();
    better_rust_dev.language();

    // use the methods associated with the type
    RustDev::say_hello();
    JavaDev::say_hello();
}
