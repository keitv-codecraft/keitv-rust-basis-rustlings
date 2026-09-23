#![allow(clippy::explicit_iter_loop, clippy::useless_vec)]

fn main() {
    let namen = vec!["Arin", "Borin"];
    for naam in namen.iter() {
        println!("{naam}");
    }
}
