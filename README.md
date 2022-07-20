# Friktion (volt) ABI

## Overview

Provides contexts and instruction functions to interact with Friktion Program via CPI calls

Friktion program ID: ```VoLT1mJz1sbnxwq5Fv2SXjdVDgPXrb9tJyC8WpMDkSp```

## Devnet Faucets

Can be found here: https://app.friktion.fi/faucet

## Docs

Full docs can be found at https://docs.friktion.fi/integration/rust-abi

## Examples

The following is an example of a CPI call into the Friktion program calling the deposit instruction. Anchor provides [docs](https://project-serum.github.io/anchor/tutorials/tutorial-3.html) on the syntax used.


```rust
use anchor_lang::declare_id;
use anchor_lang::prelude::*;
mod contexts;
pub use crate::contexts::*;

declare_id!("DAo2pDtpiBFDu4TTiv2WggP6PfQ6FnKqwSRYxpMjyuV2");

fn get_authority() -> (Pubkey, u8) {
    return Pubkey::find_program_address(&[b"daoProgramAuthority" as &[u8]], &crate::id());
}

#[program]
pub mod cpi_examples {

    use super::*;
    pub fn deposit_example<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, DepositExample<'info>>,
        deposit_amount: u64,
    ) -> Result<()> {
        let (_, bump) = get_authority();
        let base_seeds = &[b"daoProgramAuthority" as &[u8], &[bump]];
        let seeds = [&base_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.volt_program_id.clone(),
            volt_abi::cpi::accounts::Deposit {
                authority: ctx.accounts.authority.to_account_info(),
                dao_authority: ctx.accounts.dao_authority.to_account_info(),
                authority_check: ctx.accounts.dao_authority.to_account_info(),
                vault_mint: ctx.accounts.vault_mint.to_account_info(),
                volt_vault: ctx.accounts.volt_vault.to_account_info(),
                vault_authority: ctx.accounts.vault_authority.to_account_info(),
                extra_volt_data: ctx.accounts.extra_volt_data.to_account_info(),
                whitelist: ctx.accounts.whitelist.to_account_info(),
                deposit_pool: ctx.accounts.deposit_pool.to_account_info(),
                writer_token_pool: ctx.accounts.writer_token_pool.to_account_info(),
                vault_token_destination: ctx.accounts.vault_token_destination.to_account_info(),
                underlying_token_source: ctx.accounts.underlying_token_source.to_account_info(),
                round_info: ctx.accounts.round_info.to_account_info(),
                round_volt_tokens: ctx.accounts.round_volt_tokens.to_account_info(),
                round_underlying_tokens: ctx.accounts.round_underlying_tokens.to_account_info(),
                pending_deposit_info: ctx.accounts.pending_deposit_info.to_account_info(),
                entropy_program: ctx.accounts.entropy_program.to_account_info(),
                entropy_group: ctx.accounts.entropy_group.to_account_info(),
                entropy_cache: ctx.accounts.entropy_cache.to_account_info(),
                entropy_account: ctx.accounts.entropy_account.to_account_info(),
                epoch_info: ctx.accounts.epoch_info.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
            &seeds,
        );

        volt_abi::cpi::deposit(cpi_ctx, deposit_amount).unwrap();
        Ok(())
    }
}
```

using the following context:

```rust

#[derive(Accounts)]
#[instruction(
    deposit_amount: u64,
)]
pub struct DepositExample<'info> {
    #[account(mut, signer)]
    /// CHECK: skip, checked by the volt program
    pub authority: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub dao_authority: AccountInfo<'info>,

    #[account(address=volt_abi::id())]
    /// CHECK: skip, checked by the volt program, will check program id in instruction
    pub volt_program_id: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub vault_mint: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub volt_vault: AccountInfo<'info>,
    /// CHECK: skip, checked by the volt program
    pub vault_authority: AccountInfo<'info>,

    #[account()]
    /// CHECK: skip, checked by the volt program
    pub extra_volt_data: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub whitelist: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub deposit_pool: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub writer_token_pool: AccountInfo<'info>,

    #[account(mut, token::authority=dao_authority.key())]
    // user controlled token account w/ mint == vault mint
    /// CHECK: skip, checked by the volt program
    pub vault_token_destination: Box<Account<'info, TokenAccount>>,

    #[account(mut, token::authority=dao_authority.key())]
    // user controlled token account w/ mint == underlying mint
    /// CHECK: skip, checked by the volt program
    pub underlying_token_source: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub round_info: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub round_volt_tokens: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub round_underlying_tokens: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub epoch_info: AccountInfo<'info>,
    /// CHECK: skip, checked by the volt program
    #[account(mut)]
    pub pending_deposit_info: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub entropy_program: AccountInfo<'info>,
    /// CHECK: skip, checked by the volt program
    pub entropy_group: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub entropy_account: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub entropy_cache: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub system_program: AccountInfo<'info>,
    /// CHECK: skip, checked by the volt program
    pub token_program: AccountInfo<'info>,
}
```

Keep in mind, the CPI feature must be enabled on "volt-abi" as the following demonstrates:

```rust
volt-abi = { version = "0.2.0", features = ["cpi"]}
```


## Practical Notes:

- vault_token_destination && underlying_token_source accounts must be initialized prior to calling the Deposit instruction. The authorities on those token accounts must be the dao_authority, which practically is a PDA of the invoking program.
