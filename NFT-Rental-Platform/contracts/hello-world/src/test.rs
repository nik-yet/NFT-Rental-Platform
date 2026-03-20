#![cfg(test)]

use super::*;
use soroban_sdk::{Address, Env};

fn sample_address(env: &Env, idx: u8) -> Address {
    let base = match idx {
        1 => "GBRPYHIL2CI3YQ4LC2RR7YQUIQXFM4N65UWQHX5XRRJPAU3ATIB5L6JFD",
        2 => "GA6HCMBLTZS5VFS57XKKANJ2YJMEG5J2SW2YZT6C7UZ5QN2BWGUSMIXR4",
        3 => "GBZCULV5F4YMO4QZ4L3CZQL4U6DAOIWULVOSJTHYM4OU5NMHJETM6Y6KE",
        _ => "GBRPYHIL2CI3YQ4LC2RR7YQUIQXFM4N65UWQHX5XRRJPAU3ATIB5L6JFD",
    };
    Address::from_str(env, base)
}

#[test]
fn test_permissionless_rental_flow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, NFTRentalContract);
    let client = NFTRentalContractClient::new(&env, &contract_id);

    let owner = sample_address(&env, 1);
    let renter = sample_address(&env, 2);

    client.mint(&1, &owner);
    assert_eq!(client.get_owner(&1), owner);

    client.list_for_rent(&1, &owner, &10, &1000);
    let listing = client.get_listing(&1).unwrap();
    assert_eq!(listing.lessor, owner);
    assert_eq!(listing.price_per_second, 10);

    client.rent(&1, &renter, &10, &100, &1000);
    let rental = client.get_rental(&1).unwrap();
    assert_eq!(rental.renter, renter);
    assert_eq!(rental.started_at, 1000);
    assert_eq!(rental.expires_at, 1010);

    client.end_rental(&1, &1010);
    assert!(client.get_rental(&1).is_none());

    let new_owner = sample_address(&env, 3);
    client.transfer(&1, &owner, &new_owner);
    assert_eq!(client.get_owner(&1), new_owner);
}

#[test]
#[should_panic(expected = "Rental not expired yet")]
fn test_end_rental_before_expiry_panics() {
    let env = Env::default();
    let contract_id = env.register_contract(None, NFTRentalContract);
    let client = NFTRentalContractClient::new(&env, &contract_id);

    let owner = sample_address(&env, 1);
    let renter = sample_address(&env, 2);

    client.mint(&1, &owner);
    client.list_for_rent(&1, &owner, &10, &1000);
    client.rent(&1, &renter, &10, &100, &1000);
    client.end_rental(&1, &1005);
}

