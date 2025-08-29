mod constants;
mod generate;
mod validate;
mod track;

fn main() {
    println!("Password generator v0.0.1");
    let mut password: String = String::from("");
    let mut analyzed_passwords = 1;

    while !validate::validate_lower(&password) || !validate::validate_upper(&password) || !validate::validate_number(&password) || !validate::validate_special_char(&password) {
        password = generate::run(13);
        analyzed_passwords += 1;
    }

    password = generate::add_special_chain(&password, 3);
    let res = track::write(&password);

    match res {
        Ok(()) => print!(""),
        Err(e) => println!("{}", e),
    }

    println!("Generated password ({}): {}\n", analyzed_passwords, password);
    println!("Build with ❤️\nby RubenDguez using 🦀");
}
