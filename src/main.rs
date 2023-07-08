struct UserAccount {
    name: String,
    age: Option<u32>,
}

trait Balance {
    fn get_balance(&self) -> u32 {
        10
    }
}

impl Balance for UserAccount {}

fn increase_balance<T: Balance>(x: &T, amount: u32) -> Result<u32, String> {
    // use if let for types of user_account get return Ok of get_balance
    if amount <= 10 {
        return Ok(x.get_balance() + amount);
    } else {
        return Err("Increase must be less than 10!".to_owned());
    }
}

fn main() {
    let user_account = UserAccount {
        name: "Raju".to_owned(),
        age: None,
    };

    match increase_balance(&user_account, 11) {
        Ok(v) => println!("UserAccount balance increased to {}", v),
        Err(e) => println!("{}", e),
    }

    if let Some(age) = user_account.age {
        println!("UserAccount age is {}", age);
    }
}
