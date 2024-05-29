#[cfg(test)]
mod test {

    enum Direction {
        East,
        West,
        North,
        South,
    }

    #[test]
    fn match_demo() {
        let dire = Direction::South;
        match dire {
            Direction::East => println!("East"),
            Direction::North | Direction::South => {
                println!("South or North");
            },
            _ => println!("West"),
        };
    }


    enum Action {
        Say(String),
        MoveTo(i32,i32),
        ChangeColorRGB(u16,u16,u16),
    }

    #[test]
    fn match_demo1 () {
        let actions = [Action::Say("hello rust".to_string()), Action::MoveTo(1,2), Action::ChangeColorRGB(255,255,0)];
        for action in actions {
            match action {
                Action::Say(s) => {println!("{}",s);},
                Action::MoveTo(x,y) => {
                    println!("point from (0,0) move to ({},{})",x,y);
                },
                Action::ChangeColorRGB(r,g,_) => {
                    println!("change color into '(r:{},g:{},b:0)', b has bee ignored", r,g);
                }
            }
        }
    }
}