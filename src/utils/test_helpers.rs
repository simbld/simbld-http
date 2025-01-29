use lazy_static::lazy_static;
use std::sync::Mutex;

// Lazy static mutex to store the output of a test
lazy_static! {
    static ref BUFFER_OUTPUT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

// Function to capture the output of a test
pub fn capture_test<F: FnOnce()>(func: F) -> Vec<String> {
  let mut buffer = BUFFER_OUTPUT.lock().unwrap();
  buffer.clear();
  
  func();
  
  buffer.clone()
}

// Function to push a message to the buffer
pub fn push_to_buffer(msg: String) {
  let mut buffer = BUFFER_OUTPUT.lock().unwrap();
  buffer.push(msg);
}

// Function to get the content of the buffer
pub fn get_buffer_content() -> Vec<String> {
  BUFFER_OUTPUT.lock().unwrap().clone()
}