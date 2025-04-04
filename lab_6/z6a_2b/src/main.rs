fn shorter_than_four(test_strings: Vec<&str>) -> Vec<String> {
    let mut result: Vec<_> = vec![];
    for i in test_strings {
        if i.len() < 4 {
            result.push(i.to_string());
        }
    }
    result
}

fn no_a_or_A(test_strings: Vec<&str>) -> Vec<String> {
    let mut result: Vec<_> = vec![];
    let mut found = false;
    for i in test_strings {
        let c = i.chars();
        for a in c {
            if a == 'A' || a == 'a' {
                found = true;
            }
        }
        if !found {
            result.push(i.to_string());
        }
        found = false;
    }
    result
}

fn contains_numbers(test_strings: Vec<&str>) -> Vec<String> {
    let mut result: Vec<_> = vec![];
    let mut found = false;
    for i in test_strings {
        let c = i.chars();
        for a in c {
            match a.to_digit(10) {
                Some(l) => {found = true},
                None => continue,
            }
            if found {
                break;
            }
        }
        if found {
            result.push(i.to_string());
        }
        found = false;
    }
    result
}

fn reversed_strings(test_strings: Vec<&str>) -> Vec<String> {
    let mut result: Vec<_> = vec![];
    for i in test_strings {
        //let a = i.to_string().chars().rev().collect();
        let mut reversed = String::new();
        let chars: Vec<char> = i.chars().collect();
        let mut i = chars.len();
        while i > 0 {
            i -= 1;
            reversed.push(chars[i]);

        }
        result.push(reversed);
    }
    result
}

fn doubled_letter(test_strings: Vec<&str>) -> Vec<String> {
    let mut result: Vec<_> = vec![];
    let mut found = false;
    for v in test_strings {
        let chars: Vec<char> = v.chars().collect();
        let mut i = 0;
        while i < chars.len() - 1 {
            if chars[i] == chars[i+1] {
                found = true;
            }
            i += 1;
            if found {
                break;
            }
        }
        if found {
            result.push(v.to_string())
        }
        found = false;
    }
    result
}

fn main() {
    let test_strings:Vec<&str> = vec![
        "kot", "pies", "dom", "Ala", "zamek", "król", "robot", "1234", "test1",
        "inny", "pizza", "brutto", "lekki", "dzienny", "programowanie", "Rust",
        "wow", "gamma", "delta", "epsilon", "hello123", "abcd", "xyz", "foo", "bar",
    ];
    println!("shorter_than_four: {:?}\n", shorter_than_four(test_strings.clone()));
    println!("no_a_or_A: {:?}\n", no_a_or_A(test_strings.clone()));
    println!("contains_numbers: {:?}\n", contains_numbers(test_strings.clone()));
    println!("reversed_strings: {:?}\n", reversed_strings(test_strings.clone()));
    println!("doubled_letter: {:?}\n", doubled_letter(test_strings.clone()));

}
