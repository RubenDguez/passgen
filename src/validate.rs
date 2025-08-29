use crate::constants;

fn str_to_array(str: &str) -> Vec<char> {
    let mut vec: Vec<char> = vec![];

    if str == "" {
        return vec;
    }

    let mut index = 0;
    while index < str.len() {
        if let Some(c) = str.chars().nth(index) {
            vec.push(c);
        }

        index += 1;
    }

    return vec;
}

fn validate(pass: &str, chars: &str) -> bool {
    if pass == "" { return false; }

    let password = str_to_array(pass);
    let chs = str_to_array(chars);

    for x in password.iter() {
        for y in chs.iter() {
            if x == y {
                return true;
            }
        }
    }

    return false;
}

pub fn validate_lower(pass: &str) -> bool {
  return validate(pass, constants::LOWER_ALPHABET)
}

pub fn validate_upper(pass: &str) -> bool {
  return validate(pass, constants::UPPER_ALPHABET)
}

pub fn validate_number(pass: &str) -> bool {
  return validate(pass, constants::NUMBERS);
}

pub fn validate_special_char(pass: &str) -> bool {
  return validate(pass, constants::SPECIAL_CHARACTERS);
}
