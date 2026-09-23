use std::fs;

fn main() -> Result<(), std::io::Error> {
    fs::write("oefening.txt", "save")?;
    Ok(())
}
