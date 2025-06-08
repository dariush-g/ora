#[cfg(test)]
mod tests {
    use chat::message::{self, dh::generate_sent_key, encrypt::DHUser};

    #[test]
    fn test_dh_encryption() {
        let p = message::dh::parse_prime();
        let g = message::dh::G.clone();

        let alice = DHUser::new();
        let bob = DHUser::new();

        let shared_key_alice = generate_sent_key(&alice.get_private_key(), &g, &p);
        let shared_key_bob = generate_sent_key(&bob.get_private_key(), &g, &p);

        let secret_key_alice =
            message::dh::compute_shared_key(&shared_key_bob, &alice.get_private_key(), &p);
        let secret_key_bob =
            message::dh::compute_shared_key(&shared_key_alice, &bob.get_private_key(), &p);
        assert_eq!(secret_key_bob, secret_key_alice);
    }
}
