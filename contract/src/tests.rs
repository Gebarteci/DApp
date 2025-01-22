#![cfg(test)]

use multiversx_sc::testing::*;
use multiversx_sc::codec::multi_value::MultiValueEncoded;
use escrow_contract::*; // Import the crate

#[test]
fn test_create_offer() {
    let mut sc_harness = ScHarness::new_sc(EscrowContract::new());
    let owner_address = sc_harness.sc_address();
    let buyer_address = Address::from_hex("0100000000000000000000000000000000000000000000000000000000000000");
    let initial_balance = 1000u64;
    let offer_amount = 500u64;

    sc_harness.set_value(initial_balance, &BigUint::from(initial_balance));

    sc_harness.sc_call(
        &owner_address,
        "create",
        &vec![&buyer_address],
        &BigUint::from(offer_amount),
    );

    let offer_wrapper: MultiValueEncoded<Offer<sc_harness::Api>> = sc_harness.query("getOffer", &vec![&1u64]);
    let offer = offer_wrapper.get(0);
    assert_eq!(offer.offer_id, 1);
    assert_eq!(offer.status, OfferStatus::Active);
    assert_eq!(offer.amount, BigUint::from(offer_amount));
    assert_eq!(offer.creator, owner_address);
    assert_eq!(offer.recipient, buyer_address);
    assert_eq!(sc_harness.get_account(&owner_address).balance, BigUint::from(initial_balance - offer_amount));

}

#[test]
fn test_cancel_offer(){
    let mut sc_harness = ScHarness::new_sc(EscrowContract::new());
    let owner_address = sc_harness.sc_address();
    let buyer_address = Address::from_hex("0100000000000000000000000000000000000000000000000000000000000000"); // Example test address
    let initial_balance = 1000u64;
    let offer_amount = 500u64;

    sc_harness.set_value(initial_balance, &BigUint::from(initial_balance));

    sc_harness.sc_call(
        &owner_address,
        "create",
        &vec![&buyer_address],
        &BigUint::from(offer_amount),
    );
    sc_harness.sc_call(
        &owner_address,
        "cancelOffer",
        &vec![&1u64],
        &BigUint::from(0u64),
    );
    let offer_wrapper: MultiValueEncoded<Offer<sc_harness::Api>> = sc_harness.query("getOffer", &vec![&1u64]);
    let offer = offer_wrapper.get(0);
    assert_eq!(offer.status, OfferStatus::Cancelled);
    assert_eq!(sc_harness.get_account(&owner_address).balance, BigUint::from(initial_balance));

}
#[test]
fn test_accept_offer(){
    let mut sc_harness = ScHarness::new_sc(EscrowContract::new());
    let owner_address = sc_harness.sc_address();
    let buyer_address = Address::from_hex("0100000000000000000000000000000000000000000000000000000000000000"); // Example test address
    let initial_balance = 1000u64;
    let offer_amount = 500u64;

    sc_harness.set_value(initial_balance, &BigUint::from(initial_balance));

    sc_harness.sc_call(
        &owner_address,
        "create",
        &vec![&buyer_address],
        &BigUint::from(offer_amount),
    );
    sc_harness.set_value(0, &BigUint::from(0u64), &buyer_address);
    sc_harness.sc_call(
        &buyer_address,
        "acceptOffer",
        &vec![&1u64],
        &BigUint::from(0u64),
    );
    let offer_wrapper: MultiValueEncoded<Offer<sc_harness::Api>> = sc_harness.query("getOffer", &vec![&1u64]);
    let offer = offer_wrapper.get(0);
    assert_eq!(offer.status, OfferStatus::Completed);
    assert_eq!(sc_harness.get_account(&buyer_address).balance, BigUint::from(offer_amount));
    assert_eq!(sc_harness.get_account(&owner_address).balance, BigUint::from(initial_balance-offer_amount));

}