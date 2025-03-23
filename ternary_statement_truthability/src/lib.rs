pub trait Truthy {
    fn to_bool(self) -> bool;
}

macro_rules! truthy_num {
    ($($t:ty)*) => {
        $(impl Truthy for $t {
            fn to_bool(self) -> bool {
                self != 0
            }
        })*
    };
}

truthy_num!(u8 u16 u32 u64 i8 i16 i32 i64);
