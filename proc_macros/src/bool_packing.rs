#[inline]
fn mod_8(index: usize) -> usize {
    index & 0x7
}

fn push_bool(num: &mut u8, b: bool, bit_index: usize) {
    let bit_mask = (b as u8) << bit_index;
    *num |= bit_mask;
}

pub fn pack_bool<Bools>(bools: Bools) -> Vec<u8> where Bools: Iterator<Item = bool> {
    let mut packed: Vec<u8> = Vec::new();
    bools.enumerate().for_each(|(index, b)| {
        let bit_index = mod_8(index);
        if bit_index == 0 {
            packed.push(b as u8);
        } else {
            let last = packed.last_mut().unwrap();
            push_bool(last, b, bit_index);
        }
    });
    packed
}