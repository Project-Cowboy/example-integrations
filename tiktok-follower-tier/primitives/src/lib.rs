// According to chatgpt, tiktok influencers are split into tiers. We'll define each one
#[derive(Debug)]
#[repr(u8)]
pub enum AccountType {
    /// Less than 1,000 followers.
    SmallAccount,
    /// 1,000–10,000 followers.
    Nano,
    /// 10,001–50,000 followers.
    Micro,
    /// 50,001–500,000 followers.
    MidTier,
    /// 500,001–1,000,000 followers.
    Macro, 
    /// 1,000,001+ followers.
    Mega,
}

impl TryFrom<u8> for AccountType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AccountType::SmallAccount),
            1 => Ok(AccountType::Nano),
            2 => Ok(AccountType::Micro),
            3 => Ok(AccountType::MidTier),
            4 => Ok(AccountType::Macro),
            5 => Ok(AccountType::Mega),
            _ => Err(()),
        }
    }
}


impl AccountType {
    // Return an influencer type based on the amount of followers the account has
    pub fn from_follower_count(follower_count: u32) -> Self {
        match follower_count {
            0..=999 => AccountType::SmallAccount,
            1000..=10_000 => AccountType::Nano,
            10_001..=50_000 => AccountType::Micro,
            50_001..=500_000 => AccountType::MidTier,
            500_001..=1_000_000 => AccountType::Macro,
            _ => AccountType::Mega,
        }
    }
}