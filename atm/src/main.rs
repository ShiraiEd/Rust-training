use std::io;

#[derive(Debug, Default)]
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
            .all(|x| x.name == name && x.password == password)
        {
            println!("Welcome");
            true
        } else {
            println!("User not found");
            false
        }
    }

    fn check_balance(&self, user: &User){
        println!("Balance: {}\nDebt: {}", user.balance, user.debt);
        println!("Press d to deposit, w to withdraw, q to quit");

    }
}

fn main() {
    //instancia atm
    let mut atm = ATM::new(("Tigrinho").to_string());

    println!("Criar usuarios? Y or N");

    //verifica se usuario quer criar novo user
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read choice");
    if choice.trim().to_lowercase() == "y" {
        //cria loop chamado create
        'create: loop {
            let mut name = String::new();
            let mut password = String::new();
            let mut balance = String::new();
            let mut debt = String::new();

            println!("Name: ");
            io::stdin()
                .read_line(&mut name)
                .expect("Failed to read name");

            println!("Password: ");
            io::stdin()
                .read_line(&mut password)
                .expect("Failed to read password");

            println!("balance: ");

            io::stdin()
                .read_line(&mut balance)
                .expect("Failed to read balance");

            println!("debt");
            io::stdin()
                .read_line(&mut debt)
                .expect("Failed to read debt");
            //trata as variaveis
            let name = name.trim().to_string();
            let password = password.trim().to_string();
            let balance: f64 = balance.trim().parse().unwrap();
            let debt: f64 = debt.trim().parse().unwrap();
            
            //passa o usuario para o vetor da atm
            let user = User::new(name, password, balance, debt);
            atm.add_user(user);

            println!("Continue? Y or N");

            let mut choice1 = String::new();

            io::stdin().read_line(&mut choice1).expect("Failed to read");

            if choice1.trim().to_lowercase() == "n" {
                break 'create;
            }
        }

    }
    
    atm.return_users();
    
    let mut name = String::new();
    let mut password = String::new();
    
    println!("Logging: ");
    
    println!("name: ");
    io::stdin().read_line(&mut name).expect("Failed to read name");
    
    println!("Password: ");
    io::stdin().read_line(&mut password).expect("Failed to read password");
    
    let name = name.trim().to_string();
    let password = password.trim().to_string();
    
    //login
    if atm.login(&name, &password){
        if let Some(user_name) = atm.users.iter().find(|x| x.name == name.to_string()){
            atm.check_balance(user_name);
            
        };
    }
}
