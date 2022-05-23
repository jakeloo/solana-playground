use anchor_lang::prelude::*;

declare_id!("AA91rmiK5gZBw1Gmdq75NBVwm7yeF67ti8T7PnwMNoZk");

#[error_code]
pub enum GameErrorCode {
  #[msg("Battle Not Started")]
  BattleNotStarted,
  #[msg("Battle Already Started")]
  BattleAlreadyStarted,
  #[msg("Player Not Init")]
  PlayerNotInit,
  #[msg("Player Already Init")]
  PlayerAlreadyInit,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Idle,
    Active,
    Won { winner: Pubkey },
}

#[account]
pub struct Equipment {
    attack: u64,
    defense: u64,
    bump: u8,
}

#[account]
pub struct Player {
    level: u64,
    exp: u64,
    hp: u64,
    mp: u64,
    bump: u8,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user,
        space = 8 + 8 + 8 + 8 + 8 + 1,
        seeds = [b"player", user.key().as_ref()],
        bump)]
    pub player: Account<'info, Player>,

    #[account(init, payer = user,
        space = 8 + 8 + 8 + 1,
        seeds = [b"equipment", user.key().as_ref()],
        bump)]
    pub equipment: Account<'info, Equipment>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetupBattle<'info> {
    #[account(init, payer = user,
        space = 8 + (32 * 2) + 1 + (32 + 1) + 1 + (32) * 2 + (32) * 2,
        seeds = [b"battle", user.key().as_ref(), user_two.key().as_ref()],
        bump)]
    pub battle: Account<'info, Battle>,

    #[account(seeds = [b"player", user.key().as_ref()], bump = player_one.bump)]
    pub player_one: Account<'info, Player>,
    #[account(seeds = [b"equipment", user.key().as_ref()], bump = equipment_one.bump)]
    pub equipment_one: Account<'info, Equipment>,

    #[account(seeds = [b"player", user_two.key().as_ref()], bump = player_two.bump)]
    pub player_two: Account<'info, Player>,
    #[account(seeds = [b"equipment", user_two.key().as_ref()], bump = equipment_two.bump)]
    pub equipment_two: Account<'info, Equipment>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: opponent
    pub user_two: AccountInfo<'info>,

    pub system_program: Program<'info, System>
}

#[account]
pub struct Battle {
    keys: [Pubkey; 2],
    players: [Pubkey; 2],
    equipments: [Pubkey; 2],
    turn: u8,
    state: GameState,
    bump: u8,
}

#[program]
pub mod sword_play {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        require_eq!(ctx.accounts.player.level, 0, GameErrorCode::PlayerAlreadyInit);

        let player = &mut ctx.accounts.player;

        player.level = 1;
        player.exp = 0;
        player.hp = 50;
        player.mp = 50;
        player.bump = *ctx.bumps.get("player").unwrap();

        let eq = &mut ctx.accounts.equipment;
        eq.attack = 15;
        eq.defense = 15;
        eq.bump = *ctx.bumps.get("equipment").unwrap();
        Ok(())
    }

    pub fn setup_battle(ctx: Context<SetupBattle>) -> Result<()> {
        require_neq!(ctx.accounts.player_one.level, 0, GameErrorCode::PlayerNotInit);
        require_neq!(ctx.accounts.player_two.level, 0, GameErrorCode::PlayerNotInit);
        ctx.accounts.battle.start(
            [ctx.accounts.user.key(), ctx.accounts.user_two.key()],
            [ctx.accounts.player_one.key(), ctx.accounts.player_two.key()],
            [ctx.accounts.equipment_one.key(), ctx.accounts.equipment_two.key()])
    }
}

impl Battle {
    pub fn start(&mut self, keys: [Pubkey; 2], players: [Pubkey; 2], eqs: [Pubkey; 2]) -> Result<()> {
        require!(self.state != GameState::Active, GameErrorCode::BattleAlreadyStarted);
        self.keys = keys;
        self.players = players;
        self.equipments = eqs;
        self.turn = 0;
        self.state = GameState::Active;
        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.state == GameState::Active
    }

    pub fn fight(&mut self) -> Result<()> {
        require!(self.state == GameState::Active, GameErrorCode::BattleNotStarted);
        // todo
        Ok(())
    }
}
