#[cfg(test)]
mod test {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct Post {
        pub title: String, //标题
        pub author: String, //作者
        pub content: String //内容
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章{}, 作者是{}", self.title, self.author)
        }
    }

    pub struct Weibo {
        pub username: String,
        pub content: String
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了微博{}", self.username, self.content)
        }
    }


    #[test]
    fn trait_demo() {
        let post = Post {title: "Rust语言简介".to_string(), author: "Sunface".to_string(), content:"Rust棒极了!".to_string()};
        let weibo = Weibo{username:"sunface".to_string(), content: "好像微博没Tweet好用".to_string()};
        println!("{}", post.summarize());
        println!("{}", weibo.summarize());
    }


    use std::fmt::Display;

    struct Pair <T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x:T, y:T) -> Self {
            Self {
                x,
                y,
            }
        }
    }

    impl <T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    #[test]
    fn trait_demo1() {
        let pair = Pair::new(11,12);
        pair.cmp_display();
    }


    use std::ops::Add;

    #[derive(Debug)]
    struct Point<T: Add<T, Output = T>> {
        x:T,
        y:T,
    }

    impl <T: Add<T, Output = T>> Add for Point<T>{
        type Output = Point<T>;

        fn add(self, rhs: Self) -> Point<T> {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    fn add<T:Add<T,Output=T>> (a:T,b:T) -> T {
        a + b
    }

    #[test]
    fn trait_demo2() {
        let p1 = Point{x:1.1, y:1.1};
        let p2 = Point{x:2.1, y:2.1};
        println!("{:?}",add(p1,p2));

        let p3 = Point{x:1,y:1};
        let p4 = Point{x:2,y:2};
        println!("{:?}", add(p3,p4));
    }

}