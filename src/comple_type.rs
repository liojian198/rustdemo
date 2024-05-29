#[cfg(test)]
pub mod tests{

    //字符串
    #[test]
    pub fn chars_demo() {
        let mut s = String::from("hello world!!!");
        s.push('s');
        greet(&s);
        s.push('d');
        greet(&s);
    }

    fn greet(name:&str) {
        println!("hello, {}", name);
    }

    #[test]
    pub fn tup_demo1() {
        let tup:(i32,f64,u8) = (500,5.2,1);
        let (x,y,z) = tup;
        println!("y is {}",y);
    }

    #[test]
    pub fn tup_demo2() {
        let tup:(i32,f64,u8) = (500,5.2,1);
        let x = tup.0;
        let y = tup.1;
        let z = tup.2;
        println!("{}, {}, {} {x}, {y}, {z}",tup.0,tup.1,tup.2);

    }

    #[test]
    pub fn tup_demo3 () {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("the length of '{}' is {}.",s2, len);
    }

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len();
        (s, length)
    }


    struct User {
        active: bool,
        user_name: String,
        email: String,
        sign_in_count: u64,
    }

    #[test]
    pub fn struct_demo1() {
        let user1 = User {
            email: String::from("someone@qq.com"),
            user_name: String::from("zhangsan"),
            active: true,
            sign_in_count: 1
        };

        let user2 = User {
            active: false,
            ..user1
        };
        println!("{}", user1.active);
        println!("{}",user2.active);

    }

    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);

    #[test]
    pub fn struct_demo2() {
        let black = Color(0,0,0);
        let origin = Point(0,0,0);
    }

    #[test]
    pub fn enum_demo() {
        let heart = PokerSuit::Hearts;
        let diamonds = PokerSuit::Diamonds;
        print_suit(heart);
        print_suit(diamonds);
    }

    fn print_suit(card: PokerSuit) {
        println!("{:?}", card);
    }

    #[derive(Debug)]
    enum PokerSuit {
        Clubs,
        Spades,
        Diamonds,
        Hearts,
    }

}