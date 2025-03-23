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

#[test]
fn test() {
    assert_eq!('T', si!(2 > 1 ? 'T' : 'F'));
    assert_eq!('F', si!(2 < 1 ? 'T' : 'F'));

    assert_eq!('T', si!(2 ? 'T' : 'F'));
    assert_eq!('F', si!(0 ? 'T' : 'F'));

    assert_eq!('T', si!(Thingy::SoTrue ? 'T' : 'F'));
    assert_eq!('F', si!(Thingy::SoFalse ? 'T' : 'F'));
    assert_eq!('F', si!(Thingy::AlsoFalse ? 'T' : 'F'));
    assert_eq!('T', si!(Thingy::SikeTrueAgain ? 'T' : 'F'));
}
