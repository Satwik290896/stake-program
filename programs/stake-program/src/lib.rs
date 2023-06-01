use anchor_lang::prelude::*;

use anchor_spl::{
associated_token::AssociatedToken,
token::{self, Mint, Token, TokenAccount, Transfer, transfer}
};

use solana_program::clock::Clock;

declare_id!("D7MtJpDRCFLVrqRZU5qY7PjhTaL97QFuamWn8PSWHv6o");

pub mod constants {
pub const STAKE_POOL_SEED: &[u8] = b"stake_pool";
pub const STAKE_INFO_SEED: &[u8] = b"stake_info";
pub const TOKEN_ACCOUNT_SEED: &[u8] = b"token";
}

#[program]
pub mod stake_program {
use super::*;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    Ok(())
}

pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let stake_info = &mut ctx.accounts.stake_info;

    if stake_info.is_staked {
        return Err(ErrorCode::IsStaked.into());
    }
    if amount <= 0 {
        return Err(ErrorCode::NoTokens.into());
    }

    let clock = Clock::get()?;

    stake_info.staked_at_slot = clock.slot;
    stake_info.is_staked = true;

    let stake_amount = (amount)
        .checked_mul(10u64.pow(ctx.accounts.mint.decimals as u32))
        .unwrap();

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.player_token_account.to_account_info(),
                to: ctx.accounts.player_stake_token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            }
        ),
        stake_amount,
    )?;        

    Ok(())
}

pub fn destake(ctx: Context<DeStake>) -> Result<()> {
    let stake_info = &mut ctx.accounts.stake_info;

    if !stake_info.is_staked {
        return Err(ErrorCode::NotStaked.into());
    }

    let clock = Clock::get()?;
    let slots_passed = clock.slot - stake_info.staked_at_slot;

    let stake_amount = ctx.accounts.player_stake_token_account.amount;

    let reward = (slots_passed as u64)
        .checked_mul(10u64.pow(ctx.accounts.mint.decimals as u32))
        .unwrap();

    let bump = *ctx.bumps.get("stake_pool_account").unwrap();
    let signer: &[&[&[u8]]] = &[&[constants::STAKE_POOL_SEED, &[bump]]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.stake_pool_account.to_account_info(),
                to: ctx.accounts.player_token_account.to_account_info(),
                authority: ctx.accounts.stake_pool_account.to_account_info(),
            },
            signer
        ),
        reward
    );

    let staker = ctx.accounts.signer.key();
    let bump = *ctx.bumps.get("player_stake_token_account").unwrap();
    let signer: &[&[&[u8]]] = &[&[constants::TOKEN_ACCOUNT_SEED, staker.as_ref(),&[bump]]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer{
                from: ctx.accounts.player_stake_token_account.to_account_info(),
                to: ctx.accounts.player_token_account.to_account_info(),
                authority: ctx.accounts.player_stake_token_account.to_account_info(),
            },
            signer
        ),
        stake_amount
    );

    stake_info.is_staked = false;
    stake_info.staked_at_slot = clock.slot;

    Ok(())
}
}

#[derive(Accounts)]
pub struct Initialize<'info> {

#[account(mut)]
pub signer: Signer<'info>,

#[account(
    init_if_needed,
    seeds = [constants::STAKE_POOL_SEED],
    bump,
    payer = signer,
    token::mint = mint,
    token::authority = stake_pool_account,
)]
pub stake_pool_account: Account<'info, TokenAccount>,

pub mint: Account<'info, Mint>,
pub token_program: Program<'info, Token>,
pub associated_token_program: Program<'info, AssociatedToken>,
pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeStake<'info> {
#[account(mut)]
pub signer: Signer<'info>,

#[account(
    mut,
    seeds = [constants::STAKE_INFO_SEED, signer.key.as_ref()],
    bump
)]
pub stake_info: Account<'info, StakeInfo>,

#[account(
    mut,
    seeds = [constants::TOKEN_ACCOUNT_SEED, signer.key.as_ref()],
    bump
)]
pub player_stake_token_account: Account<'info, TokenAccount>,

#[account(
    mut,
    seeds = [constants::STAKE_POOL_SEED],
    bump
)]
pub stake_pool_account: Account<'info, TokenAccount>,

#[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = signer,
)]
pub player_token_account: Account<'info, TokenAccount>,

pub mint: Account<'info, Mint>,
pub token_program: Program<'info, Token>,
pub associated_token_program: Program<'info, AssociatedToken>,
pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
#[account(mut)]
pub signer: Signer<'info>,

#[account(
    init_if_needed,
    seeds = [constants::STAKE_INFO_SEED, signer.key().as_ref()],
    bump,
    payer = signer,
    space = 8 + std::mem::size_of::<StakeInfo>()
)]
pub stake_info: Account<'info, StakeInfo>,

#[account(
    init_if_needed,
    seeds = [constants::TOKEN_ACCOUNT_SEED, signer.key.as_ref()],
    bump,
    payer = signer,
    token::mint = mint,
    token::authority = player_stake_token_account,
)]
pub player_stake_token_account: Account<'info, TokenAccount>,

#[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = signer,
)]
pub player_token_account: Account<'info, TokenAccount>,

pub mint: Account<'info, Mint>,
pub token_program: Program<'info, Token>,
pub associated_token_program: Program<'info, AssociatedToken>,
pub system_program: Program<'info, System>,
}

#[account]
pub struct StakeInfo {
pub staked_at_slot: u64,
pub is_staked: bool,
}

#[error_code]
pub enum ErrorCode {
#[msg("Tokens are already staked")]
IsStaked,
#[msg("Tokens not staked")]
NotStaked,
#[msg("No tokens to stake")]
NoTokens,
}