use elliptic_curve::Field;
use elliptic_curve::sec1::ToEncodedPoint;
use elliptic_curve::sec1::FromEncodedPoint;
use elliptic_curve::sec1::FromEncodedPoint;
use elliptic_curve::ops::Neg;

use sha2::{Digest, Sha256};
use rand::Rng;
use std::fmt;
use hex;

// Тип, який представляє особистий ключ
type PrivateKey = elliptic_curve::SecretKey<secp256k1::Curve>;

// Тип, який представляє відкритий ключ
type PublicKey = elliptic_curve::PublicKey<secp256k1::Curve>;

// Тип, який представляє цифровий підпис
type Signature = elliptic_curve::Signature<secp256k1::Curve>;

// Функція генерації пари ключів
fn generate_key_pair() -> (PrivateKey, PublicKey) {
    let private_key = PrivateKey::random(&mut rand::thread_rng());
    let public_key = private_key.public_key();
    (private_key, public_key)
}

// Функція гешування повідомлення
fn hash_message(message: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(message);
    let hash = hasher.finalize();
    let mut result = [0u8; 32];
    result.copy_from_slice(&hash[..]);
    result
}

// Функція підписання повідомлення
fn sign_message(message: &str, private_key: &PrivateKey) -> Signature {
    let hash = hash_message(message);
    private_key.sign(&hash[..])
}

// Функція перевірки цифрового підпису
fn verify_signature(message: &str, signature: &Signature, public_key: &PublicKey) -> bool {
    let hash = hash_message(message);
    public_key.verify(&hash[..], signature)
}

// Функція серіалізації особистого ключа
fn serialize_private_key(private_key: &PrivateKey) -> String {
    let bytes = private_key.to_bytes();
    hex::encode(bytes)
}

// Функція десеріалізації особистого ключа
fn deserialize_private_key(serialized_private_key: &str) -> Result<PrivateKey, hex::FromHexError> {
    let bytes = hex::decode(serialized_private_key)?;
    PrivateKey::from_bytes(&bytes)
}

// Функція серіалізації відкритого ключа
fn serialize_public_key(public_key: &PublicKey) -> String {
    let bytes = public_key.to_bytes();
    hex::encode(bytes)
}

// Функція десеріалізації відкритого ключа
fn deserialize_public_key(serialized_public_key: &str) -> Result<PublicKey, hex::FromHexError> {
    let bytes = hex::decode(serialized_public_key)?;
    PublicKey::from_bytes(&bytes)
}

// Функція серіалізації цифрового підпису
fn serialize_signature(signature: &Signature) -> String {
    let bytes = signature.as_ref().to_vec();
    hex::encode(bytes)
}

// Функція десеріалізації цифрового підпису
fn deserialize_signature(serialized_signature: &str) -> Result<Signature, hex::FromHexError> {
    let bytes = hex::decode(serialized_signature)?;
    Signature::from_bytes(&bytes)
}

fn main() {
    // Приклад використання функцій

    // Генерація пари ключів
    let (private_key, public_key) = generate_key_pair();
    println!("Private key: {:?}", private_key);
    println!("Public key: {:?}", public_key);

    // Підписання повідомлення
    let message = "Hello, world!";
    let signature = sign_message(message, &private_key);
    println!("Signature: {:?}", signature);

    // Перевірка цифрового підпису
    let is_valid = verify_signature(message, &signature, &public_key);
    println!("Is valid: {}", is_valid);

    // Серіалізація та десеріалізація особистого ключа
    let serialized_private_key = serialize_private_key(&private_key);
    println!("Serialized private key: {}", serialized_private_key);

    let deserialized_private_key = deserialize_private_key(&serialized_private_key);
    match deserialized_private_key {
        Ok(private_key) => println!("Deserialized private key: {:?}", private_key),
        Err(error) => println!("Error deserializing private key: {:?}", error),
    }

    // Серіалізація та десеріалізація відкритого ключа
    let serialized_public_key = serialize_public_key(&public_key);
    println!("Serialized public key: {}", serialized_public_key);

    let deserialized_public_key = deserialize_public_key(&serialized_public_key);
    match deserialized_public_key {
        Ok(public_key) => println!("Deserialized public key: {:?}", public_key),
        Err(error) => println!("Error deserializing public key: {:?}", error),
    }

    // Серіалізація та десеріалізація цифрового підпису
    let serialized_signature = serialize_signature(&signature);
    println!("Serialized signature: {}", serialized_signature);

    let deserialized_signature = deserialize_signature(&serialized_signature);
    match deserialized_signature {
        Ok(signature) => println!("Deserialized signature: {:?}", signature),
        Err(error) => println!("Error deserializing signature: {:?}", error),
    }
}
