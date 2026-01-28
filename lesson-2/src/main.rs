
#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![]}
    }
}

impl Account {
    fn new(_id:u32, _holder:String) -> Self {
        Account { id: (_id), balance: 0, holder: (_holder) }
    }
}

fn print_acc(account: &Account){
    println!("{:#?}", account);

}

// Every value is owned by a single variable at a time
// Reassigning the value to a variable, passing it to a function, putting it into a vector, etc
// moves the value. The old owner can't be used to access the value anymore!
fn main() {
    let account = Account::new(1, String::from("melih"));
    print_acc(&account);
    
    println!("test-1: {:#?}", &account);
    println!("test-2: {:#?}", account);
    //can't move a value while a ref to the value exists
    
}
