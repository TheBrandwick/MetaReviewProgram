use anchor_lang::prelude::*;

declare_id!("9xUCA4apJEJRUVSpyYrXwjC12mTGYRgBJSd38focKqNC");

#[program]
pub mod sol_survey {
    use super::*;
    use anchor_lang::solana_program::system_instruction::transfer;
    use anchor_lang::solana_program::program::invoke;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let program_state = &mut ctx.accounts.program_state;
        program_state.owner = ctx.accounts.owner.key();
        program_state.survey_count = 0;
        program_state.user_count = 0;
        Ok(())
    }
    pub fn create_survey(
        ctx: Context<CreateSurvey>,
        bump: u8,
        max_participants_count: u64,
        reward_per_participant: u64,
        valid_until: u64,
        is_draft: bool,
        is_active: bool,
        form_uri: String,
    ) -> Result<()> {
        let program_state = &mut ctx.accounts.program_state;
        let survey = &mut ctx.accounts.survey;
        let creator = &mut ctx.accounts.creator;

        let total_funds_to_deposit = max_participants_count * reward_per_participant;
        let ix = transfer(&creator.key, &survey.key(), total_funds_to_deposit);

        invoke(&ix, &[
            creator.to_account_info(),
            survey.to_account_info(),
        ]).ok();

        survey.id = program_state.survey_count;
        survey.creator = creator.key();
        survey.max_participants_count = max_participants_count;
        survey.current_participants_count = 0;

        // TODO: check if the creator is charged correct total amount based on the max participants
        survey.reward_per_participant = reward_per_participant;
        survey.valid_until = valid_until;

        // TODO: make sure that both is_draft and is_active are not together true
        survey.is_draft = is_draft;
        survey.is_active = is_active;
        survey.form_uri = form_uri;

        program_state.survey_count += 1;
        Ok(())
    }
    pub fn edit_survey(
        ctx: Context<EditSurvey>,
        is_draft: bool,
        is_active: bool,
        form_uri: String, // link should be less than equal to initial value in size
    ) -> Result<()> {
        let survey = &mut ctx.accounts.survey;
        let user = &mut ctx.accounts.user;

        if Some(is_draft) != None {
            survey.is_draft = is_draft;
        }
        if Some(is_active) != None {
            survey.is_active = is_active;
        }
        survey.form_uri = form_uri;
        Ok(())
    }
    pub fn participate_survey(ctx: Context<ParticipateSurvey>, user_id: u64) -> Result<()> {
        let participation = &mut ctx.accounts.participation;
        let survey = &mut ctx.accounts.survey;
        let user = &mut ctx.accounts.user;

        // TODO: Check for validity of survey

        participation.id = survey.current_participants_count;
        participation.participant_address = user.key();
        participation.user_id = user_id;
        participation.survey_id = survey.id;
        participation.completed = false;
        participation.rewarded = false;

        survey.current_participants_count += 1;
        Ok(())
    }
    pub fn submit_survey_as_participant(ctx: Context<SubmitSurveyAsParticipant>, user_id: u64) -> Result<()> {
        let participation = &mut ctx.accounts.participation;
        let user = &mut ctx.accounts.user;
        if user.key() == participation.participant_address {
            // TODO: Check for validity of survey

            participation.completed = true;
        } else {
            // TODO: ERROR
        }
        Ok(())
    }
    pub fn claim_survey_reward(ctx: Context<SubmitSurveyAsParticipant>) -> Result<()> {
        let participation = &mut ctx.accounts.participation;
        let survey = &mut ctx.accounts.survey;
        let user = &mut ctx.accounts.user;
        if user.key() == participation.participant_address {
            // TODO: Check for survey completed status

            // TODO: release Funds

            // Mark reward status to true
            participation.rewarded = true;
        } else {
            // TODO: ERROR
        }
        Ok(())
    }
    pub fn sign_up_user(
        ctx: Context<SignUpUser>,
        bump: u8,
        first_name: String,
        last_name: String,
        email: String,
        profile_pic: String,
    ) -> Result<()> {
        let program_state = &mut ctx.accounts.program_state;
        let user_account = &mut ctx.accounts.user_account;
        let fund_locker = &mut ctx.accounts.fund_locker;
        let user = &mut ctx.accounts.user;
        msg!("Sign up user {}",first_name);
        user_account.id = program_state.user_count;
        user_account.wallet_address = user.key();
        user_account.first_name = first_name;
        user_account.last_name = last_name;
        user_account.email = email;
        user_account.profile_pic = profile_pic;
        user_account.survey_created_count = 0;
        user_account.survey_attempted_count = 0;
        user_account.total_reward_earned = 0;
        user_account.total_amount_spent = 0;
        user_account.funds_locker_address = fund_locker.key();

        program_state.user_count += 1;
        Ok(())
    }
    pub fn upgrade_tier(ctx: Context<UpgradeTier>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer=owner,
        space=8+std::mem::size_of::<ProgramState>(),
        seeds=[b"state".as_ref()],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct CreateSurvey<'info> {
    #[account(
        mut,
        seeds=[b"state".as_ref()],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,

    #[account(
        init,
        space=8+std::mem::size_of::<Survey>(),
        payer=creator,
        seeds=[
            b"survey".as_ref(), 
            program_state.survey_count.to_be_bytes().as_ref()
        ],
        bump
    )]
    pub survey: Account<'info, Survey>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EditSurvey<'info> {
    #[account(
        mut,
        seeds=[b"survey".as_ref(), survey.id.to_be_bytes().as_ref()],
        bump
    )]
    pub survey: Account<'info, Survey>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(user_id: u64)]
