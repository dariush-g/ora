use std::sync::LazyLock;

use num_bigint::BigUint;
use num_traits::Zero;
use rand::RngCore;

const P_HEX: &str = "FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B139B22514A08798E3404DDEF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3DC2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F83655D23DCA3AD961C62F356208552BB9ED529077096966D670C354E4ABC9804F1746C08CA18217C32905E462E36CE3BE39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9DE2BCBF6955817183560FB53B8A40F7C63A67E8BC1D0F5B9AC8DACEF1C07CBADC84CE2A34C7C4B1A34A0C4F27A60E1B6CC73D0A1AC2E74B6CD67EFFE2A4B8F2A3B4A2C7E9A3F894ECC50B2AE8A7926D3CAEF3D80F47B8B1E3DE9847B1C0F3F5E1D09EEE4C8F2F9F4C08CF9F4F7DC7D3A2E9E2F7A74D3F5B6E8F3F2B8B9F4F1C3E4D5C7A2B1E9F4C3A2D5B6E8F1C2A3B4D5E6F7A8B9C0D1E2F3A4B5C6D7E8F9A0B1C2D3E4F5A6B7C8D9E0F1A2B3C4D5E6F7A8B9C0D1E2F3A4B5C6D7E8F9A0B1C2D3E4F5A6B7C8D9E0F1A2B3C4D5E6F7A8B9C0D1E2F3";

pub(crate) static P: LazyLock<BigUint> =
    LazyLock::new(|| BigUint::parse_bytes(P_HEX.as_bytes(), 16).unwrap());
pub static G: LazyLock<BigUint> = LazyLock::new(|| BigUint::from(2u32));

pub fn parse_prime() -> BigUint {
    BigUint::parse_bytes(P_HEX.as_bytes(), 16).expect("invalid hex for P")
}

pub fn generate_private_key() -> BigUint {
    let p = &P;
    let mut rng = rand::rng();
    let mut random_bytes = vec![0u8; (p.bits() / 8 + 1) as usize]; // +1 for safety
    rng.fill_bytes(&mut random_bytes);

    let mut candidate = BigUint::from_bytes_be(&random_bytes);
    candidate %= (*p).clone();

    if candidate.is_zero() {
        return generate_private_key();
    }

    candidate
}

pub fn generate_sent_key(private: &BigUint) -> BigUint {
    G.modpow(private, &P)
}

pub fn compute_shared_key(their_shared: &BigUint, my_private: &BigUint, p: &BigUint) -> BigUint {
    their_shared.modpow(my_private, p)
}
