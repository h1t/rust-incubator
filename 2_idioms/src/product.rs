use std::{borrow::Borrow, fmt::Display};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ProductName {
    Water,
    Juice,
    Soda,
}

pub type ProductPrice = usize;

impl Display for ProductName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductName::Water => write!(f, "Water"),
            ProductName::Juice => write!(f, "Juice"),
            ProductName::Soda => write!(f, "Soda"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Product {
    name: ProductName,
    price: ProductPrice,
}

impl Product {
    pub fn new(name: ProductName, price: ProductPrice) -> Self {
        Self { name, price }
    }

    pub fn price(&self) -> ProductPrice {
        self.price
    }

    pub fn name(&self) -> ProductName {
        self.name
    }
}

impl Borrow<ProductName> for Product {
    fn borrow(&self) -> &ProductName {
        &self.name
    }
}
