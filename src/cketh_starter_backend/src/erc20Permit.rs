// SPDX-License-Identifier: MIT

use openzeppelin_rs::token::erc20::{ERC20, IERC20Permit};
use openzeppelin_rs::utils::{EIP712, Nonces};
use ethers::core::types::{Address, U256};
use ethers::prelude::*;
use std::convert::TryInto;

#[derive(Default)]
pub struct ERC20Permit {
    erc20: ERC20,
    eip712: EIP712,
    nonces: Nonces,
}

impl ERC20Permit {
    const PERMIT_TYPEHASH: [u8; 32] = [
        0x6c, 0x1a, 0x8e, 0x7b, 0x68, 0x1f, 0x4e, 0xf5,
        0x8c, 0x3a, 0x7d, 0xd2, 0x3f, 0xa5, 0x8f, 0xe5,
        0x4e, 0xd3, 0xd9, 0xe2, 0xa4, 0xb1, 0xc5, 0xb4,
        0xd4, 0xa5, 0xe2, 0xa3, 0xe9, 0x8b, 0xc1, 0x82,
    ];

    pub fn new(name: String) -> Self {
        let eip712 = EIP712::new(name.clone(), "1");
        Self {
            erc20: ERC20::new(name),
            eip712,
            nonces: Nonces::default(),
        }
    }

    pub fn permit(
        &mut self,
        owner: Address,
        spender: Address,
        value: U256,
        deadline: u64,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    ) -> Result<(), String> {
        if block_timestamp() > deadline {
            return Err(format!("ERC2612ExpiredSignature({})", deadline));
        }

        let nonce = self.nonces.nonces(owner);
        let struct_hash = keccak256(&[
            &Self::PERMIT_TYPEHASH,
            &owner.to_fixed_bytes(),
            &spender.to_fixed_bytes(),
            &value.to_fixed_bytes(),
            &nonce.to_fixed_bytes(),
            &deadline.to_le_bytes(),
        ]);

        let hash = self.eip712.hash_typed_data(struct_hash);
        
        let signer = ecdsa_recover(hash.clone(), v.into(), r.clone(), s.clone())
            .map_err(|_| format!("ERC2612InvalidSigner({}, {})", owner, signer))?;

        if signer != owner {
            return Err(format!("ERC2612InvalidSigner({}, {})", signer, owner));
        }

        self.erc20.approve(owner.clone(), spender.clone(), value)?;
        
        Ok(())
    }

    pub fn nonces(&self, owner: Address) -> U256 {
        self.nonces.nonces(owner)
    }

    pub fn domain_separator(&self) -> [u8; 32] {
        self.eip712.domain_separator()
    }
}

// Utility function to get the current block timestamp
fn block_timestamp() -> u64 {
    // Implementation to retrieve the current block timestamp
}

// Function to recover the address from the signature
fn ecdsa_recover(hash: [u8; 32], v: u8, r: [u8; 32], s: [u8; 32]) -> Result<Address, ()> {
    // Implementation for ECDSA signature recovery
}