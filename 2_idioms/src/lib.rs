use money::{Coin, Money};
use product::{Product, ProductName};
use std::{cmp::Ordering, collections::BTreeMap, fmt::Display};

mod money;
mod product;

trait State {}

#[derive(Debug)]
pub struct InitState;

impl State for InitState {}

#[derive(Debug)]
pub struct WaitingForProductSelectState;

impl State for WaitingForProductSelectState {}

#[derive(Debug)]
pub struct WaitingForCoinInsertState {
    product: Product,
    inserted: Money,
}

impl State for WaitingForCoinInsertState {}

#[derive(Debug)]
pub struct VendingMachine<S> {
    products: BTreeMap<Product, usize>,
    coins: BTreeMap<Coin, usize>,
    state: S,
}

pub type WaitingForProductSelectMachine = VendingMachine<WaitingForProductSelectState>;
pub type WaitingForCoinInsertMachine = VendingMachine<WaitingForCoinInsertState>;
pub type InitStateMachine = VendingMachine<InitState>;

pub type Purchase = (WaitingForProductSelectMachine, Product, Option<Money>);

impl Default for VendingMachine<InitState> {
    fn default() -> Self {
        Self {
            products: Default::default(),
            coins: Default::default(),
            state: InitState,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    NotEnoughProduct(WaitingForProductSelectMachine, ProductName),
    NotEnoughMoneyForProduct(WaitingForCoinInsertMachine, ProductName, usize),
    NotEnoughMoneyForChange(WaitingForProductSelectMachine, usize),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotEnoughProduct(_, name) => {
                write!(f, "there is no such {name} product anymore")
            }
            Error::NotEnoughMoneyForProduct(_, name, amount) => {
                write!(f, "you have to add {amount} money to buy {name} product",)
            }
            Error::NotEnoughMoneyForChange(_, amount) => {
                write!(f, "there is not enough money to give change from {amount}")
            }
        }
    }
}

impl std::error::Error for Error {}

impl VendingMachine<InitState> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_product(&mut self, product: Product) -> &mut Self {
        self.products
            .entry(product)
            .and_modify(|amount| *amount += 1)
            .or_insert(1);
        self
    }

    pub fn add_products(&mut self, products: impl IntoIterator<Item = Product>) -> &mut Self {
        for product in products {
            self.add_product(product);
        }
        self
    }

    pub fn add_coin(&mut self, coin: Coin) -> &mut Self {
        self.coins
            .entry(coin)
            .and_modify(|amount| *amount += 1)
            .or_insert(1);
        self
    }

    pub fn add_coins(&mut self, coins: impl IntoIterator<Item = Coin>) -> &mut Self {
        for coin in coins {
            self.add_coin(coin);
        }
        self
    }

    pub fn start(self) -> VendingMachine<WaitingForProductSelectState> {
        VendingMachine {
            products: self.products,
            coins: self.coins,
            state: WaitingForProductSelectState,
        }
    }
}

impl VendingMachine<WaitingForProductSelectState> {
    pub fn select(self, name: ProductName) -> Result<WaitingForCoinInsertMachine, Error> {
        match self.products.get_key_value(&name) {
            Some((_, &0)) => Err(Error::NotEnoughProduct(self, name)),
            Some((&product, _)) => Ok(self.next_state(product)),
            //TODO
            None => unreachable!(),
        }
    }

    fn next_state(self, product: Product) -> WaitingForCoinInsertMachine {
        VendingMachine {
            products: self.products,
            coins: self.coins,
            state: WaitingForCoinInsertState {
                product,
                inserted: Default::default(),
            },
        }
    }
}

impl VendingMachine<WaitingForCoinInsertState> {
    pub fn insert_coin(&mut self, coin: Coin) {
        self.state.inserted.add(coin)
    }

    pub fn insert_coins(&mut self, coins: impl IntoIterator<Item = Coin>) {
        self.state.inserted.extend(coins.into_iter())
    }

    pub fn cancel(self) -> WaitingForProductSelectMachine {
        self.next_state()
    }

    pub fn buy(mut self) -> Result<Purchase, Error> {
        let inserted = self.state.inserted.sum();
        let price = self.state.product.price();
        let product = self.state.product;

        match inserted.cmp(&price) {
            Ordering::Less => Err(Error::NotEnoughMoneyForProduct(
                self,
                product.name(),
                price - inserted,
            )),
            Ordering::Greater | Ordering::Equal => {
                let change = inserted - price;
                let inserted_money = std::mem::take(&mut self.state.inserted);
                match money::get_change(&self.coins, inserted_money, change) {
                    Some((new_machine_coins, change)) => {
                        self.coins = new_machine_coins;
                        self.take_off_product(product.name());
                        Ok((self.next_state(), product, change))
                    }
                    None => Err(Error::NotEnoughMoneyForChange(self.next_state(), change)),
                }
            }
        }
    }