pub struct ParticipateSurvey<'info> {
    #[account(
        init,
        payer=user,
        space=8+std::mem::size_of::<Participation>(),
        seeds=[
            b"participation".as_ref(), 
            user_id.to_be_bytes().as_ref(),
            survey.id.to_be_bytes().as_ref()
        ],
        bump
    )]
    pub participation: Account<'info, Participation>,

    #[account(
        mut,
        seeds=[b"survey".as_ref(), survey.id.to_be_bytes().as_ref()],
        bump
    )]
    pub survey: Account<'info, Survey>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(user_id: u64)]
pub struct SubmitSurveyAsParticipant<'info> {
    #[account(
        mut,
        seeds=[
            b"participation".as_ref(), 
            user_id.to_be_bytes().as_ref(),
            survey.id.to_be_bytes().as_ref()
        ],
        bump
    )]
    pub participation: Account<'info, Participation>,

    #[account(
        mut,
        seeds=[b"survey".as_ref(), survey.id.to_be_bytes().as_ref()],
        bump
    )]
    pub survey: Account<'info, Survey>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct SignUpUser<'info> {
    #[account(
        mut,
        seeds=[b"state".as_ref()],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,

    #[account(
        init,
        payer=user,
        space=8+std::mem::size_of::<User>(),
        seeds=[
            b"user".as_ref(),
            user.key.as_ref()
        ],
        bump
    )]
    pub user_account: Account<'info, User>,

    #[account(
        init,
        payer=user,
        space=8+std::mem::size_of::<FundLocker>(),
        seeds=[
            b"locker".as_ref(),
            program_state.user_count.to_be_bytes().as_ref()
        ],
        bump
    )]
    pub fund_locker: Account<'info, FundLocker>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpgradeTier {}

#[account]
pub struct ProgramState {
    pub owner: Pubkey,
    pub survey_count: u64,
    pub user_count: u64,
}

#[account]
pub struct Survey {
    pub id: u64,
    pub creator: Pubkey,
    pub max_participants_count: u64,
    pub current_participants_count: u64,
    pub reward_per_participant: u64,
    pub valid_until: u64,
    pub is_draft: bool,
    pub is_active: bool,
    pub form_uri: String,
}

#[account]
pub struct User {
    pub id: u64,
    pub wallet_address: Pubkey,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub profile_pic: String,
    pub survey_created_count: u64,
    pub survey_attempted_count: u64,
    pub total_reward_earned: u64,
    pub total_amount_spent: u64,
    pub funds_locker_address: Pubkey,
}

#[account]
pub struct Participation {
    pub id: u64,
    pub participant_address: Pubkey,
    pub user_id: u64,
    pub survey_id: u64,
    pub completed: bool,
    pub rewarded: bool,
}

#[account]
pub struct FundLocker {
    pub funds: u64,
    pub lock_date: u64,
    pub release_date: u64,
    pub user_id: u64,
}
