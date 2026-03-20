#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

// Storage Keys
const NFT_OWNER: Symbol = symbol_short!("OWNER");
const LISTING_INFO: Symbol = symbol_short!("LISTING");
const RENTAL_INFO: Symbol = symbol_short!("RENTAL");
const ESCROW_AGENT: Symbol = symbol_short!("ESCROW");

#[derive(Clone)]
#[contracttype]
pub struct Listing {
    pub lessor: Address,
    pub price_per_second: i128,
    pub max_duration_seconds: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct Rental {
    pub renter: Address,
    pub started_at: u64,
    pub expires_at: u64,
    pub paid_amount: i128,
}

#[derive(Clone)]
#[contracttype]
pub struct Escrow {
    pub agent: Address,
}

#[contract]
pub struct NFTRentalContract;

#[contractimpl]
impl NFTRentalContract {
    // Permissionless mint: anyone can mint a new token to any address.
    pub fn mint(env: Env, token_id: u64, to: Address) {
        if env.storage().instance().has(&(NFT_OWNER, token_id)) {
            panic!("Token already minted");
        }
        env.storage().instance().set(&(NFT_OWNER, token_id), &to);
    }

    // Permissionless transfer: only current owner can transfer by proof of ownership off-chain.
    pub fn transfer(env: Env, token_id: u64, from: Address, to: Address) {
        let current_owner: Address = env
            .storage()
            .instance()
            .get(&(NFT_OWNER, token_id))
            .unwrap_or_else(|| panic!("Token does not exist"));

        if current_owner != from {
            panic!("Transfer failed: not owner");
        }

        env.storage().instance().set(&(NFT_OWNER, token_id), &to);
        // If a listing existed, clear it (relist required by new owner)
        env.storage().instance().remove(&(LISTING_INFO, token_id));
        env.storage().instance().remove(&(RENTAL_INFO, token_id));
    }

    // Owner lists for rent; permissionless contract-level, but actual listing should be from active owner.
    pub fn list_for_rent(
        env: Env,
        token_id: u64,
        lessor: Address,
        price_per_second: i128,
        max_duration_seconds: u64,
    ) {
        let current_owner: Address = env
            .storage()
            .instance()
            .get(&(NFT_OWNER, token_id))
            .unwrap_or_else(|| panic!("Token does not exist"));

        if current_owner != lessor {
            panic!("Listing failed: lessor not owner");
        }

        if price_per_second <= 0 {
            panic!("Listing failed: price must be positive");
        }

        let listing = Listing {
            lessor,
            price_per_second,
            max_duration_seconds,
        };

        env.storage().instance().set(&(LISTING_INFO, token_id), &listing);
    }

    pub fn cancel_listing(env: Env, token_id: u64, lessor: Address) {
        let listing: Listing = env
            .storage()
            .instance()
            .get(&(LISTING_INFO, token_id))
            .unwrap_or_else(|| panic!("Listing does not exist"));

        if listing.lessor != lessor {
            panic!("Cancel failed: not listing owner");
        }

        env.storage().instance().remove(&(LISTING_INFO, token_id));
    }

    // Rent a listed NFT. permissionless to call; owner face-value enforcement by on-chain state.
    pub fn rent(
        env: Env,
        token_id: u64,
        renter: Address,
        duration_seconds: u64,
        current_time: u64,
        paid_amount: i128,
    ) {
        let listing: Listing = env
            .storage()
            .instance()
            .get(&(LISTING_INFO, token_id))
            .unwrap_or_else(|| panic!("No active listing"));

        if duration_seconds == 0 || duration_seconds > listing.max_duration_seconds {
            panic!("Invalid rental duration");
        }

        if paid_amount != listing.price_per_second * (duration_seconds as i128) {
            panic!("Incorrect payment amount");
        }

        let maybe_rental: Option<Rental> = env.storage().instance().get(&(RENTAL_INFO, token_id));
        if let Some(active_rental) = maybe_rental {
            if active_rental.expires_at > current_time {
                panic!("Token already rented");
            }
        }

        let rental = Rental {
            renter,
            started_at: current_time,
            expires_at: current_time + duration_seconds,
            paid_amount,
        };
        env.storage().instance().set(&(RENTAL_INFO, token_id), &rental);
    }

    // Escrow mechanism optional, explicitly chosen by user.
    pub fn set_escrow_agent(env: Env, token_id: u64, agent: Address) {
        let owner: Address = env
            .storage()
            .instance()
            .get(&(NFT_OWNER, token_id))
            .unwrap_or_else(|| panic!("Token does not exist"));

        if owner != agent {
            // optional: only owner can set escrow agent; permissioned feature
            env.storage().instance().set(&(ESCROW_AGENT, token_id), &Escrow { agent });
        } else {
            panic!("Escrow agent cannot be owner")
        }
    }

    pub fn get_owner(env: Env, token_id: u64) -> Address {
        env.storage().instance().get(&(NFT_OWNER, token_id)).unwrap_or_else(|| panic!("Token does not exist"))
    }

    pub fn get_listing(env: Env, token_id: u64) -> Option<Listing> {
        env.storage().instance().get(&(LISTING_INFO, token_id))
    }

    pub fn get_rental(env: Env, token_id: u64) -> Option<Rental> {
        env.storage().instance().get(&(RENTAL_INFO, token_id))
    }

    pub fn get_escrow_agent(env: Env, token_id: u64) -> Option<Escrow> {
        env.storage().instance().get(&(ESCROW_AGENT, token_id))
    }

    pub fn end_rental(env: Env, token_id: u64, current_time: u64) {
        let maybe_rental: Option<Rental> = env.storage().instance().get(&(RENTAL_INFO, token_id));
        if let Some(active_rental) = maybe_rental {
            if active_rental.expires_at <= current_time {
                env.storage().instance().remove(&(RENTAL_INFO, token_id));
            } else {
                panic!("Rental not expired yet");
            }
        } else {
            panic!("No active rental");
        }
    }
}

#[cfg(test)]
mod test;
