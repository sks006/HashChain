use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}
#[macros::call]
impl<T: Config> Pallet<T> {
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        match self.get_claim(&claim) {
            Some(_) => Err("claim already exists"),
            None => {
                self.claims.insert(claim, caller);
                Ok(())
            }
        }
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let claim_owner = self.get_claim(&claim).ok_or("claim does not exist")?;
        if claim_owner != &caller {
            return Err("caller is not the owner of the claim");
        }
        self.claims.remove(&claim);
        Ok(())
    }
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }
}

#[cfg(test)]
mod test {
    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut poe = super::Pallet::<TestConfig>::new();

        // Create a claim
        let _ = poe.create_claim("fehim", "my_doc");
        assert_eq!(poe.get_claim(&"my_doc"), Some(&"fehim"));

        // Try to revoke the claim with a different caller
        let res = poe.revoke_claim("shihab", "my_doc");
        assert_eq!(res, Err("caller is not the owner of the claim"));

        // Revoke the claim with the correct caller
        let res = poe.revoke_claim("fehim", "my_doc");
        assert_eq!(res, Ok(()));

        // Try to revoke the claim again (should fail)
        let res = poe.revoke_claim("fehim", "my_doc");
        assert_eq!(res, Err("claim does not exist"));
    }
}
