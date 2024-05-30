#[cfg(test)]
mod test {
    #[test]
    fn closure_demo() {
        let x = 1;
        let sum = |y| x  + y;
        assert_eq!(3, sum(2));
    }

    //传统函数实现

    use std::thread;
    use std::time::Duration;
    #[test]
    fn closure_demo1 () {
        let intensity = 10;
        let random_number = 7;
        workout(intensity, random_number);
    }

    fn release_time(intensity: u32) -> u32 {
        println!("休息时间");
        thread::sleep(Duration::from_secs(2));
        intensity
    }

    fn workout(intensity: u32, random_number: u32) {
        if intensity < 25 {
            println!("今天活力满满，先做{}个俯卧撑！", release_time(intensity));

        } else if random_number == 3 {
            println!("昨天练过度了，今天还是休息下吧！");
        } else {
            println!(
                "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
                release_time(intensity)
            );
        }
    }



    //闭包实现
    #[test]
    fn closure_demo2() {
        let intensity = 10;
        let random_number = 7;
        workout1(intensity, random_number);
    }

    fn workout1 (intensity: u32, random_number: u32) {
        let action = || -> u32{
            println!("休息时间");
            thread::sleep(Duration::from_secs(2));
            intensity
        };
        if intensity < 25 {
            println!("今天活力满满，先做{}个俯卧撑！", action());
        } else if random_number == 3 {
            println!("昨天练过度了，今天还是休息下吧！");
        } else {
            println!(
                "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
                action()
            );
        }
    }

}