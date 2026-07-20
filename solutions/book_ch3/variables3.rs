fn main() {
    // Expressies zoals 42u128 , 3.14f32 ; 365 en "KeiTV" zijn zogenaamde
    // literals. Deze zijn bezonder omdat ze constant zijn tijdens
    // compilatie en daardoor rechtsreeks als bits in je uiteindelijke
    // programma terecht komen. Laten we eens kijken naar wat verschillende
    // literals.

    // Integers schrijven we meestal in base 10 (tien), omdat we gewend zijn
    // om zo te tellen: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 en daarna beginnen we
    // weer bij 0 met een 1 ervoor: 10, 11, 12, 13, ...
    // Als we bijvoorbeeld 527 schrijven dan bedoelen we eigenlijk
    // 5*100 + 2*10 + 7*1. Oftewel, 5⋅10² + 2⋅10¹ + 7⋅10⁰
    // 10 is ons aangeleerd en dus eenvoudig, maar een computer denkt in bits.
    // Een bit kan alleen de waarde 0 of 1 hebben. Een computer denkt dus in
    // base 2. Als je wilt begrijpen hoe een getal er in base-2 uitziet, moet
    // je dus wat hersengymnastiek doen.
    // Bijvoorbeeld, het getal 5 kun je schrijven als 4 + 1. Dit is
    // 1⋅2² + 0⋅2¹ + 1⋅2⁰ en dus ziet de computer 101 (base-2) in plaats van 5.
    // Dit is de binaire representatie van 5.
    // Gelukkig van de computer dit ook voor ons doen:
    println!("De binaire representatie van 5 is {:#b}", 5);

    // Omgekeerd kunnen we de binaire representatie van 5 ook schrijven als
    // literal door er '0b' voor de zetten.
    println!("De decimale representatie van 101 is {}", 0b101);

    // Van al die nulletjes en eentjes wordt je snel tureluurs, dus zijn er
    // meerdere gebruikelijke bases die zich mentaal makkelijker laten vertalen
    // van en naar binair. Drie bekende voorbeelden zijn octaal, hexadecimaal en
    // base-64. Octaal is base-8 en hexadecimaal is base-16. Voor bases hoger dan
    // tien is het gebruikelijk om letters en andere karakters te gebruiken.
    // Zo tellen we in hexadecimaal met 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C, D,
    // E, F, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1A, 1B, 1C, 1D, 1E, 1F, 20
    // Merk op dat 20 in base-16 dus 2 * 16 = 32 is.
    // Voor octaal gebruik je '0o' en voor hexadecimaal gebruik je '0x'
    println!("De octale representatie van 20 is {:#o}", 20);
    println!("De decimale representatie van 24 is {}", 0o24);
    println!("De hexadecimale representatie van 30 is {:#x}", 30);
    println!("De decimale representatie van 1e is {}", 0x1e);

    // Je ziet allerlei getallen langskomen, zoals 2, 8, 16 en 64. Dit zijn
    // allemaal machten van 2. Het is als programmeur verstandig om ze uit
    // je hoofd te kennen: 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, ...
    // Wanneer over getallen in de computerwereld gaat zijn dit meestal
    // de usual suspects. Nu begrijp je waarom.

    // TODO: Print de octale representatie van het hexadecimale getal 53977.
    println!(
        "De octale representatie van het hexadecimale getal 53977 is {:#o}",
        0x53977
    );
}
