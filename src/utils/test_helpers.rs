use lazy_static::lazy_static;
use std::io::{self, Write};
use std::sync::Mutex;

// Create a global buffer to capture the output
lazy_static! {
  static ref OUTPUT: Mutex<Vec<u8>> = Mutex::new(Vec::new());
}

// Function to capture the output of tests
pub fn capture_output<F: FnOnce()>(func: F) -> String {
  let mut buffer = OUTPUT.lock().unwrap();
  buffer.clear(); // Clear the buffer before capturing new output

  // Replace stdout with a temporary buffer
  let previous_stdout = io::stdout();
  let mut handle = previous_stdout.lock();
  let _ = write!(handle, ""); // Redirect stdout

  func();

  // Return the captured content
  String::from_utf8(buffer.clone()).expect("Invalid UTF-8 output")
}
