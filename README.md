# Ternary Statement Macro
Add a ternary statement (like an inline if, but better* looking) to Rust.

(This should not be used seriously, it was only written as an exploration of proc macros).

## Usage
The basic macro is called `si!`.
```rs
    assert_eq!('T', si!(1 + 1 == 2 ? 'T' : 'F'));
```
(remember that both return values must be of the same type or diverge, as in standard if expressions).

Anything that implements `Truthy` can be used as a predicate. By default, only `bool`s, numeric types and enums with `derive(Truthyable)` implement it by default.

Do keep in mind that, if an enum is `derive(Truthable)`, every variant must be tagged with either `truthy` or `falsey`.

```
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

assert_eq!('F', si!(0 ? 'T' : 'F'));

assert_eq!('T', si!(Thingy::SoTrue ? 'T' : 'F'));
assert_eq!('F', si!(Thingy::SoFalse ? 'T' : 'F'));
assert_eq!('F', si!(Thingy::AlsoFalse ? 'T' : 'F'));
assert_eq!('T', si!(Thingy::SikeTrueAgain ? 'T' : 'F'));
```
}
