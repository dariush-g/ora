use num_bigint::BigUint;
use rand::RngCore;
use serde::{Deserialize, Serialize};

use crate::message::{
    content::MessageContent,
    dh::generate_private_key,
    sha256::{self, sha256},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    KeyRequest(BigUint),
    KeyResponse(BigUint),
    EncryptedMessage(EncryptedMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    pub encrypted_content: Vec<u8>,
    pub nonce: Vec<u8>,
}

impl EncryptedMessage {
    pub fn new(plaintext: MessageContent, secret: &BigUint) -> Self {
        let nonce = generate_nonce();
        let key = derive_key(secret, &nonce);
        Self {
            encrypted_content: xor_keystream_encrypt(&plaintext.get_content(), &key),
            nonce,
        }
    }

    pub fn get_encrypted_content(&self) -> &Vec<u8> {
        &self.encrypted_content
    }

    pub fn get_nonce(&self) -> &Vec<u8> {
        &self.nonce
    }

    pub fn xor_keystream_decrypt(&self, secret: &BigUint) -> MessageContent {
        let mut plaintext = Vec::with_capacity(self.encrypted_content.len());
        let key = derive_key(secret, &self.nonce);

        for (i, &byte) in self.encrypted_content.iter().enumerate() {
            let mut block = key.to_vec();

            block.extend_from_slice(&(i as u32).to_be_bytes());
            let keystream_block = sha256(&block);
            plaintext.push(byte ^ keystream_block[i % 32]);
        }

        MessageContent::Text(plaintext)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DHUser {
    private_key: BigUint,
}

impl DHUser {
    pub fn new() -> Self {
        Self {
            private_key: generate_private_key(),
        }
    }

    pub fn get_private_key(&self) -> &BigUint {
        &self.private_key
    }

    pub fn set_private_key(&mut self, private_key: BigUint) {
        self.private_key = private_key;
    }

    pub fn get_combined_key(&self, shared_key: BigUint) -> BigUint {
        shared_key.modpow(&self.private_key, &crate::message::dh::P)
    }
}

pub fn generate_nonce() -> Vec<u8> {
    let mut nonce = vec![0u8; 12];
    rand::rng().fill_bytes(&mut nonce);
    nonce
}

fn derive_key(shared_secret: &BigUint, nonce: &[u8]) -> [u8; 32] {
    let mut input = shared_secret.to_bytes_be();
    input.extend_from_slice(nonce);
    sha256(&input)
}

fn xor_keystream_encrypt(plaintext: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let mut ciphertext = Vec::with_capacity(plaintext.len());

    for (i, &byte) in plaintext.iter().enumerate() {
        let mut block = key.to_vec();
        block.extend_from_slice(&(i as u32).to_be_bytes());

        let keystream_block = sha256(&block);
        ciphertext.push(byte ^ keystream_block[i % 32]);
    }

    ciphertext
}
