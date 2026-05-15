// Complete Bounty contract implementation
// Addresses issue #1

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec, Map};

#[derive(Clone)]
pub struct Bounty {
    pub id: Symbol,
    pub creator: Address,
    pub amount: i128,
    pub description: Symbol,
    pub claimant: Option<Address>,
    pub is_resolved: bool,
}

#[contract]
pub struct BountyContract;

#[contractimpl]
impl BountyContract {
    pub fn create_bounty(env: Env, creator: Address, amount: i128, description: Symbol) -> Symbol {
        creator.require_auth();
        let id = Symbol::new(&env, &format!("bounty_{}", env.ledger().sequence()));
        let bounty = Bounty {
            id: id.clone(),
            creator,
            amount,
            description,
            claimant: None,
            is_resolved: false,
        };
        env.storage().persistent().set(&id.clone(), &bounty);
        id
    }

    pub fn claim_bounty(env: Env, id: Symbol, claimant: Address) {
        claimant.require_auth();
        let mut bounty: Bounty = env.storage().persistent().get(&id).unwrap();
        assert!(!bounty.is_resolved, "Bounty already resolved");
        assert!(bounty.claimant.is_none(), "Bounty already claimed");
        bounty.claimant = Some(claimant);
        env.storage().persistent().set(&id, &bounty);
    }

    pub fn resolve_bounty(env: Env, id: Symbol) {
        let mut bounty: Bounty = env.storage().persistent().get(&id).unwrap();
        bounty.creator.require_auth();
        assert!(bounty.claimant.is_some(), "No claimant");
        bounty.is_resolved = true;
        env.storage().persistent().set(&id, &bounty);
    }

    pub fn get_bounty(env: Env, id: Symbol) -> Bounty {
        env.storage().persistent().get(&id).unwrap()
    }
}
