use rand::Rng;

use crate::constants;

pub fn run(length: u8) -> String {
  let mut rng = rand::thread_rng();
  let mut password = String::from("");

  let mut index = 0;
  while index < length {
    let random_number= rng.gen_range(0..constants::SUPPORTED_CHARS.len());
    let char = constants::SUPPORTED_CHARS.chars().nth(random_number);
    if let Some(c) = char {
      password.push(c);
    }
    index = index + 1;
  }

  // index = 0;
  // let random_number = rng.gen_range(0..constants::SPECIAL_CHARACTERS.len());
  // let special_char = constants::SPECIAL_CHARACTERS.chars().nth(random_number);
  // while index < 3 {
  //   if let Some(c) = special_char {
  //     password.push(c);
  //   }
  //   index = index + 1;
  // }

  return password;
}

pub fn add_special_chain(pass: &str, repeat: u8) -> String {
  let mut rng = rand::thread_rng();
  let mut index: u8 = 0;
  let random_number = rng.gen_range(0..constants::SPECIAL_CHARACTERS.len());
  let special_char = constants::SPECIAL_CHARACTERS.chars().nth(random_number);
  let mut password = String::from(pass);

  while index < repeat {
    if let Some(c) = special_char {
      password.push(c);
    }
    index = index + 1;
  }

  return password;
}
