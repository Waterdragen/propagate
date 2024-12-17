pub trait __GetIndex {
    fn get_index(&self) -> usize;
}

pub trait __GoodIndex: __GetIndex {
    fn good_indexes(&self) -> &'static [u8];
    fn is_good(&self) -> bool {
        get_bit_at(self.good_indexes(), self.get_index())
    }
}

pub trait __BadIndex: __GetIndex {
    fn bad_indexes(&self) -> &'static [u8];
    fn is_bad(&self) -> bool {
        get_bit_at(self.bad_indexes(), self.get_index())
    }
}

fn get_bit_at(bytes: &[u8], index: usize) -> bool {
    // Div-mod by 8
    let byte_index = index >> 3;
    let bit_index = index & 0x7;
    let byte = bytes[byte_index];
    (byte >> bit_index & 1) != 0
}

const GOOD_INDEXES: &[u8] = &[0b01];
const BAD_INDEXES: &[u8] = &[0b10];

macro_rules! impl_index {
    ($ty:ident[$($generics:tt)*],
    $good_variant:ident, $bad_variant:ident,
    $good_method:ident, $bad_method:ident) => {
        impl <$($generics)*> __GetIndex for $ty <$($generics)*> {
            fn get_index(&self) -> usize {
                match self {
                    $good_variant{..} => 0,
                    $bad_variant{..} => 1,
                }
            }
        }
        impl <$($generics)*> __GoodIndex for $ty <$($generics)*> {
            fn good_indexes(&self) -> &'static [u8] { GOOD_INDEXES }
            fn is_good(&self) -> bool { self.$good_method() }
        }
        impl <$($generics)*> __BadIndex for $ty <$($generics)*> {
            fn bad_indexes(&self) -> &'static [u8] { BAD_INDEXES }
            fn is_bad(&self) -> bool { self.$bad_method() }
        }
    };
}

use core::ops::ControlFlow::{self, Continue, Break};

impl_index!(Result[T, E], Ok, Err, is_ok, is_err);
impl_index!(Option[T], Some, None, is_some, is_none);
impl_index!(ControlFlow[B, C], Continue, Break, is_continue, is_break);
