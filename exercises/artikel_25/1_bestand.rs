use std::fs;

fn main() -> Result<(), std::io::Error> {
    // TODO: Schrijf "save" naar oefening.txt.
    fs::write("oefening.txt", "")?;
    Ok(())
}