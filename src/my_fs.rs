pub mod load_file {
  use std::fs::File;
  use std::io::prelude::*;
  use std::io::BufReader;

  pub fn read_file(filepath: &str) -> String {
      let file = File::open(filepath).expect("could not open file::");
      let mut buffered_reader = BufReader::new(file);
      let mut contents = String::new();
      let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
          Ok(_number_of_bytes) => _number_of_bytes,
          Err(_err) => 0,
      };

      contents
  }
}
