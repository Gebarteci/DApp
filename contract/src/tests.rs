#[cfg(test)]
mod tests {
    use multiversx_sc::types::Address;
    use multiversx_sc_scenario::{rust_biguint, managed_address, managed_biguint, DebugApi};
    use multiversx_sc_scenario::testing_framework::*;
    use escrow_contract::{OfferStatus, EscrowContract};

    const WASM_PATH: &'static str = "output/escrow-contract.wasm";

    struct ContractSetup<ContractObjBuilder>
    where
        ContractObjBuilder: 'static + Copy + Fn() -> escrow_contract::ContractObj<DebugApi>,
    {
        pub blockchain_wrapper: BlockchainStateWrapper,
        pub owner_address: Address,
        pub buyer_address: Address,
        pub contract_wrapper: ContractObjWrapper<escrow_contract::ContractObj<DebugApi>, ContractObjBuilder>,
    }

    fn setup_contract<ContractObjBuilder>(
        builder: ContractObjBuilder,
    ) -> ContractSetup<ContractObjBuilder>
    where
        ContractObjBuilder: 'static + Copy + Fn() -> escrow_contract::ContractObj<DebugApi>,
    {
        let rust_zero = rust_biguint!(0u64);
        let mut blockchain_wrapper = BlockchainStateWrapper::new();
        
        // Give accounts initial balance of 1000 EGLD
        let owner_address = blockchain_wrapper.create_user_account(&rust_biguint!(1_000_000_000_000_000_000u64));
        let buyer_address = blockchain_wrapper.create_user_account(&rust_biguint!(1_000_000_000_000_000_000u64));
        
        let contract_wrapper = blockchain_wrapper.create_sc_account(
            &rust_zero,
            Some(&owner_address),
            builder,
            WASM_PATH,
        );

        blockchain_wrapper
            .execute_tx(&owner_address, &contract_wrapper, &rust_zero, |sc| {
                sc.init();
            })
            .assert_ok();

        ContractSetup {
            blockchain_wrapper,
            owner_address,
            buyer_address,
            contract_wrapper,
        }
    }

    #[test]
    fn test_create_offer() {
        let mut setup = setup_contract(escrow_contract::contract_obj);
        
        setup.blockchain_wrapper
            .execute_tx(
                &setup.owner_address,
                &setup.contract_wrapper,
                &rust_biguint!(500_000_000_000_000_000u64), // 0.5 EGLD
                |sc| {
                    sc.create(managed_address!(&setup.buyer_address));
                },
            )
            .assert_ok();

        setup.blockchain_wrapper
            .execute_query(&setup.contract_wrapper, |sc| {
                let offer = sc.offer(1).get();
                assert!(matches!(offer.status, OfferStatus::Active));
                assert_eq!(offer.offer_id, 1);
                assert_eq!(offer.amount, managed_biguint!(500_000_000_000_000_000u64));
                assert_eq!(offer.creator, managed_address!(&setup.owner_address));
                assert_eq!(offer.recipient, managed_address!(&setup.buyer_address));
            })
            .assert_ok();
    }

    #[test]
    fn test_cancel_offer() {
        let mut setup = setup_contract(escrow_contract::contract_obj);
        
        setup.blockchain_wrapper
            .execute_tx(
                &setup.owner_address,
                &setup.contract_wrapper,
                &rust_biguint!(500_000_000_000_000_000u64), // 0.5 EGLD
                |sc| {
                    sc.create(managed_address!(&setup.buyer_address));
                },
            )
            .assert_ok();

        setup.blockchain_wrapper
            .execute_tx(
                &setup.owner_address,
                &setup.contract_wrapper,
                &rust_biguint!(0),
                |sc| {
                    sc.cancel_offer(1);
                },
            )
            .assert_ok();

        setup.blockchain_wrapper
            .execute_query(&setup.contract_wrapper, |sc| {
                let offer = sc.offer(1).get();
                assert!(matches!(offer.status, OfferStatus::Cancelled));
            })
            .assert_ok();
    }

    #[test]
    fn test_accept_offer() {
        let mut setup = setup_contract(escrow_contract::contract_obj);
        
        setup.blockchain_wrapper
            .execute_tx(
                &setup.owner_address,
                &setup.contract_wrapper,
                &rust_biguint!(500_000_000_000_000_000u64), // 0.5 EGLD
                |sc| {
                    sc.create(managed_address!(&setup.buyer_address));
                },
            )
            .assert_ok();

        setup.blockchain_wrapper
            .execute_tx(
                &setup.buyer_address,
                &setup.contract_wrapper,
                &rust_biguint!(0),
                |sc| {
                    sc.accept_offer(1);
                },
            )
            .assert_ok();

        setup.blockchain_wrapper
            .execute_query(&setup.contract_wrapper, |sc| {
                let offer = sc.offer(1).get();
                assert!(matches!(offer.status, OfferStatus::Completed));
            })
            .assert_ok();
    }
}