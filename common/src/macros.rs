#[macro_export]
macro_rules! bootstrap {
    ($x:expr) => {
        fn main() {
            use $crate::input_fetcher;

            let input = input_fetcher::fetch_or_exit($x);
            println!("{}", solve(input.as_str()));
        }
    };
}

#[macro_export]
macro_rules! read_main {
    () => {
        fn main() {
            use std::io::Read;

            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input).unwrap();
            println!("{}", solve(input.as_str()));
        }
    };
}
