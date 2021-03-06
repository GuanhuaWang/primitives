extern crate core;
extern crate ethereum_types;

#[cfg(test)]
#[macro_use]
extern crate uint;
#[cfg(test)]
#[macro_use]
extern crate crunchy;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[cfg(test)]
extern crate serde_json;

#[cfg(test)]
pub mod hash_tests;
#[cfg(test)]
pub mod uint_tests;
#[cfg(test)]
pub mod serialization;
