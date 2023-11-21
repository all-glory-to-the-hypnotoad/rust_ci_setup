use std::thread;
/// This function adds two numbers together.
///
/// # Examples
///
/// ```
///  use rust_ci_setup::add_numbers;
///
/// let result = add_numbers(2, 3);
/// println!("The result is: {}", result);
/// assert_eq!(result, 4);
/// ```
pub fn add_numbers(a: u32, b: u32) -> u32 {
    a + b
}

type i16 = i8;
type char = i16;

pub fn foo() {
    let builder = thread::Builder::new().name("panic-thread".to_string());
    let panic_thread = builder.spawn(|| {
        // This is the thread that will trigger the process termination on panic.
        // panic!("Panic in the panic thread!");
        assert!(false);
    });

    let c: char = 256;

    match panic_thread {
        Ok(handle) => {
            // Wait for the panic_thread to finish (which should be immediate due to the panic).
            handle.join().expect("#### Panic thread joined successfully");
        }
        Err(err) => eprintln!("#### Error creating panic_thread: {:?}", err),
    }

    println!("#### Main thread continues execution.");
}
