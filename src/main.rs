use std::collections::HashMap;
fn main() {
    println!("Hello, world!");

    // let mut balances = step1::BalanceModule {
    //     balance: HashMap::new(),
    // };
    // balances.balance.insert(1, 100);
    // println!("{:?}", balances.balance.get(&1).unwrap());

    let mut balances = step1::BalanceModule::new();

    balances.balance.insert(1, 100);
    balances.set_balance(2, 200);

    let result = balances.transfer(1, 2, 50);

    println!("{:?}",result);


}

mod step1;
mod step2;
mod step3;

#[test]
pub fn test_step_1() {
    let mut balances = step1::BalanceModule::new();
    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    assert!(balances.get_balance(1) == 100);
    assert!(balances.get_balance(2) == 200);

    balances.transfer(1, 2, 50);
    assert!(balances.get_balance(2) == 250);
    //assert_eq!(balances.balance.get(&1).unwrap(),100);
}

#[test]
pub fn test_step_2() {
    let mut balances = step2::BalanceModule::new();
    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    assert!(balances.get_balance(1) == 100);
    assert!(balances.get_balance(2) == 200);

    balances.transfer(1, 2, 50);
    assert!(balances.get_balance(2) == 250);
    //assert_eq!(balances.balance.get(&1).unwrap(),100);
}

#[test]
pub fn test_step_3() {

    type AccountId = u16;
    type Balance = u32;
    let mut balances = step3::BalanceModule::new();
    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    assert!(balances.get_balance(1) == 100);
    assert!(balances.get_balance(2) == 200);

    balances.transfer(1, 2, 50);
    assert!(balances.get_balance(2) == 250);
    //assert_eq!(balances.balance.get(&1).unwrap(),100);
}

