#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: if initial_balance >= 0.0 { initial_balance } else { 0.0 },
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        let account = BankAccount::new(100.0);
        assert_eq!(account.balance(), 100.0);

        // Test creating an account with a negative initial balance
        let account = BankAccount::new(-50.0);
        assert_eq!(account.balance(), 0.0);
    }

    #[test]
    fn test_deposit() {
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert_eq!(account.balance(), 150.0);

        // Test depositing a negative amount
        account.deposit(-20.0);
        assert_eq!(account.balance(), 150.0);
    }

    #[test]
    fn test_withdraw() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(40.0);
        assert_eq!(account.balance(), 60.0);

        // Test withdrawing more than the balance
        account.withdraw(200.0);
        assert_eq!(account.balance(), 60.0);

        // Test withdrawing a negative amount
        account.withdraw(-30.0);
        assert_eq!(account.balance(), 60.0);
    }

    #[test]
    fn test_balance() {
        let account = BankAccount::new(75.0);
        assert_eq!(account.balance(), 75.0);
    }

    #[test]
    fn test_edge_cases() {
        let mut account = BankAccount::new(100.0);

        // Withdraw the exact balance amount
        account.withdraw(100.0);
        assert_eq!(account.balance(), 0.0);

        // Deposit zero
        account.deposit(0.0);
        assert_eq!(account.balance(), 0.0);
    }
}