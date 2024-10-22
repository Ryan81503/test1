mod bank_account;

fn main() {
    let mut account = bank_account::BankAccount::new(200.0);
    println!("Initial balance: ${}", account.balance());

    account.deposit(100.0);
    println!("After depositing $100: ${}", account.balance());

    account.withdraw(50.0);
    println!("After withdrawing $50: ${}", account.balance());
}