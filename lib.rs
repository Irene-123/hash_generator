#![cfg_attr(not(feature = "std"), no_std)]
use ink_lang as ink;
use sha2::{Sha256, Digest};
use base64ct::{Base64, Encoding};


#[ink::contract]
mod hash_generator {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct HashGenerator {
        /// Stores a single `bool` value on the storage.
      
        hash: u8
    }

    impl HashGenerator {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
            hash: 0}
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            use sha2::*;
            use base64ct::*;
          

            let mut hasher = Sha256::new();
            let data = b"Meet me tom @2 pm";
            hasher.update(data);
            // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
            // hasher.update("String data");
            // Note that calling `finalize()` consumeshasher
            self.hash = hasher.finalize().as_slice().to_vec()[0];
            ink_env::debug_println!("{}", "Hello log");
            
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> u8 {
            self.hash
        }
    }

    // Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    // module and test functions are marked with a `#[test]` attribute.
    // The below code is technically just normal Rust code.
    // #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        // We test if the default constructor does its job.
        // #[ink::test]
        // fn default_works() {
        //     let hash_generator = HashGenerator::default();
        //     assert_eq!(hash_generator.get(), false);
        // }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut hash_generator = HashGenerator::new();
            // assert_eq!(hash_generator.get(), false);
            println!("{}", hash_generator.get());
            // hash_generator.flip();
            // assert_eq!(hash_generator.get(), true);
        }
    }
}
