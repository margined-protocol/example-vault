use crate::ExampleVault;
use margined_vault::contract::Describe;

impl Describe for ExampleVault {
    const CONTRACT_NAME: &'static str = env!("CARGO_PKG_NAME");
    const VAULT_STANDARD_VERSION: u16 = 1;
    const VAULT_STANDARD_EXTENSIONS: [&'static str; 2] = ["lockup", "force-unlock"];
}
