use fp_account::EthereumSignature;
use sp_core::{Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

pub type Signature = EthereumSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
fn main() {
    let seed = match std::env::args().skip(1).next() {
        Some(seed) => seed,
        None => "Alice".to_string(),
    };
    println!(
        "seed {seed} -> {} ",
        get_account_id_from_seed::<sp_core::ecdsa::Public>(&seed)
    );
}
/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn authority_keys_from_seed(s: &str) -> AccountId {
    get_account_id_from_seed::<sp_core::ecdsa::Public>(s)
}
