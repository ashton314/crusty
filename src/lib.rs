#[no_mangle]
pub extern "C" fn greet_generic() {
    println!("Greetings chap. Rust has been called.");
}

pub mod heap;
pub mod cheap_job;
