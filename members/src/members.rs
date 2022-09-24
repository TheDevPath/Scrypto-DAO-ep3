use scrypto::prelude::*;

blueprint! {

  pub struct Members{
  // define members component state here ... this is what lives on the blockchain
        vaults: KeyValueStore<ResourceAddress, Vault>,
        all_auth_resources: Vec<ResourceAddress>,
        auth_vault_alpha: Vault,
        auth_vault_bravo: Vault,
        auth_vault_charlie: Vault,
  }

  impl Members {
        pub fn instantiate() -> ComponentAddress {
            let alpha =
                Vault::with_bucket(Members::create_basic_badge("Alpha".to_owned()));
            let bravo =
                Vault::with_bucket(Members::create_basic_badge("Bravo".to_owned()));
            let charlie =
                Vault::with_bucket(Members::create_basic_badge("Charlie".to_owned()));
            let authorities: Vec<ResourceAddress> = vec![
                alpha.resource_address(),
                bravo.resource_address(),
                charlie.resource_address(),
            ];

            Self {
                vaults: KeyValueStore::new(),
                all_auth_resources: authorities,
                auth_vault_alpha: alpha,
                auth_vault_bravo: bravo,
                auth_vault_charlie: charlie,
            }
            .instantiate()
            .globalize()
        }

        pub fn create_basic_badge(name: String) -> Bucket {
            ResourceBuilder::new_fungible()
                .metadata("name", format!("{} authority token", name))
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(1)
        }
  }
}