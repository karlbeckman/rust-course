#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl  Account {
    fn new(id: u32, holder: String) -> Self {
        Account { id: id, balance: 0, holder: holder }
    }

    fn deposit(& mut self, amount: u32) {
        self.balance += amount as i32;
    }

    fn withdraw(& mut self, amount: u32) {
        self.balance -= amount as i32;
    }

    fn summary(&self) -> String {
        format!("Holder: {}, balance: {}", self.holder, self.balance)
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(& mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account|account.summary())
            .collect::<Vec<String>>()
    }
}


fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("Karl Beckman"));
    account.deposit(120);

    bank.add_account(account);
    bank.add_account(Account::new(2, String::from("Snorren 23")));
    println!("Total balance: {}", bank.total_balance());

    println!("{:#?}", bank.summary());
}
