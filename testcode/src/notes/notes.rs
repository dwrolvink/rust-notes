// use std::fmt;

// use std::str::FromStr;

// General
// ============================================================

// struct Accept {
//     module_data_folder: String,
// }
// struct Run {
//     module_data_folder: String,
// }
// enum CliCommand {
//     Accept(Accept),
//     Run(Run),
// }
// impl CliCommand {
//     pub fn get_string(&self) -> &String {
//         match self {
//             CliCommand::Accept(value) => ,
//             CliCommand::Run(value) => value,
//         }
//     }
// }

// struct Config {
//     command: CliCommand,
// }

// impl FromStr for CliCommand {
//     type Err = ();
//     fn from_str(input: &str) -> Result<CliCommand, Self::Err> {
//         match input.to_lowercase().as_str() {
//            "accept"  => Ok(CliCommand::Accept),
//             "run"  => Ok(CliCommand::Run),
//             _      => Err(()),
//         }
//     }
// }

// fn parse_config(args: &[String]) -> Config {
//     let command = args[1].clone();
//     let command = CliCommand::from_str(command.as_str()).unwrap();
//     dbg!(command.get_value());
//     return Config{command: command};
// }

// fn parse_accept_config(args: &[String]) {
//     // expect args[2] == module_data_folder

// }


/*
pub trait Group {
    fn foo(self: &Self){
        println!("Called from group")
    }
}

#[derive (Copy, Debug, Clone, Eq, PartialEq)] // How could I derive Group here as well?
pub struct AGroup<G: Group>(G);

impl<G: Group> AGroup<G> {
    pub fn inner(&self) -> &impl Group {
        &self.0
    }
}

pub struct Test {}
impl Group for Test {}

fn main( ) {
    let a = AGroup( Test {} );
    a.inner().foo();
}
*/