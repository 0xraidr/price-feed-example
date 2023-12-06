use anchor_lang::prelude::*;
use pyth_sdk_solana::load_price_feed_from_account_info;
use std::str::FromStr;

const SOL_USDC_FEED: &str = "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix";
const STALENESS_THRESHOLD: u64 = 60; // staleness threshold in seconds

declare_id!("F61FWPgNwNZoyo2PhqYZPY5VoSgoT69p8fBfW8kKiKcx");

#[program]
pub mod pyth_example {
    use super::*;

    pub fn fetch_price(ctx: Context<FetchSolanaPrice>) -> Result<()> {

        // Set desired price address and load current price     
                let price_account_info = &ctx.accounts.price_feed;
                let price_feed = load_price_feed_from_account_info( &price_account_info ).unwrap();
                let current_timestamp = Clock::get()?.unix_timestamp;
                let current_price = price_feed.get_price_no_older_than(current_timestamp, STALENESS_THRESHOLD).unwrap();
       
        // Convert price for human readability for the values in nearest dollar amount
                let display_price = u64::try_from(current_price.price).unwrap() / 10u64.pow(u32::try_from(-current_price.expo).unwrap());
                let display_confidence = u64::try_from(current_price.conf).unwrap() / 10u64.pow(u32::try_from(-current_price.expo).unwrap());

        // Log current price
        msg!("SOL/USD PRICE: ({} +- {})", display_price, display_confidence);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FetchSolanaPrice<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: we are just reading this
    #[account(address = Pubkey::from_str(SOL_USDC_FEED).unwrap() @ FeedError::InvalidPriceFeed)]
    pub price_feed: AccountInfo<'info>,
}



#[error_code]
pub enum FeedError {
    #[msg("Invalid Price Feed")]
    InvalidPriceFeed,
}
