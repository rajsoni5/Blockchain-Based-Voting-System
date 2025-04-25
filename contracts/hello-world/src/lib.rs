#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short};

// Stores details of a proposal
#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub vote_count: u64,
}

// Mapping Proposal ID
#[contracttype]
pub enum ProposalKey {
    Proposal(u64),
}

const PROPOSAL_COUNTER: Symbol = symbol_short!("P_COUNT");

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    // Function to create a new proposal
    pub fn create_proposal(env: Env, title: String, description: String) -> u64 {
        let mut proposal_count: u64 = env.storage().instance().get(&PROPOSAL_COUNTER).unwrap_or(0);
        proposal_count += 1;

        let new_proposal = Proposal {
            id: proposal_count,
            title,
            description,
            vote_count: 0,
        };

        env.storage().instance().set(&ProposalKey::Proposal(proposal_count), &new_proposal);
        env.storage().instance().set(&PROPOSAL_COUNTER, &proposal_count);
        env.storage().instance().extend_ttl(10000, 10000);

        proposal_count
    }

    // Function to vote for a proposal
    pub fn vote(env: Env, proposal_id: u64) {
        let key = ProposalKey::Proposal(proposal_id);
        let mut proposal: Proposal = env.storage().instance().get(&key).expect("Proposal not found");

        proposal.vote_count += 1;

        env.storage().instance().set(&key, &proposal);
        env.storage().instance().extend_ttl(10000, 10000);
    }

    // Function to get details of a proposal
    pub fn get_proposal(env: Env, proposal_id: u64) -> Proposal {
        let key = ProposalKey::Proposal(proposal_id);
        env.storage().instance().get(&key).expect("Proposal not found")
    }
}


