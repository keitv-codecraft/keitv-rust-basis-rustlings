use std::{
    f64::consts::{PI, TAU},
    mem::swap,
};

fn main() {
    // Soms willen we dat een veriabele al berekent wordt tijdens compilatie.
    // Hiertoe gebruiken we `const` in plaats van `let`. Bijvoorbeeld,
    {
        const FACTOR: f64 = 4.0 / 3.0 * PI;
        let radius: f64 = 10.0;
        println!(
            "Een bal met straal 10 heeft een volume van {}",
            FACTOR * radius.powi(3)
        );
    }

    // De berekening mag afkomstig zijn van een functie, maar dan moet deze
    // functie zelf ook `const` zijn. Bijvoorbeeld,
    {
        const fn degrees_to_radians(value: f64) -> f64 {
            value / 360.0 * TAU
        }
        const DEGREES_TO_RADIANS: f64 = degrees_to_radians(1.0);
        // De functie is ook gewoon tijdens runtime te gebruiken:
        println!("180 degrees = {} radians", degrees_to_radians(180.0));
        println!("90 degrees = {} radians", 90.0 * DEGREES_TO_RADIANS);
    }

    // TODO: Maak de fibonacci functie efficienter door compile-time
    //       constanten te introduceren
    {
        const RT5: f64 = 2.236_067_977_499_79;
        const PHI: f64 = f64::midpoint(1.0, RT5);
        const PSI: f64 = PHI - 1.0;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        fn fibonacci(n: u16) -> u64 {
            let n = (n + 1).into();
            let fibn = (PHI.powi(n) - PSI.powi(n)) / RT5;
            fibn.round() as u64
        }

        fn expected(n: u16) -> u64 {
            if n < 2 {
                return n.into();
            }
            let mut a = 0;
            let mut b = 1;
            for _ in 1..=n {
                swap(&mut a, &mut b);
                b += a;
            }
            b
        }

        for n in 0..10 {
            assert_eq!(fibonacci(n), expected(n), "{n}th Fibonacci numer");
        }
    }
}
