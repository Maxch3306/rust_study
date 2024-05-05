use restaurant::eat_at_restaurant;
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String)
}

impl Message {
    fn oh(&self) {
        match self {
            Message::Quit=>{
                println!("ggggg")
            }
            Message::Move { x, y }=>{
                println!("move{x},y:{y}")
            }
            Message::Write(string)=>{
                println!("string: {string}")
            }
        }
        
    }
}


fn main() {
    
    //dbg!(res);
    eat_at_restaurant();
    let a = Message::Move { x: (22131), y: (8888) };
    let b = Message::Quit;
    let c = Message::Write("sad".to_string());
    b.oh();
    a.oh();
    c.oh();
    //let mut s1 = String::from("abc");
    //let r1 = &mut s1;
    //r1.push_str("string");
    //s1.push_str("123");
    //let r2 = &mut s1;
    //create a value x:string



    //print!("s1:{s1}");
    //print!("r1:{r1}");
}