    fn next_state(self) -> WaitingForProductSelectMachine {
        VendingMachine {
            products: self.products,
            coins: self.coins,
            state: WaitingForProductSelectState,
        }
    }

    fn take_off_product(&mut self, name: ProductName) {
        if let Some(count) = self.products.get_mut(&name) {
            *count -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_machine(
        product: Product,
        products: &[Product],
        coin: Coin,
        coins: &[Coin],
    ) -> WaitingForProductSelectMachine {
        let mut machine = VendingMachine::default();

        machine.add_product(product);
        machine.add_products(products.iter().cloned());

        machine.add_coin(coin);
        machine.add_coins(coins.iter().cloned());

        check_machine_state(
            &machine.products,
            &machine.coins,
            &[(ProductName::Water, 1), (ProductName::Soda, 1)],
            &[
                (Coin::One, 1),
                (Coin::Two, 1),
                (Coin::Five, 1),
                (Coin::Ten, 1),
                (Coin::Twenty, 1),
                (Coin::Fifty, 1),
            ],
        );

        machine.start()
    }

    fn select_product(
        machine: WaitingForProductSelectMachine,
        name: ProductName,
        machine_products: &[(ProductName, usize)],
        machine_coins: &[(Coin, usize)],
    ) -> WaitingForCoinInsertMachine {
        let res = machine.select(name);

        assert!(res.is_ok());
        let machine = res.unwrap();

        check_machine_state(
            &machine.products,
            &machine.coins,
            machine_products,
            machine_coins,
        );
        machine
    }

    fn select_not_enough_product(
        machine: WaitingForProductSelectMachine,
        name: ProductName,
        machine_products: &[(ProductName, usize)],
        machine_coins: &[(Coin, usize)],
    ) -> WaitingForProductSelectMachine {
        let res = machine.select(name);
        let res = if let Err(Error::NotEnoughProduct(machine, product_name)) = res {
            Some((machine, product_name))
        } else {
            None
        };
        assert!(res.is_some());
        let (machine, product_name) = res.unwrap();
        assert_eq!(name, product_name);

        check_machine_state(
            &machine.products,
            &machine.coins,
            machine_products,
            machine_coins,
        );
        machine
    }

    fn bye_not_enough_money(
        mut machine: WaitingForCoinInsertMachine,
        coins: &[Coin],
        amount: usize,
        machine_products: &[(ProductName, usize)],
        machine_coins: &[(Coin, usize)],
    ) -> WaitingForCoinInsertMachine {
        machine.insert_coins(coins.iter().cloned());

        let res = machine.buy();
        let res =
            if let Err(Error::NotEnoughMoneyForProduct(machine, ProductName::Water, money)) = res {
                Some((machine, money))
            } else {
                None
            };
        assert!(res.is_some());
        let (machine, money_amount) = res.unwrap();

        assert_eq!(money_amount, amount);
        check_machine_state(
            &machine.products,
            &machine.coins,
            machine_products,
            machine_coins,
        );

        machine
    }

    fn bye_not_enough_change(
        mut machine: WaitingForCoinInsertMachine,
        coins: &[Coin],
        amount: usize,
        machine_products: &[(ProductName, usize)],
        machine_coins: &[(Coin, usize)],
    ) -> WaitingForProductSelectMachine {
        machine.insert_coins(coins.iter().cloned());

        let res = machine.buy();
        let res = if let Err(Error::NotEnoughMoneyForChange(machine, money)) = res {
            Some((machine, money))
        } else {
            None
        };
        assert!(res.is_some());
        let (machine, money_amount) = res.unwrap();

        assert_eq!(money_amount, amount);
        check_machine_state(
            &machine.products,
            &machine.coins,
            machine_products,
            machine_coins,
        );

        machine
    }

    fn cancel_coin_inserting(
        mut machine: WaitingForCoinInsertMachine,
        coins: &[Coin],
        machine_products: &[(ProductName, usize)],
        machine_coins: &[(Coin, usize)],
    ) -> WaitingForProductSelectMachine {
        machine.insert_coins(coins.iter().cloned());

        let machine = machine.cancel();

        check_machine_state(
            &machine.products,
            &machine.coins,
            machine_products,
            machine_coins,
        );

        machine
    }

    fn bye(
        mut machine: WaitingForCoinInsertMachine,
        product_name: ProductName,
        insertd_coins: &[Coin],
        change: &[usize],
        machine_products: &[(ProductName, usize)],
        machine_coins: &[(Coin, usize)],
    ) -> WaitingForProductSelectMachine {
        machine.insert_coins(insertd_coins.iter().cloned());

        let res = machine.buy();
        assert!(res.is_ok());
        let (machine, product, money) = res.unwrap();
        assert_eq!(product.name(), product_name);
        assert!(money.is_some());
        let money = money.unwrap();
        check_money(money, change);
        check_machine_state(
            &machine.products,
            &machine.coins,
            machine_products,
            machine_coins,
        );

        machine
    }

    #[test]
    fn test_vending_machine() {
        let machine = init_machine(
            Product::new(ProductName::Water, 10),
            &[Product::new(ProductName::Soda, 11)],
            Coin::One,
            &[Coin::Two, Coin::Five, Coin::Ten, Coin::Twenty, Coin::Fifty],
        );

        let machine = select_product(
            machine,
            ProductName::Water,
            &[(ProductName::Water, 1), (ProductName::Soda, 1)],
            &[
                (Coin::One, 1),
                (Coin::Two, 1),
                (Coin::Five, 1),
                (Coin::Ten, 1),
                (Coin::Twenty, 1),
                (Coin::Fifty, 1),
            ],
        );

        let machine = bye_not_enough_money(
            machine,
            &[Coin::One, Coin::One, Coin::Two],
            6,
            &[(ProductName::Water, 1), (ProductName::Soda, 1)],
            &[
                (Coin::One, 1),
                (Coin::Two, 1),
                (Coin::Five, 1),
                (Coin::Ten, 1),
                (Coin::Twenty, 1),
                (Coin::Fifty, 1),
            ],
        );

        let machine = bye(
            machine,
            ProductName::Water,
            &[Coin::Ten],
            &[2, 2],
            &[(ProductName::Water, 0), (ProductName::Soda, 1)],
            &[
                (Coin::One, 3),
                (Coin::Two, 0),
                (Coin::Five, 1),
                (Coin::Ten, 2),
                (Coin::Twenty, 1),
                (Coin::Fifty, 1),
            ],
        );

        let machine = select_not_enough_product(
            machine,
            ProductName::Water,
            &[(ProductName::Water, 0), (ProductName::Soda, 1)],
            &[
                (Coin::One, 3),
                (Coin::Two, 0),
                (Coin::Five, 1),
                (Coin::Ten, 2),
                (Coin::Twenty, 1),
                (Coin::Fifty, 1),
            ],
        );

        let machine = select_product(
            machine,
            ProductName::Soda,
            &[(ProductName::Water, 0), (ProductName::Soda, 1)],
            &[
                (Coin::One, 3),
                (Coin::Two, 0),
                (Coin::Five, 1),
                (Coin::Ten, 2),
                (Coin::Twenty, 1),
                (Coin::Fifty, 1),
            ],
        );

        let machine = bye_not_enough_change(
            machine,
            &[Coin::Fifty],
            39,
            &[(ProductName::Water, 0), (ProductName::Soda, 1)],
            &[
                (Coin::One, 3),
                (Coin::Two, 0),
                (Coin::Five, 1),
                (Coin::Ten, 2),
                (Coin::Twenty, 1),
                (Coin::Fifty, 1),
            ],
        );

        let machine = select_product(
            machine,
            ProductName::Soda,
            &[(ProductName::Water, 0), (ProductName::Soda, 1)],
            &[
                (Coin::One, 3),
                (Coin::Two, 0),
                (Coin::Five, 1),
                (Coin::Ten, 2),
                (Coin::Twenty, 1),
                (Coin::Fifty, 1),
            ],
        );

        let _ = cancel_coin_inserting(
            machine,
            &[Coin::One],
            &[(ProductName::Water, 0), (ProductName::Soda, 1)],
            &[
                (Coin::One, 3),
                (Coin::Two, 0),
                (Coin::Five, 1),
                (Coin::Ten, 2),
                (Coin::Twenty, 1),
                (Coin::Fifty, 1),
            ],
        );
    }

    fn check_money(money: Money, etalon: &[usize]) {
        assert_eq!(
            money
                .get_coins()
                .into_iter()
                .map(usize::from)
                .collect::<Vec<_>>(),
            etalon
        );
    }

    fn check_machine_state(
        machine_products: &BTreeMap<Product, usize>,
        machine_coins: &BTreeMap<Coin, usize>,
        products: &[(ProductName, usize)],
        coins: &[(Coin, usize)],
    ) {
        assert_eq!(
            machine_products
                .clone()
                .into_iter()
                .map(|(product, amount)| (product.name(), amount))
                .collect::<Vec<_>>(),
            products
        );
        assert_eq!(machine_coins.clone().into_iter().collect::<Vec<_>>(), coins);
    }
}
