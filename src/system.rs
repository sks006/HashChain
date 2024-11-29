use num::traits::{One, Zero};
use std::collections::BTreeMap;
pub use SystemPallet as Pallet;

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + Clone + Copy + std::ops::AddAssign;
    type Nonce: Zero + One + Clone + Copy;
}

#[derive(Debug)]
pub struct SystemPallet<T: Config> {
    pub block_number: T::BlockNumber,
    pub nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> SystemPallet<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        self.nonce.insert(who.clone(), nonce + T::Nonce::one());
    }

    pub fn get_nonce(&self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }
}

#[derive(Debug)]
pub enum Call {
    IncBlockNumber,
}

impl<T: Config> super::support::Dispatch for SystemPallet<T> {
    type Caller = T::AccountId;
    type Call = Call;

    fn dispatch(
        &mut self,
        _caller: Self::Caller,
        call: Self::Call,
    ) -> super::support::DispatchResult {
        match call {
            Call::IncBlockNumber => {
                self.inc_block_number();
                Ok(())
            }
        }
    }
}
