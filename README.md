# This is a personal Blockchain project
## Features
### Primary language 
__Rust__ [![Rust logo](https://th.bing.com/th/id/OIP.pnTN1j0W6CEtmtji83uENQHaE8?rs=1&pid=ImgDetMain/150)]

## Scalability Features
### Block Structure (support.rs)
__A Block structure that holds multiple extrinsics (transactions)__

~~~~bash
pub struct Block<Header, Extrinsic> {
    pub header: Header,
    pub extrinsics: Vec<Extrinsic>,
}
~~~~

### Efficient Data Management (BTreeMap)
__BTreeMap for storing claims and balances ensures that My data structures are efficiently managed, providing quicker access and updates. O(log n) complexity for inserts, deletions, and lookups, which scales__

| Notation   | Complexity   |
|------------|--------------|
| O(1)       | Constant     |
| O(N * log N) | Log linear  |
| O(N^2)     | Quadratic    |
| O(N^3)     | Cubic        |
| O(2^N)     | Exponential  |


[![Statistic](https://media.geeksforgeeks.org/wp-content/cdn-uploads/20220812122843/Logarithmic-time-complexity-blog-1.jpg)](https://media.geeksforgeeks.org/wp-content/cdn-uploads/20220812122843/Logarithmic-time-complexity-blog-1.jpg)



~~~~bash
#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}
~~~~

### Modular Design:
- mod balances;
- mod proof_of_existence;
- mod support;
- mod system;
__Each module can be optimized and scaled independently.__

## Ensuring Data Integrity and Immutability:

### Blockchain's Immutable Ledger:
__Blockchain technology ensures that once data is written, it cannot be altered__

### Block Structure (support.rs):

~~~~bash
pub struct Block<Header, Extrinsic> {
    pub header: Header,
    pub extrinsics: Vec<Extrinsic>,
}~~~~


__Each block's header contains essential information (like block_number), and the block itself holds multiple transactions (extrinsics)__

### Proof of Existence (proof_of_existence.rs):

~~~~bash
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}

~~~~
__Storing claims in a BTreeMap and linking them to account IDs, you ensure that each claim is uniquely associated with its creator__

### System Module (system.rs):


~~~~bash
#[derive(Debug)]
pub struct SystemPallet<T: Config> {
    pub block_number: T::BlockNumber,
    pub nonce: BTreeMap<T::AccountId, T::Nonce>,
}~~~~


__System module tracks the block number and manages nonces for each account, ensuring that each transaction is unique and ordered, which helps prevent double-spending and maintains data integrity__

### Transaction Validation:


~~~~bash 
pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
    match self.get_claim(&claim) {
        Some(_) => Err("claim already exists"),
        None => {
            self.claims.insert(claim, caller);
            Ok(())
        }
    }
}
~~~~

__create_claim function checks if the claim already exists__

## User Account Management in My Code:

### Account ID Management (types.rs):

pub type AccountId = String;


### Balances Module (balances.rs):

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

`
__The balances module stores account balances in a BTreeMap__


~~~bash
pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
    self.balances.insert(who.clone(), amount);
}~~


__set_balance function allows updating the balance of a specific account.__


~~~~bash
pub fn get_balance(&self, who: &T::AccountId) -> T::Balance {
    *self.balances.get(who).unwrap_or(&T::Balance::zero())
}~~~~


__get_balance function retrieves the balance of a specific account, enabling the system to track account balances accurately__

### System Module (system.rs):

~~~~bash
pub struct SystemPallet<T: Config> {
    pub nonce: BTreeMap<T::AccountId, T::Nonce>,
}
~~~~


__The nonce map in the SystemPallet keeps track of the number of transactions sent by each account__


~~~~bash
pub fn inc_nonce(&mut self, who: &T::AccountId) {
    let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
    self.nonce.insert(who.clone(), nonce + T::Nonce::one());
}
~~~~

__inc_nonce function increments the nonce for a specific account, ensuring that each transaction is processed uniquely and sequentially__


## Installation
~~~bash
git clone https://github.com/user/project.git
cd project
npm install
~~~
:rocket:
:tada: 
:bug:
:memo: 
:sparkles:
