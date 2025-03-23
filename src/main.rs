use ternary_statement_macro::{si, Truthyable};
use ternary_statement_truthability::Truthy;

#[derive(Truthyable)]
enum Thingy {
    #[truthy]
    SoTrue,
    #[falsey]
    SoFalse,
    #[falsey]
    AlsoFalse,
    #[truthy]
    SikeTrueAgain,
}

fn main() {
    println!("0: {}", si!(0 ? 'T' : 'F'));
    println!("1: {}", si!(2 ? 'T' : 'F'));

    println!("2: {}", si!(Thingy::SoTrue ? 'T' : 'F'));
    println!("3: {}", si!(Thingy::SoFalse ? 'T' : 'F'));
    println!("4: {}", si!(Thingy::AlsoFalse ? 'T' : 'F'));
    println!("5: {}", si!(Thingy::SikeTrueAgain ? 'T' : 'F'));
}
