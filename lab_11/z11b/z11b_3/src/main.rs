#[derive(PartialEq, Debug)]
enum PaymentMethod {
    Cash,
    Card,
    Transfer
}
#[derive(Debug)]
struct Transaction {
    amount: f64,
    method: PaymentMethod
}

impl Transaction {
    fn new(amount: f64, method: PaymentMethod) -> Transaction {
        Transaction {
            amount,
            method
        }
    }
}

fn sumuj_po_metodzie(transakcje: Vec<Transaction>, metoda: PaymentMethod) -> Vec<Transaction> {
    let mut wybrane = Vec::new();
    for t in transakcje {
        if t.method == metoda {
            wybrane.push(t);
        }
    }
    wybrane
}

fn main() {
    let mut transakcje = Vec::new();
    let t = Transaction::new(4200.00,PaymentMethod::Transfer);
    let t1 = Transaction::new(125.99,PaymentMethod::Card);
    let t2 = Transaction::new(1.99,PaymentMethod::Cash);
    let t3 = Transaction::new(32.99,PaymentMethod::Cash);
    let t4 = Transaction::new(1220.0,PaymentMethod::Cash);
    transakcje.push(t);
    transakcje.push(t1);
    transakcje.push(t2);
    transakcje.push(t3);
    transakcje.push(t4);
    println!("{:?}", sumuj_po_metodzie(transakcje,PaymentMethod::Cash));
}
