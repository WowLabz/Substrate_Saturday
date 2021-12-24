use std::collections::HashMap;

type AccountId = u32;
type Balance = u32;

pub struct BalanceModule {
    pub balance: HashMap<AccountId, Balance>,
}

impl BalanceModule {
    pub fn new() -> Self {
        Self {
            balance: HashMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: AccountId, amount: Balance) {
        self.balance.insert(who, amount);
    }

    pub fn get_balance(&self, who: AccountId) -> Balance {
        *self.balance.get(&who).unwrap_or(&0)
    }

    pub fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance) -> Result<(), &'static str> {
        let from_balance = self.balance.get(&from).ok_or("From does not have enough funds")?;
        let to_balance = self.balance.get(&to).ok_or("to balance does not exist")?;

        let new_from_balance = from_balance.checked_sub(amount).ok_or("user does not exist")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow")?;

        self.balance.insert(from, new_from_balance);
        self.balance.insert(to, new_to_balance);

        Ok(())

    }
    
}