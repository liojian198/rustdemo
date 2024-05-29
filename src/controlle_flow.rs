#[cfg(test)]
pub mod test {
    #[test]
    fn if_demo () {
        let condition = true;

        let number = if condition {
            5
        } else {
            6
        };
        println!("{number}");
    }

    #[test]
    fn for_demo () {
        for i in 1..=6 {
            println!("{}",i);
        }
    }

    #[test]
    fn while_demo () {
        let mut n = 0;
        while n < 11 {
            println!("{}!", n);
            n = n+1;
        }
        println!("我出来了！！");
    }

}