// Rexports
pub use ternary_statement_macro::{si, Truthyable};
pub use ternary_statement_truthability::Truthy;

#[test]
fn test() {
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
    assert_eq!('T', si!(2 > 1 ? 'T' : 'F'));
    assert_eq!('F', si!(2 < 1 ? 'T' : 'F'));

    assert_eq!('T', si!(2 ? 'T' : 'F'));
    assert_eq!('F', si!(0 ? 'T' : 'F'));

    assert_eq!('T', si!(Thingy::SoTrue ? 'T' : 'F'));
    assert_eq!('F', si!(Thingy::SoFalse ? 'T' : 'F'));
    assert_eq!('F', si!(Thingy::AlsoFalse ? 'T' : 'F'));
    assert_eq!('T', si!(Thingy::SikeTrueAgain ? 'T' : 'F'));
}
