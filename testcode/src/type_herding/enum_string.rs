/*
    I have two String newtypes, that I want to use in some functions interchangeably.
    Both as input and as output.
    There are mulitple ways to do this, but they are all a bit lacking.

    Let's first look at the enum way (1)

    First, define the newtypes:
*/ 

pub struct Cake(pub String);
pub struct IceCream(pub String);

/* 
    To define the way to get the wrapped string, we could just write 
    `impl Cake { fn to_string(...){...} }`
    But we use a trait here, to show an alternative to using enums later on (2)
*/

pub trait Snackable {
    fn to_string(&self) -> String;
}
impl Snackable for Cake {
    fn to_string(&self) -> String {
        return self.0.clone();
    }
}
impl Snackable for IceCream {
    fn to_string(&self) -> String {
        return self.0.clone();
    }
}

/*
    Now, to herd these into one type, we create an enum that contains both.
    Also implement a way to get the inner String.
    (Note that the match down here is the only way to access the inner type)
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

    // To get the wrapped value without an inline match, we can prepare
    // a couple of functions to do this for us.
    pub fn get_cake_ref(&self) -> Option<&Cake> {
        return match self {
            Snack::Cake(value) => Some(value),
            Snack::IceCream(value) => None,
        }
    }
    pub fn get_icecream_ref(&self) -> Option<&IceCream> {
        return match self {
            Snack::Cake(value) => None,
            Snack::IceCream(value) => Some(value),
        }
    }
}

/*
    Now we can create a function that outputs type Snack, 
    and one that has type Snack as input.
*/

pub fn request_snack(snack_name: &str) -> Option<Snack> {
    return match snack_name {
        "blackforest cake" => Some(
            Snack::Cake(
                Cake(snack_name.to_string())
            )),
        "lemon ice cream" => Some(Snack::IceCream(IceCream(snack_name.to_string()))),
        _ => None
    };
}

pub fn eat_snack(snack: Snack) {
    let snack_name = snack.to_string();
    println!("mmhhhmm, this {} is tasty", snack_name.as_str());
}

/* 
    Same as eat_snack(), but using traits as common input
*/
pub fn eat_snack_by_trait(snack: impl Snackable) {
    let snack_name = snack.to_string();
    println!("mmhhhmm, this {} is tasty", snack_name.as_str());
}

fn main() {
    /* 
        (1) 
        Let's use the enum to have the function create a Cake/IceCream 
        based on the string value:
    */
    // get snack object
    let snack = request_snack("blackforest cake");

    // test whether we got an actual snack back
    let snack = match snack {
        Some(snack) => snack,
        None => {
            println!("I don't have that snack");
            std::process::exit(0);
        }
    };

    // consoom
    eat_snack(snack);

    /* 
        (2)
        We can also use the newtypes directly, and have functions accept multiple
        types, by referring to the trait they share:
    */
    // we can also pass multiple types to a functon by trait
    let snack2 = IceCream("strawberry ice cream".to_string());
    eat_snack_by_trait(snack2);

    /*
        Note that traits do not offer a solution to having functions *return* 
        multiple values, only enums do that.

        (3)
        But how to get the original cake back when we use an enum?
        (Let's say we want a function to instantiate the correct type for us, and then 
        use it).
    */

    let snack3 = request_snack("lemon ice cream").unwrap();
    let icecream = snack3.get_icecream_ref().unwrap();
    println!("{}", icecream.to_string());

}

/*
    Further notes:
    
    Issue for both methods is getting the original newtype (not the string)
    after wrapping it in the enum, as any general function can only return a single type.

    We must use the match block in the caller, or implement a separate 
    function for each type, as we did with the .get_icecream_ref().
    (This could be implemented with a macro).

    All pretty clunky, better options should be looked for.
*/