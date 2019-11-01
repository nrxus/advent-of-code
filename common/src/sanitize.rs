pub fn to_i32s<'a>(input: &'a str) -> impl Iterator<Item = i32> + Clone + 'a {
    input.lines().map(|l| l.parse::<i32>().unwrap())
}
