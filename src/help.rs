use super::cmd;

pub fn run() -> i32 {
    println!("xobox");
    println!();
    println!("Functions:");
    for c in cmd::COMMANDS {
        println!("\t{}", c.name);
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_help() {
        assert_eq!(run(), 0 as i32)
    }
}
