#![no_std]
use soroban_sdk::{contract, contractimpl,contracttype, symbol_short, Address, Env, Symbol, Vec};

// Storage Keys
const NFT_OWNER: Symbol = symbol_short!("OWNER");
const RENTAL_INFO: Symbol = symbol_short!("RENT");

// Rental Struct
#[derive(Clone)]
#[contracttype]
pub struct Rental {
    pub renter: Address,
    pub expires_at: u64,
}

#[contract]
pub struct NFTRentalContract;

#[contractimpl]
impl NFTRentalContract {

    // Mint NFT (simple ownership assignment)
    pub fn mint(env: Env, to: Address, token_id: u64) {
        to.require_auth();
        env.storage().instance().set(&(NFT_OWNER, token_id), &to);
    }

    // Rent NFT
    pub fn rent(
        env: Env,
        owner: Address,
        renter: Address,
        token_id: u64,
        duration: u64,
        current_time: u64,
    ) {
        owner.require_auth();

        let stored_owner: Address = env
            .storage()
            .instance()
            .get(&(NFT_OWNER, token_id))
            .unwrap();

        if stored_owner != owner {
            panic!("Not owner");
        }

        let rental = Rental {
            renter: renter.clone(),
            expires_at: current_time + duration,
        };

        env.storage().instance().set(&(RENTAL_INFO, token_id), &rental);
    }

    // Get NFT Owner
    pub fn get_owner(env: Env, token_id: u64) -> Address {
        env.storage().instance().get(&(NFT_OWNER, token_id)).unwrap()
    }

    // Get Rental Info
    pub fn get_rental(env: Env, token_id: u64) -> Option<Rental> {
        env.storage().instance().get(&(RENTAL_INFO, token_id))
    }

    // End Rental (after expiry or manually)
    pub fn end_rental(env: Env, owner: Address, token_id: u64) {
        owner.require_auth();

        let stored_owner: Address = env
            .storage()
            .instance()
            .get(&(NFT_OWNER, token_id))
            .unwrap();

        if stored_owner != owner {
            panic!("Not owner");
        }

        env.storage().instance().remove(&(RENTAL_INFO, token_id));
    }
}