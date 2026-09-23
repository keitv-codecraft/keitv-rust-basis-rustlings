fn main() {
    let mut teller = 1;

    loop {
        println!("{teller}");
        teller += 1;
        if teller > 3 {
            break;
        }
    }
}
