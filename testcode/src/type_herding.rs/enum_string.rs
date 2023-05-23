/*
    I have two String newtypes, that I want to use in some functions interchangeably.
    Both as input and as output.
    There are supposedly multiple ways to do this (traits?), but this is so far the only
    that I got working.

    First, define the newtypes, and implement a way to get the wrapped String:
*/ 

pub struct Cake(pub String);
impl Cake {
    pub fn to_string(&self) -> String {
        return self.0.clone();
    }
}

pub struct IceCream(pub String);
impl IceCream {
    pub fn to_string(&self) -> String {
        return self.0.clone();
    }
}

/*
    Now, to herd these into one type, we create an enum that contains both.
    Also implement a way to get the inner String.
    (Note that the match down here is the only way to acces the inner type)
*/

pub enum Snack {
    Cake(Cake),
    IceCream(IceCream),
}
impl Snack {
    pub fn to_string(&self) -> String {
        return match self {
            Snack::Cake(value) => value.to_string(),
            Snack::IceCream(value) => value.to_string(),
        }
    }
}

/*
    Now we can create a function that outputs type Snack, 
    and one that has type Snack as input:
*/

pub fn request_snack(snack_name: &str) -> Option<Snack> {
    match snack_name {
        "blackforest cake" => Some(Snack::Cake(Cake(snack_name.to_string()))),
        "lemon ice cream" => Some(Snack:IceCream(IceCream(snack_name.to_string()))),
        _ => None
    };
}

pub fn eat_snack(snack: Snack) {
    let snack_name = snack.to_string();
    println!("mmhhhmm, this {} is tasty", snack_name.as_str());
}

fn main() {
    let snack = request_snack("blackforest cake");
    let snack = match snack {
        Some(snack) => snack,
        None => {
            println!("I don't have that snack");
            std::process::exit();
        }
    }
    println!("eating {}", snack.to_string());
}