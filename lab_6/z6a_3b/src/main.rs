fn indeksy(tablica: &[&str], element: &str) -> Vec<usize> {
    let mut result: Vec<_> = vec![];
    for (i,&v) in tablica.into_iter().enumerate() {
        if v == element {
            result.push(i)
        }
    }
    result
}

fn main() {
    let test_strings = vec![
        "kot",
        "pies",
        "dom",
        "Ala",
        "zamek",
        "król",
        "kot",
        "robot",
        "1234",
        "kot",
        "test1",
        "inny",
        "pizza",
        "kot",
        "brutto",
        "kot",
        "lekki",
        "dzienny",
        "kot",
        "programowanie",
        "Rust",
        "wow",
        "gamma",
        "kot",
        "delta",
        "epsilon",
        "hello123",
        "abcd",
        "xyz",
        "foo",
        "bar",
    ];
    println!("{:?}", indeksy(&test_strings, "kot"));
}
