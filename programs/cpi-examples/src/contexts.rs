use anchor_lang::prelude::*;
use anchor_lang::{account, Accounts};
use anchor_spl::token::TokenAccount;

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

#[derive(Accounts)]
#[instruction(
    withdraw_amount: u64,
)]
pub struct WithdrawExample<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account()]
    /// CHECK: skip, checked by volt program
    pub dao_authority: AccountInfo<'info>,

    /// CHECK: skip, checked by volt program
    pub authority_check: AccountInfo<'info>,

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

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub extra_volt_data: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub whitelist: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub deposit_pool: AccountInfo<'info>,

    #[account(mut)]
    // user controlled token account w/ mint == vault mint
    /// CHECK: skip, checked by the volt program
    pub vault_token_source: AccountInfo<'info>,

    #[account(mut)]
    // user controlled token account w/ mint == underlying mint
    /// CHECK: skip, checked by the volt program
    pub underlying_token_destination: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub round_info: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub round_underlying_tokens: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    #[account(mut)]
    pub pending_withdrawal_info: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub epoch_info: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub fee_acct: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub system_program: AccountInfo<'info>,
    /// CHECK: skip, checked by the volt program
    pub token_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(
    deposit_amount: u64,
)]
pub struct DepositWithClaimExample<'info> {
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

    /// CHECK: skip, checked by the volt program
    #[account(mut)]
    pub pending_deposit_info: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub pending_deposit_round_info: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub pending_deposit_round_volt_tokens: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    #[account(mut)]
    pub pending_deposit_round_underlying_tokens: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    #[account(mut)]
    pub epoch_info: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub system_program: AccountInfo<'info>,
    /// CHECK: skip, checked by the volt program
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(
    withdraw_amount: u64,
)]
pub struct WithdrawWithClaimExample<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account()]
    /// CHECK: skip, checked by volt program
    pub dao_authority: AccountInfo<'info>,

    /// CHECK: skip, checked by volt program
    pub authority_check: AccountInfo<'info>,

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

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub extra_volt_data: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub whitelist: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub deposit_pool: AccountInfo<'info>,

    #[account(mut)]
    // user controlled token account w/ mint == vault mint
    /// CHECK: skip, checked by the volt program
    pub vault_token_source: AccountInfo<'info>,

    #[account(mut)]
    // user controlled token account w/ mint == underlying mint
    /// CHECK: skip, checked by the volt program
    pub underlying_token_destination: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub round_info: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub round_underlying_tokens: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub pending_withdrawal_round_info: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    #[account(mut)]
    pub pending_withdrawal_info: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub pending_withdrawal_round_underlying_tokens_for_pws: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub epoch_info: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub fee_acct: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub system_program: AccountInfo<'info>,
    /// CHECK: skip, checked by the volt program
    pub token_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ClaimPendingDepositExample<'info> {
    /// CHECK: skip, checked by the volt program
    pub authority: AccountInfo<'info>,

    #[account()]
    /// CHECK: skip, checked by the volt program, will check program id in instruction
    pub volt_program_id: AccountInfo<'info>,

    #[account()]
    /// CHECK: skip, checked by the volt program
    pub volt_vault: AccountInfo<'info>,

    #[account()]
    /// CHECK: skip, checked by the volt program
    pub extra_volt_data: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub vault_authority: AccountInfo<'info>,

    #[account(mut)]
    // user controlled token account w/ mint == underlying mint
    /// CHECK: skip, checked by the volt program
    pub user_vault_tokens: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub pending_deposit_round_info: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub pending_deposit_round_volt_tokens: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    #[account(mut)]
    pub pending_deposit_info: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub system_program: AccountInfo<'info>,
    /// CHECK: skip, checked by the volt program
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ClaimPendingWithdrawalExample<'info> {
    /// CHECK: skip, checked by the volt program
    pub authority: AccountInfo<'info>,

    #[account(address=volt_abi::id())]
    /// CHECK: skip, checked by the volt program, will check program id in instruction
    pub volt_program_id: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub volt_vault: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub extra_volt_data: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub vault_authority: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub vault_mint: AccountInfo<'info>,

    #[account(mut)]
    // user controlled token account w/ mint == vault mint
    /// CHECK: skip, checked by the volt program
    pub underlying_token_destination: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub pending_withdrawal_round_info: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    #[account(mut)]
    pub pending_withdrawal_info: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub round_underlying_tokens_for_pending_withdrawals: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub system_program: AccountInfo<'info>,
    /// CHECK: skip, checked by the volt program
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts, Debug)]
pub struct CancelPendingDepositExample<'info> {
    /// CHECK: skip, checked by the volt program
    #[account(mut)]
    pub authority: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    #[account()]
    vault_mint: AccountInfo<'info>,

    #[account()]
    /// CHECK: skip, checked by the volt program, will check program id in instruction
    pub volt_program_id: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub volt_vault: AccountInfo<'info>,

    #[account()]
    /// CHECK: skip, checked by the volt program
    pub extra_volt_data: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub vault_authority: AccountInfo<'info>,

    #[account(mut)]
    // user controlled token account w/ mint == underlying mint
    /// CHECK: skip, checked by the volt program
    pub underlying_token_destination: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    #[account(mut)]
    pub pending_deposit_info: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub round_info: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub round_underlying_tokens: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub epoch_info: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,

    /// CHECK: skip, checked by the volt program
    pub system_program: AccountInfo<'info>,
    /// CHECK: skip, checked by the volt program
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelPendingWithdrawalExample<'info> {
    /// CHECK: skip, checked by the volt program
    pub authority: AccountInfo<'info>,

    #[account(address=volt_abi::id())]
    /// CHECK: skip, checked by the volt program, will check program id in instruction
    pub volt_program_id: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub volt_vault: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub extra_volt_data: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    pub vault_authority: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub vault_mint: AccountInfo<'info>,

    #[account(mut)]
    // user controlled token account w/ mint == vault mint
    /// CHECK: skip, checked by the volt program
    pub vault_token_destination: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    #[account(mut)]
    pub pending_withdrawal_info: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub round_info: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: skip, checked by the volt program
    pub round_underlying_tokens: AccountInfo<'info>,

    /// CHECK: skip, checked by the volt program
    #[account(mut)]
    pub epoch_info: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,

    /// CHECK: skip, checked by the volt program
    pub system_program: AccountInfo<'info>,
    /// CHECK: skip, checked by the volt program
    pub token_program: AccountInfo<'info>,
}
