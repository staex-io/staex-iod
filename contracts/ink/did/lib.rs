#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod did {

    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Did {
        value: bool,
    }

    #[ink(event)]
    pub struct BeforeFlipping {
        from: AccountId,
        field1: u64,
        field2: String,
        field3: String,
    }

    #[ink(event)]
    pub struct AfterFlipping {
        from: AccountId,
        field1: u64,
        field2: String,
        field3: bool,
    }

    impl Did {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            ink::env::debug_print!("flip method executed");
            let from = self.env().caller();
            self.env().emit_event(BeforeFlipping {
                from,
                field1: 123,
                field2: String::from("asd"),
                field3: String::from("dsa"),
            });
            self.value = !self.value;
            self.env().emit_event(AfterFlipping {
                from,
                field1: 312,
                field2: String::from("qwe"),
                field3: true,
            });
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            ink::env::debug_print!("get method executed");
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let did = Did::default();
            assert!(!did.get());
        }

        #[ink::test]
        fn it_works() {
            let mut did = Did::new(false);
            assert!(!did.get());
            did.flip();
            assert!(did.get());
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;

        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = DidRef::default();

            let contract_account_id = client
                .instantiate("did", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<DidRef>(contract_account_id).call(|did| did.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = DidRef::new(false);
            let contract_account_id = client
                .instantiate("did", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<DidRef>(contract_account_id).call(|did| did.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            let flip = build_message::<DidRef>(contract_account_id).call(|did| did.flip());
            let _flip_result =
                client.call(&ink_e2e::bob(), flip, 0, None).await.expect("flip failed");

            let get = build_message::<DidRef>(contract_account_id).call(|did| did.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
