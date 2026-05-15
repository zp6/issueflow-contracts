// Implement bounty creation function
// Addresses issue #4

use soroban_sdk::{contractimpl, Address, Env, Symbol, BytesN};

use crate::BountyContract;

#[contractimpl]
impl BountyContract {
    pub fn create_bounty_with_deadline(
        env: Env,
        creator: Address,
        amount: i128,
        description: Symbol,
        deadline: u64,
    ) -> Symbol {
        creator.require_auth();
        assert!(amount > 0, "Amount must be positive");
        assert!(deadline > env.ledger().timestamp(), "Deadline must be in future");

        let id = Symbol::new(&env, &format!("b{}", env.ledger().sequence()));
        env.storage().persistent().set(&id.clone(), &(
            id.clone(),
            creator,
            amount,
            description,
            deadline,
            Option::<Address>::None,
            false,
        ));
        id
    }

    pub fn cancel_bounty(env: Env, id: Symbol) {
        let (.., creator, _, _, _, claimant, is_resolved): (
            Symbol, Address, i128, Symbol, u64, Option<Address>, bool,
        ) = env.storage().persistent().get(&id).unwrap();
        creator.require_auth();
        assert!(!is_resolved, "Already resolved");
        assert!(claimant.is_none(), "Has claimant");
        env.storage().persistent().remove(&id);
    }
}
