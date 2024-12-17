use crate::{Good, Bad, FromGood, FromBad, ExactlyTwoDistinctVariants};

pub trait TwoStates<G, B>: Good<G> + Bad<B> + FromGood<G> + FromBad<B> + ExactlyTwoDistinctVariants {
    #[inline]
    fn two_states(self) -> Result<G, B> {
        match self.good() {
            Ok(good) => Ok(good),
            Err(self_) => {
                match self_.bad() {
                    Err(bad) => Err(bad),
                    Ok(self_) => two_states_integrity_failed(&self_),
                }
            }
        }
    }
}

impl<T, G, B> TwoStates<G, B> for T where T: Good<G> + Bad<B> + FromGood<G> + FromBad<B> + ExactlyTwoDistinctVariants {}

#[cold]
#[track_caller]
fn two_states_integrity_failed<T>(t: &T) -> ! {
    unreachable!("Encountered a non-binary variant for type {}. This should never happen.", core::any::type_name_of_val(&t))
}
