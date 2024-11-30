# This is a personal Blockchain project
## Features
### Primary language 
 __Rust__    <br><a href="https://doc.rust-lang.org/std/primitive.str.html">
  <img src="https://th.bing.com/th/id/OIP.pnTN1j0W6CEtmtji83uENQHaE8?rs=1&pid=ImgDetMain/50" alt="Rust logo" width="50" />
</a>


## Scalability Features
### Block Structure (support.rs)
 A Block structure that holds multiple extrinsics (transactions) 

```
pub struct Block<Header, Extrinsic> {
    pub header: Header,
    pub extrinsics: Vec<Extrinsic>,
}
```

### Efficient Data Management (BTreeMap)

 BTreeMap for storing claims and balances ensures that My data structures are efficiently managed, providing quicker access and updates. O(log n) complexity for inserts, deletions, and lookups, which scales 

| Notation   | Complexity   |
|------------|--------------|
| O(1)       | Constant     |
| O(N * log N) | Log linear  |
| O(N^2)     | Quadratic    |
| O(N^3)     | Cubic        |
| O(2^N)     | Exponential  |


[![Statistic](https://media.geeksforgeeks.org/wp-content/cdn-uploads/20220812122843/Logarithmic-time-complexity-blog-1.jpg)](https://media.geeksforgeeks.org/wp-content/cdn-uploads/20220812122843/Logarithmic-time-complexity-blog-1.jpg)



```
#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}
```

### Modular Design:
- mod balances;
- mod proof_of_existence;
- mod support;
- mod system;

 Each module can be optimized and scaled independently. 


## Ensuring Data Integrity and Immutability:


### Blockchain's Immutable Ledger:

 Blockchain technology ensures that once data is written, it cannot be altered 

### Block Structure (support.rs):

```
pub struct Block<Header, Extrinsic> {
    pub header: Header,
    pub extrinsics: Vec<Extrinsic>,
}
```

 Each block's header contains essential information (like block_number), and the block itself holds multiple transactions (extrinsics) 



### Proof of Existence (proof_of_existence.rs):

```
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}
```
 Storing claims in a BTreeMap and linking them to account IDs, you ensure that each claim is uniquely associated with its creator 



### System Module (system.rs):

```
#[derive(Debug)]
pub struct SystemPallet<T: Config> {
    pub block_number: T::BlockNumber,
    pub nonce: BTreeMap<T::AccountId, T::Nonce>,
}
```
 System module tracks the block number and manages nonces for each account, ensuring that each transaction is unique and ordered, which helps prevent double-spending and maintains data integrity 



### Transaction Validation:

``` 
pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
    match self.get_claim(&claim) {
        Some(_) => Err("claim already exists"),
        None => {
            self.claims.insert(claim, caller);
            Ok(())
        }
    }
}
```
 create_claim function checks if the claim already exists 



## User Account Management in My Code:


### Account ID Management (types.rs):
```
pub type AccountId = String;
```

### Balances Module (balances.rs):

```
#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

```
 The balances module stores account balances in a BTreeMap 



```
pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
    self.balances.insert(who.clone(), amount);
}
```
 set_balance function allows updating the balance of a specific account. 



```
pub fn get_balance(&self, who: &T::AccountId) -> T::Balance {
    *self.balances.get(who).unwrap_or(&T::Balance::zero())
}
```
 get_balance function retrieves the balance of a specific account, enabling the system to track account balances accurately 



### System Module (system.rs):

```
pub struct SystemPallet<T: Config> {
    pub nonce: BTreeMap<T::AccountId, T::Nonce>,
}
```
 The nonce map in the SystemPallet keeps track of the number of transactions sent by each account 



```
pub fn inc_nonce(&mut self, who: &T::AccountId) {
    let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
    self.nonce.insert(who.clone(), nonce + T::Nonce::one());
}
```
 inc_nonce function increments the nonce for a specific account, ensuring that each transaction is processed uniquely and sequentially 


## Installation
```
git clone https://github.com/sks006/HashChain.git
cd project
npm install
```
:rocket:
:tada: 
:bug:
:memo: 
:sparkles:
