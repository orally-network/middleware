use candid::Nat;
use ic_web3_rs::types::U256;

pub trait ToNativeTypes {
    fn to_u64(&self) -> u64;
    fn to_u32(&self) -> u32;
    fn to_u256(&self) -> U256;
}

impl ToNativeTypes for Nat {
    fn to_u64(&self) -> u64 {
        let nat_digits = self.0.to_u64_digits();
        let mut number: u64 = 0;
        if !nat_digits.is_empty() {
            number = *nat_digits.last().expect("nat should be a number");
        }
        number
    }

    fn to_u32(&self) -> u32 {
        let nat_digits = self.0.to_u32_digits();
        let mut number: u32 = 0;
        if !nat_digits.is_empty() {
            number = *nat_digits.last().expect("nat should be a number");
        }
        number
    }

    fn to_u256(&self) -> U256 {
        U256::from_big_endian(&self.0.to_bytes_be())
    }
}

pub trait ToNatType {
    fn to_nat(&self) -> Nat;
}

impl ToNatType for U256 {
    fn to_nat(&self) -> Nat {
        let mut buf = Vec::with_capacity(32);
        for i in self.0.iter().rev() {
            buf.extend(i.to_be_bytes());
        }

        Nat(num_bigint::BigUint::from_bytes_be(&buf))
    }
}
