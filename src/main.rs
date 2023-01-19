// Rust Fundamentals

struct BankAccount {
    balance: i32,
    verified: bool
}

fn print_balance(account: &BankAccount) {
    println!("{:?}", account.balance)
}

fn print_verified(account: &BankAccount) {
    println!("{:?}", account.verified)
}

fn main() {
    // Variables (snake_case) - Variables are immutable unless the mut keyword is provided.
    let mutable_tag = "CHANGE ME";
    let mut my_mutable_name = mutable_tag;
    let my_name = "Zach Ayers";
    let foo = add(1,2);
    let items: [i32;2] = [1, 2];
    let vec = vec![1,2];
    let mut vec_struct = Vec::new();
    vec_struct.push(3);
    vec_struct.push(4);

    println!("{:?}", my_mutable_name);
    println!("{:?}", items);
    println!("{:?}", vec);
    println!("{:?}", vec_struct);

    match my_mutable_name {
        "CHANGE ME" => my_mutable_name = my_name,
        _ => return
    }

    println!("{:?}", my_name);
    println!("{:?}", my_mutable_name);
    println!("{:?}", foo);

    let my_account = BankAccount {
        verified: false,
        balance: 100
    };

    print_balance(&my_account);
    print_verified(&my_account);
}

fn add(num_one :i32, num_two :i32) -> i32 {
    return num_one + num_two;
}


