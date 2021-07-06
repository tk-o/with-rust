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


        // generate Data Encryption Key (DEK)
        let dek = Key::from_slice(b"an example very very secret key.");
        // encrypt data
        let cipher = Aes256Gcm::new(dek);
        let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

        let ciphertext = cipher.encrypt(nonce, message.as_bytes().as_ref())
            .expect("encryption failure!"); // NOTE: handle this error to avoid panics!

        // validate the AES implementation
        let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
            .expect("decryption failure!"); // NOTE: handle this error to avoid panics!

        assert_eq!(String::from_utf8(plaintext).unwrap(), message);
        // END: data encryption

        // START: DEK encryption

        let mut recrypt = Recrypt::new();

        println!("DEK (len: {:?}): {:?}", dek.len(), dek);
        // generate a plaintext to encrypt
        // it's going to represent a message `m`
        // TODO: make sure to pass both: the DEK and the nonce here
        //       as these are needed to decrypt AES-ecrypted data
        let pt = recrypt.gen_plaintext();

        // ALICE: generates a public/private keypair to encrypt the message `m` initially
        let (alice_priv_key, alice_pub_key) = recrypt.generate_key_pair().unwrap();

        // ENRICO: generate signing keys
        let enrico_keypair = recrypt.generate_ed25519_key_pair();

        // Enrico encrypts a message `m` to `pk_A`
        // c_A = encrypt(pk_A, m)
        // where the Enrico's key pair sits then?
        let encrypted_val = recrypt.encrypt(&pt, &alice_pub_key, &enrico_keypair).unwrap();

        // BOB: generate a second public/private keypair as the target of the transform.
        // after applying the transform, `sk_B` will be able to decrypt the data!
        let (bob_priv_key, bob_pub_key) = recrypt.generate_key_pair().unwrap();

        // Alice generates a transform key that will change which private key can decrypt the data
        // rk_A->B
        let alice_to_bob_transform_key = recrypt.generate_transform_key(
            &alice_priv_key,
            &bob_pub_key,
            &enrico_keypair
        ).unwrap();

        // URSULA: Transform the plaintext to be encrypted to the target!
        // The data is _not_ decrypted here. Simply transformed!
        // c_B = re-encrypt(rk_A->B, c_A)
        let transformed_val = recrypt.transform(
            encrypted_val,
            alice_to_bob_transform_key,
            &enrico_keypair
        ).unwrap();

        // TODO: understand how does Urusla know the signing keypair of Enrico? I'd mean Enrico revealing his private key!
        // ???????

        // Bob decrypts the transformed value with the his `pk_B` and gets the message `m`
        // m = decrypt(sk_B, c_B)
        let decrypted_val = recrypt.decrypt(
            transformed_val,
            &bob_priv_key
        ).unwrap();

        // original message `m` recovered.
        assert_eq!(pt, decrypted_val);
    }
}
