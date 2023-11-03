use std::thread;

fn main() {
    let builder = thread::Builder::new().name("panic-thread".to_string());
    let panic_thread = builder.spawn(|| {
        // This is the thread that will trigger the process termination on panic.
        // panic!("Panic in the panic thread!");
        assert!(false);
    });

    match panic_thread {
        Ok(handle) => {
            // Wait for the panic_thread to finish (which should be immediate due to the panic).
            handle.join().expect("#### Panic thread joined successfully");
        }
        Err(err) => eprintln!("#### Error creating panic_thread: {:?}", err),
    }

    println!("#### Main thread continues execution.");
}
