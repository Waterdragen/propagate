#[doc(hidden)]
pub trait __GoodIndex {
    fn __good_indexes(&self) -> &'static [u8];
    fn __get_index(&self) -> usize;
    fn __is_good(&self) -> bool {
        get_bit_at(self.__good_indexes(), self.__get_index())
    }
}

#[doc(hidden)]
pub trait __BadIndex {
    fn __bad_indexes(&self) -> &'static [u8];
    fn __get_index(&self) -> usize;
    fn __is_bad(&self) -> bool {
        get_bit_at(self.__bad_indexes(), self.__get_index())
    }
}

fn get_bit_at(bytes: &[u8], index: usize) -> bool {
    // Div-mod by 8
    let byte_index = index >> 3;
    let bit_index = index & 0x7;
    let byte = bytes[byte_index];
    (byte >> bit_index & 1) != 0
}

impl<T, E> __GoodIndex for Result<T, E> {
    fn __good_indexes(&self) -> &'static [u8] { unimplemented!() }
    fn __get_index(&self) -> usize { unimplemented!() }
    fn __is_good(&self) -> bool { self.is_ok() }
}

impl<T> __GoodIndex for Option<T> {
    fn __good_indexes(&self) -> &'static [u8] { unimplemented!() }
    fn __get_index(&self) -> usize { unimplemented!() }
    fn __is_good(&self) -> bool { self.is_some() }
}

impl<T, E> __BadIndex for Result<T, E> {
    fn __bad_indexes(&self) -> &'static [u8] { unimplemented!() }
    fn __get_index(&self) -> usize { unimplemented!() }
    fn __is_bad(&self) -> bool { self.is_err() }
}

impl<T> __BadIndex for Option<T> {
    fn __bad_indexes(&self) -> &'static [u8] { unimplemented!() }
    fn __get_index(&self) -> usize { unimplemented!() }
    fn __is_bad(&self) -> bool { self.is_none() }
}
