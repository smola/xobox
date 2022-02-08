pub struct Command {
    pub name: &'static str,
    pub run: fn(&[String]) -> i32,
}
