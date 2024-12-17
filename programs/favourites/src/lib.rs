use anchor_lang::prelude::*;

declare_id!("J17SgfdpURh63ScmEXJ6u9hC94rqSj7gBT96KT1boDSb");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favourites {
    use super::*;

    pub fn set_favourite(
        context: Context<SetFavourites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greetings from {}", context.program_id);

        #[allow(unused_variables)]
        let user_public_key = context.accounts.user.key();

        msg!("User {user_public_key}'s fav number: {number}, fav color is {color} and hobbies are {hobbies:?}");

        context.accounts.favourites.set_inner(Favourites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
}

// Since we are going to save this in account we are going to use the macro
#[account]
#[derive(InitSpace)] // This gives us the space of total struct
pub struct Favourites {
    pub number: u64,

    #[max_len(59)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavourites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        // Create if needed
        init_if_needed,
        // Who is going to pay for the information to write on blockchain
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favourites::INIT_SPACE,
        seeds = [b"favourites", user.key().as_ref()], // So we are setting the key as the userKey
                                                      // and the favourite
        bump
    )]
    pub favourites: Account<'info, Favourites>,

    pub system_program: Program<'info, System>,
}
