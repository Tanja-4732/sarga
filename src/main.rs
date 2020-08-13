mod cli;
mod constants;
mod core;
mod test;

fn main() {
    println!("Sarga version {}", constants::VERSION);
    println!("Copyright 2020 Bernd-L; All rights reserved.");
    println!("Licensed under the AGPL 3.0 <https://www.gnu.org/licenses/agpl-3.0.en.html>\n");

    test::test_main();
}
