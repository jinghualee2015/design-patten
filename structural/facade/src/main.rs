use crate::wallet_facade::WalletFacade;

mod account;
mod ledger;
mod notification;
mod wallet;
mod security_code;
mod wallet_facade;

fn main() -> Result<(), String> {
    let mut wallet = WalletFacade::new("abc".into(), 1234);
    println!();

    wallet.add_money_to_wallet(&"abc".into(), 1234, 10)?;
    println!();

    wallet.deduct_money_from_wallet(&"abc".into(), 1234, 5)
}
