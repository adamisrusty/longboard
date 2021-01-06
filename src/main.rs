use std::io::stdout;

use crossterm::{
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    Result,
};

use solana_sdk::signature::{Keypair, Signer};

fn main() -> Result<()> {
    let keypair = Keypair::new();
    let pubkey = bs58::encode(keypair.pubkey()).into_string();

    execute!(
        stdout(),
        SetForegroundColor(Color::Black),
        SetBackgroundColor(Color::Green),
        Print(pubkey),
    )?;
    Ok(())
}
