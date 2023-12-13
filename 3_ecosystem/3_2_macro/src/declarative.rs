#[macro_export]
macro_rules! btreemap {
    ($($key:expr => $value:expr),+ $(,)?) => {
        std::collections::BTreeMap::from([
            $( ($key, $value) ),+
        ])
    };
}

pub use btreemap;
