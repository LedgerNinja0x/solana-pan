use crate::errors::Error;
use crate::models::{Program, Token2022};
use crate::prelude::FankorResult;
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;

pub struct CpiInitializeMint2<'info> {
    pub mint: AccountInfo<'info>,
}

pub fn initialize_mint2(
    program: &Program<Token2022>,
    accounts: CpiInitializeMint2,
    decimals: u8,
    mint_authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
    signer_seeds: &[&[&[u8]]],
) -> FankorResult<()> {
    let ix = spl_token_2022::instruction::initialize_mint2(
        program.address(),
        accounts.mint.key,
        mint_authority,
        freeze_authority,
        decimals,
    )?;

    solana_program::program::invoke_signed(&ix, &[accounts.mint], signer_seeds)
        .map_or_else(|e| Err(Error::ProgramError(e)), |_| Ok(()))
}
