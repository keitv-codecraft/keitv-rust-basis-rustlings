#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
fn main() {
    // Rust heeft veel verschillende data types. Bijvoorbeeld,
    // i8, u8, i16, u16, i32, u32, i64, u64, i128, u128 voor de integers
    // f16, f32, f64 voor de floating point getallen
    // char voor karakters en bool voor boolean waarden.
    // Dit zijn de zogenaamde primitieve typen. Rust stelt ons ook in
    // staat om nieuwe types te maken op basis van bestaande types.
    // Zo is er het type [T; N] voor een array voor N instanties van type T.
    // Er is ook een tuple type (T1, T2, ..., Tn) voor een lijst van types Ti.

    // Daarnaast zijn er veel verschillende operators om de waarden van
    // verschillende typen te combineren tot nieuwe waarden.
    // Zie de volgende pagina voor een volledige lijst:
    // https://doc.rust-lang.org/book/appendix-02-operators.html

    // Er is ook de mogelijkheid om types op andere manieren te combineren
    // tot nieuwe types door gebruik te maken van `struct` en `enum`.
    // Hier komen we later op terug.

    // In deze opgave kijken we naar veelvoorkomende expressies van types.
    // Aan jou de taak om de expressies te corrigeren, zodat deze compileren.
    // TODO: fix de compiler errors
    {
        let x = 1.0_f32 + 2.0_f64;
        assert!((x - 3.0) < 0.0001);
    }
    {
        let x = 10u8 * 2u16;
        assert_eq!(x, 20);
    }
    {
        let x: u8 = 100u16 / 3;
        assert_eq!(x, 33);
    }
    {
        let x = 7.0 < 20;
        assert!(x);
    }

    // Het komt vaak voor dat veriabelen naar een ander type omgezet moeten
    // worden. Het is belangrijk dat dit zorgvuldig gebeurd.
    // Zo is er geen probleem om een integer om te zetten naar een breder
    // integer type, maar voor veel andere conversies moeten we expliciet
    // zijn bij het afhandelen van problemen. Afhankelijk van de situatie
    // gebruiken we hiervoor verschillende methoden.
    {
        let x = 64.9_f64;
        let y: f32 = x as f32; // Vergeet de hoge precisie informatie
        assert!((y - 64.9) < 0.0001);
    }
    {
        let x = 64.9_f32;
        let y: u8 = x as u8; // Vergeet alles achter de komma
        assert_eq!(y, 64u8);
    }
    {
        let x = 64.9_f32;
        let y: u8 = x.round() as u8; // Rond af voor truncation
        assert_eq!(y, 65u8);
    }
    {
        let x = 42u32;
        let y = u64::from(x); // Conversie die niet kan falen
        assert_eq!(y, 42u64);
    }
    {
        let x = 42u32;
        let y = u16::try_from(x); // Conversie die kan falen
        if let Ok(y) = y {
            assert_eq!(y, 42u16);
        }
    }

    // TODO: Voer de volgende conversies veilig uit
    {
        let x = 20u8;
        let y: u16 = x;
        assert_eq!(y, 20u16);
    }
    {
        let x = 20i8;
        let y: u8 = x;
        assert_eq!(y, 20u8);
    }
    {
        let x = 20u8;
        let y: i8 = x;
        assert_eq!(y, 20i8);
    }
    {
        let x = 20u8;
        let y: f32 = x;
        assert!((y - 20.0) < 0.0001);
    }
    {
        let x = 20u32;
        let y: f32 = x;
        assert!((y - 20.0) < 0.0001);
    }
    {
        let x = 20.0_f32;
        let y: u32 = x;
        assert_eq!(y, 20u32);
    }
    {
        let x = 20.7_f32;
        let y: u32 = x;
        assert_eq!(y, 21u32);
    }

    // Het casten met `as` is beter om te vermijden waar mogelijk,
    // omdat het problemen kan veroorzaken (bugs)
    // Standaard houdt Rust dit tegen, maar voor deze oefening
    // hebben we het maar even toegelaten ;)
    // Zie direct boven deze main functie. Zie ook de volgende pagina
    // voor een complete lijst van potentiele problemen waar Rust
    // op controleert.
    // https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html
}
