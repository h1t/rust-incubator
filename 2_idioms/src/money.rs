use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct Money(Vec<Coin>);

impl Money {
    pub fn add(&mut self, coin: Coin) {
        self.0.push(coin)
    }

    pub fn extend(&mut self, coins: impl Iterator<Item = Coin>) {
        self.0.extend(coins)
    }

    pub fn from_amount(mut amount: usize) -> Self {
        let nominals = Coin::all_in_descent_order();
        let mut coins = Vec::new();

        // println!("start: {amount} {nominals:?}");

        while amount != 0 {
            for nominal in nominals {
                if let Some(new_amount) = amount.checked_sub(nominal.into()) {
                    coins.push(nominal);
                    amount = new_amount;
                    break;
                    // println!("iter: {amount}");
                }
            }
        }
        // println!("res: {coins:?}");

        Self(coins)
    }

    pub fn sum(&self) -> usize {
        self.0.iter().copied().map(Into::<usize>::into).sum()
    }

    pub fn get_coins(&self) -> Vec<Coin> {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Coin {
    One,
    Two,
    Five,
    Ten,
    Twenty,
    Fifty,
}

impl Coin {
    pub fn all_in_descent_order() -> [Coin; 6] {
        use Coin::*;
        [Fifty, Twenty, Ten, Five, Two, One]
    }
}

impl From<Coin> for usize {
    fn from(value: Coin) -> Self {
        match value {
            Coin::One => 1,
            Coin::Two => 2,
            Coin::Five => 5,
            Coin::Ten => 10,
            Coin::Twenty => 20,
            Coin::Fifty => 50,
        }
    }
}

//NOTE: this is a very naive algorithm
// for a more accurate algorithm you need to implement the *Knapsack problem*
pub(crate) fn get_change(
    machine_coins: &BTreeMap<Coin, usize>,
    new_coins: Money,
    amount: usize,
) -> Option<(BTreeMap<Coin, usize>, Option<Money>)> {
    let mut new_machine_coins = machine_coins.clone();

    for coin in new_coins.0 {
        new_machine_coins
            .entry(coin)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    if amount != 0 {
        let change_coins = Money::from_amount(amount);
        for coin in &change_coins.0 {
            match new_machine_coins.get_mut(coin) {
                Some(count) if *count != 0 => *count -= 1,
                Some(_) | None => return None,
            }
        }
        Some((new_machine_coins, Some(change_coins)))
    } else {
        Some((new_machine_coins, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::money;

    fn get_coins_from_amount(amount: usize) -> Vec<usize> {
        Money::from_amount(amount)
            .0
            .into_iter()
            .map(Into::into)
            .collect()
    }

    #[test]
    fn test_coins_from_amount() {
        assert_eq!(get_coins_from_amount(0), &[]);
        assert_eq!(get_coins_from_amount(1), &[1]);
        assert_eq!(get_coins_from_amount(2), &[2]);
        assert_eq!(get_coins_from_amount(3), &[2, 1]);
        assert_eq!(get_coins_from_amount(4), &[2, 2]);
        assert_eq!(get_coins_from_amount(5), &[5]);
        assert_eq!(get_coins_from_amount(6), &[5, 1]);
        assert_eq!(get_coins_from_amount(7), &[5, 2]);
        assert_eq!(get_coins_from_amount(8), &[5, 2, 1]);
        assert_eq!(get_coins_from_amount(9), &[5, 2, 2]);
        assert_eq!(get_coins_from_amount(10), &[10]);
        assert_eq!(get_coins_from_amount(11), &[10, 1]);
        assert_eq!(get_coins_from_amount(12), &[10, 2]);
        assert_eq!(get_coins_from_amount(20), &[20]);
        assert_eq!(get_coins_from_amount(50), &[50]);
        assert_eq!(get_coins_from_amount(100), &[50, 50]);
        assert_eq!(get_coins_from_amount(138), &[50, 50, 20, 10, 5, 2, 1]);
    }

    fn get_change(coins: &[(Coin, usize)], amount: usize) -> Vec<usize> {
        let coins_map = coins.iter().copied().collect::<BTreeMap<_, _>>();
        let change = money::get_change(&coins_map, Money::default(), amount);
        assert!(change.is_some());
        let money = change.unwrap().1;
        assert!(money.is_some());
        money.unwrap().0.into_iter().map(Into::into).collect()
    }

    fn get_empty_change(coins: &[(Coin, usize)], amount: usize) -> bool {
        let coins_map = coins.iter().copied().collect::<BTreeMap<_, _>>();
        let change = money::get_change(&coins_map, Money::default(), amount);
        change.is_none()
    }

    #[test]
    fn test_get_change() {
        assert_eq!(get_change(&[(Coin::One, 1)], 1), &[1]);
        assert_eq!(get_change(&[(Coin::One, 2)], 1), &[1]);

        let coins = vec![
            (Coin::One, 1),
            (Coin::Two, 1),
            (Coin::Five, 1),
            (Coin::Ten, 1),
            (Coin::Twenty, 1),
            (Coin::Five, 1),
        ];
        assert_eq!(get_change(&coins, 1), &[1]);
        assert_eq!(get_change(&coins, 2), &[2]);
        assert_eq!(get_change(&coins, 3), &[2, 1]);
        assert_eq!(get_change(&coins, 5), &[5]);
        assert_eq!(get_change(&coins, 6), &[5, 1]);
        assert_eq!(get_change(&coins, 7), &[5, 2]);
        assert_eq!(get_change(&coins, 8), &[5, 2, 1]);
        assert_eq!(get_change(&coins, 10), &[10]);
        assert_eq!(get_change(&coins, 11), &[10, 1]);
        assert_eq!(get_change(&coins, 12), &[10, 2]);

        assert!(get_empty_change(&coins, 4));
        assert!(get_empty_change(&coins, 9));
        assert!(get_empty_change(&coins, 14));
        assert!(get_empty_change(&coins, 19));
        assert!(get_empty_change(&coins, 24));
        assert!(get_empty_change(&coins, 29));
    }
}
