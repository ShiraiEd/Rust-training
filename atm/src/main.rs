use std::io;

/// Reads a line from stdin and returns it trimmed.
/// Avoids repeating String::new() + read_line() + trim() everywhere.
fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

#[derive(Debug)]
struct User {
    name: String,
    password: String,
    balance: f64,
    debt: f64,
}

impl User {
    fn new(name: String, password: String, balance: f64, debt: f64) -> Self {
        Self {
            name,
            password,
            balance,
            debt,
        }
    }
}

#[derive(Debug)]
struct ATM {
    name: String,
    users: Vec<User>,
}

impl ATM {
    fn new(name: String) -> Self {
        Self {
            name,
            users: Vec::new(),
        }
    }

    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    fn return_users(&self) {
        self.users.iter().for_each(|x| println!("{}", x.name));
    }

    fn login(&self, name: &str, password: &str) -> bool {
        if self
            .users
            .iter()
            .any(|x| x.name == name && x.password == password)
        {
            println!("Welcome");
            true
        } else {
            println!("User not found");
            false
        }
    }

    fn deposit(&mut self, name: &str, amount: f64) {
        // iter_mut() gives us mutable references so we can modify the user's balance
        if let Some(user) = self.users.iter_mut().find(|x| x.name == name) {
            user.balance += amount;
            println!("Deposited {:.2}. New balance: {:.2}", amount, user.balance);
        } else {
            println!("User not found");
        }
    }

    fn withdraw(&mut self, name: &str, amount: f64) {
        if let Some(user) = self.users.iter_mut().find(|x| x.name == name) {
            // check if the user has enough balance before withdrawing
            if amount > user.balance {
                println!("Insufficient funds. Your balance is {:.2}", user.balance);
            } else {
                user.balance -= amount;
                println!("Withdrew {:.2}. New balance: {:.2}", amount, user.balance);
            }
        } else {
            println!("User not found");
        }
    }

    fn pay_debts(&mut self, name: &str) {
        if let Some(user) = self.users.iter_mut().find(|x| x.name == name) {
            if user.debt <= 0.0 {
                println!("You have no debts!");
                return;
            }

            println!("How much of the debt do you want to pay? A for all, v for a specific value");
            let input = read_input().to_lowercase();
            match input.as_str() {
                "a" => {
                    if user.balance < user.debt {
                        println!("Not enough funds to cover all the debts");
                    } else {
                        user.balance -= user.debt;
                        user.debt = 0.0;
                        println!("Debt all paid, funds remaining: {:.2}", user.balance);
                    }
                }
                "v" => {
                    println!(
                        "Balance: {:.2}\nDebt: {:.2}\nInput the amount to be paid:",
                        user.balance, user.debt
                    );
                    let amount: f64 = read_input().parse().expect("Please type a number");
                    if amount > user.balance {
                        println!("Not enough funds in the account");
                    } else if amount > user.debt {
                        println!("Amount surpasses the total debt, please try again");
                    } else {
                        user.balance -= amount;
                        user.debt -= amount;
                        println!("Debt paid successfully. Balance: {:.2}", user.balance);
                    }
                }
                _ => {
                    println!("Invalid option, please try again");
                }
            }
        }
    }
}

fn main() {
    //instancia atm
    let mut atm = ATM::new(("Tigrinho").to_string());

    println!("Criar usuarios? Y or N");
    println!();

    //verifica se usuario quer criar novo user
    if read_input().to_lowercase() == "y" {
        //cria loop chamado create
        'create: loop {
            println!("Name: ");
            let name = read_input();

            println!("Password: ");
            let password = read_input();

            println!("Balance: ");
            let balance: f64 = read_input().parse().expect("Please type a number");

            println!("Debt: ");
            let debt: f64 = read_input().parse().expect("Please type a number");

            //passa o usuario para o vetor da atm
            let user = User::new(name, password, balance, debt);
            atm.add_user(user);

            println!("Continue? Y or N");

            if read_input().to_lowercase() == "n" {
                break 'create;
            }
        }
    }
    println!();
    atm.return_users();
    println!();

    println!("Logging: ");

    println!("Name: ");
    let name = read_input();

    println!("Password: ");
    let password = read_input();

    //login
    if atm.login(&name, &password) {
        // after successful login, enter the operations loop
        loop {
            // print balance without holding a mutable borrow
            if let Some(user) = atm.users.iter().find(|x| x.name == name) {
                println!("Balance: {:.2}\nDebt: {:.2}", user.balance, user.debt);
            }
            println!("Press d to deposit, w to withdraw, p to pay debts, q to quit");

            let action = read_input().to_lowercase();

            match action.as_str() {
                "d" => {
                    println!("Amount to deposit: ");
                    let amount: f64 = match read_input().parse() {
                        Ok(v) => v,
                        Err(_) => {
                            println!("Invalid amount");
                            continue;
                        }
                    };
                    atm.deposit(&name, amount);
                }
                "w" => {
                    println!("Amount to withdraw: ");
                    let amount: f64 = match read_input().parse() {
                        Ok(v) => v,
                        Err(_) => {
                            println!("Invalid amount");
                            continue;
                        }
                    };
                    atm.withdraw(&name, amount);
                }
                "q" => {
                    println!("Goodbye!");
                    break;
                }
                "p" => {
                    atm.pay_debts(&name);
                }
                _ => {
                    println!("Invalid option. Use d, w, or q.");
                }
            }
        }
    } else {
        println!("Login failed.");
    }
}
