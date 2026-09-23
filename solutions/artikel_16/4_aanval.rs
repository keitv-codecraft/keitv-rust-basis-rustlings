struct Vijand {
    gezondheid: i32,
}

fn main() {
    let mut vijanden = vec![Vijand { gezondheid: 30 }, Vijand { gezondheid: 50 }];
    let aanval = |vijand: &mut Vijand| {
        vijand.gezondheid -= 10;
    };
    for vijand in &mut vijanden {
        aanval(vijand);
    }
    assert_eq!(vijanden[0].gezondheid, 20);
    assert_eq!(vijanden[1].gezondheid, 40);
}
