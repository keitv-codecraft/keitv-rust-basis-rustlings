#[derive(Debug, PartialEq)]
struct Vijand {
    gezondheid: i32,
}

fn main() {
    let mut vijanden = vec![Vijand { gezondheid: 30 }, Vijand { gezondheid: 50 }];

    // TODO: Trek 10 gezondheid af bij iedere vijand.
    assert_eq!(vijanden[0].gezondheid, 20);
    assert_eq!(vijanden[1].gezondheid, 40);
}