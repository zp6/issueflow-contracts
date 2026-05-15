// Add bounty splitting
// Addresses issue #2

use soroban_sdk::{contractimpl, Address, Env, Symbol, Vec};

use crate::BountyContract;

#[derive(Clone, Debug)]
pub struct Split {
    pub claimant: Address,
    pub percentage: u32,
}

#[contractimpl]
impl BountyContract {
    pub fn split_bounty(
        env: Env,
        id: Symbol,
        splits: Vec<Split>,
    ) {
        let (bounty_id, creator, amount, description, deadline, _, is_resolved): (
            Symbol, Address, i128, Symbol, u64, Option<Address>, bool,
        ) = env.storage().persistent().get(&id).unwrap();

        creator.require_auth();
        assert!(is_resolved, "Bounty must be resolved");
        assert!(deadline > env.ledger().timestamp() || is_resolved, "Expired");

        let total_pct: u32 = splits.iter().map(|s| s.percentage).sum();
        assert!(total_pct == 100, "Splits must total 100%");

        for split in splits.iter() {
            let share = amount * (split.percentage as i128) / 100;
            env.storage().persistent().set(
                &Symbol::new(&env, &format!("split_{}_{}", id.to_string(), split.claimant.to_string())),
                &(split.claimant.clone(), share),
            );
        }
    }
}
