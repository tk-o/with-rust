#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        use recrypt::api::Plaintext;
        use aes_gcm::{Aes256Gcm, Key, Nonce};
        use aes_gcm::aead::{Aead, NewAead};
        use recrypt::prelude::*;
        use std::fs;

        // START: data encryption
        // load data to that is going to be shared as a message
        let message = fs::read_to_string("input/raw-data-provided-via-subscription.json")
            .expect("A file with raw data must exist");

        let key = Key::from_slice(b"an example very very secret key.");
        let cipher = Aes256Gcm::new(key);

        let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

        let ciphertext = cipher.encrypt(nonce, message.as_bytes().as_ref())
            .expect("encryption failure!"); // NOTE: handle this error to avoid panics!

        let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
            .expect("decryption failure!"); // NOTE: handle this error to avoid panics!

        assert_eq!(&plaintext, message.as_bytes());
        // END: data encryption

        // START: key encryption

        let mut recrypt = Recrypt::new();

// generate a plaintext to encrypt
        let pt = recrypt.gen_plaintext();

// generate signing keys
        let signing_keypair= recrypt.generate_ed25519_key_pair();

// generate a public/private keypair to encrypt the data to initially.
        let (initial_priv_key, initial_pub_key) = recrypt.generate_key_pair().unwrap();

// encrypt the data to `initial_pub_key`!
        let encrypted_val = recrypt.encrypt(&pt, &initial_pub_key, &signing_keypair).unwrap();

// generate a second public/private keypair as the target of the transform.
// after applying the transform, `target_priv_key` will be able to decrypt the data!
        let (target_priv_key, target_pub_key) = recrypt.generate_key_pair().unwrap();

// generate a transform key that will change which private key can decrypt the data
        let initial_to_target_transform_key = recrypt.generate_transform_key(
            &initial_priv_key,
            &target_pub_key,
            &signing_keypair).unwrap();

// Transform the plaintext to be encrypted to the target!
// The data is _not_ decrypted here. Simply transformed!
        let transformed_val = recrypt.transform(
            encrypted_val,
            initial_to_target_transform_key,
            &signing_keypair).unwrap();

// decrypt the transformed value with the target private key and recover the plaintext
        let decrypted_val = recrypt.decrypt(transformed_val, &target_priv_key).unwrap();

// plaintext recovered.
        assert_eq!(pt, decrypted_val);


    }
}
