#[derive(Debug)]
struct MySt {
    lol: i32,
    asd: i32,
    kek: String,
    qwe: f32
}


impl MySt {
    fn some(self) {
        println!("{}", self.lol)
    }
}

fn main() {
    let y = {
    };
    let qstruct = MySt {
        lol : 1,
        asd: 0,
        kek: "".to_string(),
        qwe: 0.0,
    };
    let qwe = MySt::some(qstruct);

    print!("{:?}" , y);
}