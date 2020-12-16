#[macro_export]
macro_rules! read_main {
    () => {
        fn main() {
            use std::io::Read;

            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input).unwrap();
            let input = input.as_str();
            let start = std::time::Instant::now();
            let solution = solve(input);
            let elapsed = start.elapsed();
            let seconds = elapsed.as_secs();
            let millis = elapsed.subsec_millis();
            let micros = elapsed.subsec_micros() - millis * 1000;

            println!(
                "{}\n\nfinished in: {seconds}s {ms}.{us}ms",
                solution,
                seconds = seconds,
                ms = millis,
                us = micros
            );
        }
    };
}
