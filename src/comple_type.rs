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

}