#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use recrypt::prelude::*;

// create a new recrypt
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
