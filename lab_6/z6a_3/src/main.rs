fn indeksy(tablica: &[&str], element: &str) -> Vec<usize> {
    let result: Vec<usize> = tablica
        .into_iter()
        .enumerate()
        .filter(|&(_, &v)| v == element)
        .map(|(i, _)| i)
        .collect();
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
