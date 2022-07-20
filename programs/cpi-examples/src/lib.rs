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

    pub fn withdraw_example<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, WithdrawExample<'info>>,
        withdraw_amount: u64,
    ) -> Result<()> {
        let (_, bump) = get_authority();
        let base_seeds = &[b"daoProgramAuthority" as &[u8], &[bump]];
        let seeds = [&base_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.volt_program_id.clone(),
            volt_abi::cpi::accounts::Withdraw {
                authority: ctx.accounts.authority.to_account_info(),
                dao_authority: ctx.accounts.dao_authority.to_account_info(),
                authority_check: ctx.accounts.dao_authority.to_account_info(),
                vault_mint: ctx.accounts.vault_mint.to_account_info(),
                volt_vault: ctx.accounts.volt_vault.to_account_info(),
                vault_authority: ctx.accounts.vault_authority.to_account_info(),
                extra_volt_data: ctx.accounts.extra_volt_data.to_account_info(),
                whitelist: ctx.accounts.whitelist.to_account_info(),
                deposit_pool: ctx.accounts.deposit_pool.to_account_info(),
                vault_token_source: ctx.accounts.vault_token_source.to_account_info(),
                underlying_token_destination: ctx
                    .accounts
                    .underlying_token_destination
                    .to_account_info(),
                round_info: ctx.accounts.round_info.to_account_info(),
                round_underlying_tokens: ctx.accounts.round_underlying_tokens.to_account_info(),
                pending_withdrawal_info: ctx.accounts.pending_withdrawal_info.to_account_info(),
                epoch_info: ctx.accounts.epoch_info.to_account_info(),
                fee_acct: ctx.accounts.fee_acct.to_account_info(),

                rent: ctx.accounts.rent.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
            &seeds,
        );

        volt_abi::cpi::withdraw(cpi_ctx, withdraw_amount).unwrap();
        Ok(())
    }

    pub fn deposit_with_claim_example<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, DepositWithClaimExample<'info>>,
        deposit_amount: u64,
    ) -> Result<()> {
        let (_, bump) = get_authority();
        let base_seeds = &[b"daoProgramAuthority" as &[u8], &[bump]];
        let seeds = [&base_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.volt_program_id.clone(),
            volt_abi::cpi::accounts::DepositWithClaim {
                authority: ctx.accounts.authority.to_account_info(),
                dao_authority: ctx.accounts.dao_authority.to_account_info(),
                authority_check: ctx.accounts.dao_authority.to_account_info(),
                vault_mint: ctx.accounts.vault_mint.to_account_info(),
                volt_vault: ctx.accounts.volt_vault.to_account_info(),
                vault_authority: ctx.accounts.vault_authority.to_account_info(),
                extra_volt_data: ctx.accounts.extra_volt_data.to_account_info(),
                deposit_pool: ctx.accounts.deposit_pool.to_account_info(),
                writer_token_pool: ctx.accounts.writer_token_pool.to_account_info(),
                vault_token_destination: ctx.accounts.vault_token_destination.to_account_info(),
                underlying_token_source: ctx.accounts.underlying_token_source.to_account_info(),
                round_info: ctx.accounts.round_info.to_account_info(),
                round_underlying_tokens: ctx.accounts.round_underlying_tokens.to_account_info(),
                pending_deposit_info: ctx.accounts.pending_deposit_info.to_account_info(),
                epoch_info: ctx.accounts.epoch_info.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                sol_transfer_authority: ctx.accounts.authority.to_account_info(),
                pending_deposit_round_info: ctx
                    .accounts
                    .pending_deposit_round_info
                    .to_account_info(),
                pending_deposit_round_volt_tokens: ctx
                    .accounts
                    .pending_deposit_round_volt_tokens
                    .to_account_info(),
                pending_deposit_round_underlying_tokens: ctx
                    .accounts
                    .pending_deposit_round_underlying_tokens
                    .to_account_info(),
            },
            &seeds,
        );

        volt_abi::cpi::deposit_with_claim(cpi_ctx, deposit_amount, false).unwrap();
        Ok(())
    }

    pub fn withdraw_with_claim_example<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, WithdrawWithClaimExample<'info>>,
        withdraw_amount: u64,
    ) -> Result<()> {
        let (_, bump) = get_authority();
        let base_seeds = &[b"daoProgramAuthority" as &[u8], &[bump]];
        let seeds = [&base_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.volt_program_id.clone(),
            volt_abi::cpi::accounts::WithdrawWithClaim {
                authority: ctx.accounts.authority.to_account_info(),
                dao_authority: ctx.accounts.dao_authority.to_account_info(),
                authority_check: ctx.accounts.dao_authority.to_account_info(),
                vault_mint: ctx.accounts.vault_mint.to_account_info(),
                volt_vault: ctx.accounts.volt_vault.to_account_info(),
                vault_authority: ctx.accounts.vault_authority.to_account_info(),
                extra_volt_data: ctx.accounts.extra_volt_data.to_account_info(),
                deposit_pool: ctx.accounts.deposit_pool.to_account_info(),
                vault_token_source: ctx.accounts.vault_token_source.to_account_info(),
                underlying_token_destination: ctx
                    .accounts
                    .underlying_token_destination
                    .to_account_info(),
                round_info: ctx.accounts.round_info.to_account_info(),
                round_underlying_tokens: ctx.accounts.round_underlying_tokens.to_account_info(),
                pending_withdrawal_info: ctx.accounts.pending_withdrawal_info.to_account_info(),
                epoch_info: ctx.accounts.epoch_info.to_account_info(),
                fee_acct: ctx.accounts.fee_acct.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                pending_withdrawal_round_info: ctx
                    .accounts
                    .pending_withdrawal_round_info
                    .to_account_info(),
                pending_withdrawal_round_underlying_tokens_for_pws: ctx
                    .accounts
                    .pending_withdrawal_round_underlying_tokens_for_pws
                    .to_account_info(),
            },
            &seeds,
        );

        volt_abi::cpi::withdraw_with_claim(cpi_ctx, withdraw_amount).unwrap();
        Ok(())
    }

    pub fn claim_pending_deposit_example<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ClaimPendingDepositExample<'info>>,
    ) -> Result<()> {
        let (_, bump) = get_authority();
        let base_seeds = &[b"daoProgramAuthority" as &[u8], &[bump]];
        let seeds = [&base_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.volt_program_id.clone(),
            volt_abi::cpi::accounts::ClaimPendingDeposit {
                authority: ctx.accounts.authority.to_account_info(),
                volt_vault: ctx.accounts.volt_vault.to_account_info(),
                vault_authority: ctx.accounts.vault_authority.to_account_info(),
                extra_volt_data: ctx.accounts.extra_volt_data.to_account_info(),
                user_vault_tokens: ctx.accounts.user_vault_tokens.to_account_info(),
                pending_deposit_info: ctx.accounts.pending_deposit_info.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                pending_deposit_round_info: ctx
                    .accounts
                    .pending_deposit_round_info
                    .to_account_info(),
                pending_deposit_round_volt_tokens: ctx
                    .accounts
                    .pending_deposit_round_volt_tokens
                    .to_account_info(),
            },
            &seeds,
        );

        volt_abi::cpi::claim_pending(cpi_ctx).unwrap();
        Ok(())
    }

    pub fn claim_pending_withdrawal_example<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ClaimPendingWithdrawalExample<'info>>,
    ) -> Result<()> {
        let (_, bump) = get_authority();
        let base_seeds = &[b"daoProgramAuthority" as &[u8], &[bump]];
        let seeds = [&base_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.volt_program_id.clone(),
            volt_abi::cpi::accounts::ClaimPendingWithdrawal {
                authority: ctx.accounts.authority.to_account_info(),
                vault_mint: ctx.accounts.vault_mint.to_account_info(),
                volt_vault: ctx.accounts.volt_vault.to_account_info(),
                vault_authority: ctx.accounts.vault_authority.to_account_info(),
                extra_volt_data: ctx.accounts.extra_volt_data.to_account_info(),
                underlying_token_destination: ctx
                    .accounts
                    .underlying_token_destination
                    .to_account_info(),
                pending_withdrawal_info: ctx.accounts.pending_withdrawal_info.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                pending_withdrawal_round_info: ctx
                    .accounts
                    .pending_withdrawal_round_info
                    .to_account_info(),
                round_underlying_tokens_for_pending_withdrawals: ctx
                    .accounts
                    .round_underlying_tokens_for_pending_withdrawals
                    .to_account_info(),
            },
            &seeds,
        );

        volt_abi::cpi::claim_pending_withdrawal(cpi_ctx).unwrap();
        Ok(())
    }

    pub fn cancel_pending_deposit_example<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CancelPendingDepositExample<'info>>,
    ) -> Result<()> {
        let (_, bump) = get_authority();
        let base_seeds = &[b"daoProgramAuthority" as &[u8], &[bump]];
        let seeds = [&base_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.volt_program_id.clone(),
            volt_abi::cpi::accounts::CancelPendingDeposit {
                authority: ctx.accounts.authority.to_account_info(),
                volt_vault: ctx.accounts.volt_vault.to_account_info(),
                extra_volt_data: ctx.accounts.extra_volt_data.to_account_info(),
                vault_authority: ctx.accounts.vault_authority.to_account_info(),
                pending_deposit_info: ctx.accounts.pending_deposit_info.to_account_info(),
                underlying_token_destination: ctx
                    .accounts
                    .underlying_token_destination
                    .to_account_info(),
                round_underlying_tokens: ctx.accounts.round_underlying_tokens.to_account_info(),
                round_info: ctx.accounts.round_info.to_account_info(),
                epoch_info: ctx.accounts.epoch_info.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
            &seeds,
        );

        volt_abi::cpi::cancel_pending_deposit(cpi_ctx).unwrap();
        Ok(())
    }

    pub fn cancel_pending_withdrawal_example<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CancelPendingWithdrawalExample<'info>>,
    ) -> Result<()> {
        let (_, bump) = get_authority();
        let base_seeds = &[b"daoProgramAuthority" as &[u8], &[bump]];
        let seeds = [&base_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.volt_program_id.clone(),
            volt_abi::cpi::accounts::CancelPendingWithdrawal {
                authority: ctx.accounts.authority.to_account_info(),
                vault_mint: ctx.accounts.vault_mint.to_account_info(),
                volt_vault: ctx.accounts.volt_vault.to_account_info(),
                vault_authority: ctx.accounts.vault_authority.to_account_info(),
                extra_volt_data: ctx.accounts.extra_volt_data.to_account_info(),
                pending_withdrawal_info: ctx.accounts.pending_withdrawal_info.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                vault_token_destination: ctx.accounts.vault_token_destination.to_account_info(),
                round_info: ctx.accounts.round_info.to_account_info(),
                epoch_info: ctx.accounts.epoch_info.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &seeds,
        );

        volt_abi::cpi::cancel_pending_withdrawal(cpi_ctx).unwrap();
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("invalid deposit program id")]
    InvalidDepositProgramId,
    #[msg("invalid dao authority")]
    InvalidDaoAuthority,
}
