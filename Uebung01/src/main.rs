use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Bitte Zahl eingeben");
    let generierte_zahl = rand::thread_rng().gen_range(1,101);

    loop {
        let mut zahl = String::new();
        io::stdin()
            .read_line(&mut zahl)
            .expect("Failed to read line");

        let zahl:i32 = match zahl.trim().parse() {
           Ok(num) => num,
            Err(_) => continue,
        };

        println!("Du hast geraten: {}", zahl);

        match zahl.cmp(&generierte_zahl) {
            Ordering::Less =>  println!("{}","Du bist drunter".red()),
            Ordering::Equal => {
                println!("{}", "Du hast es Getroffen".green());
                break;
            },
            Ordering::Greater =>  println!("{}","Du bis drüber".red())
        }
    }
}
