use crate::clean;

#[allow(dead_code)]

pub fn play_movie(name: &str) {
    println!("Playing movie {}", name);
    clean::perform_cleanup_public();
}
#[allow(dead_code)]
fn play_audio(name: &str) {
    println!("Playing audio {}", name)
}

#[allow(dead_code)]
pub fn new_stuff() {
    println!("He said the file is now a moudle?");
}
