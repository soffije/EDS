# EDS

This code implements the Elliptic Curve Digital Signature Algorithm (ECDSA) using the `elliptic-curve` and `sha2` crates in Rust. It provides functions for generating a key pair, hashing a message, signing a message, verifying a digital signature, and serializing/deserializing private keys, public keys, and signatures. The implementation uses the secp256k1 elliptic curve and the SHA-256 hash function.

Principle of Operation:
1. The code defines types for private keys, public keys, and signatures using the `elliptic-curve` crate.
2. The `generate_key_pair` function generates a random private key and derives the corresponding public key.
3. The `hash_message` function hashes a given message using the SHA-256 algorithm and returns a 32-byte hash value.
4. The `sign_message` function signs a message using a private key by hashing the message and computing the signature.
5. The `verify_signature` function verifies the validity of a digital signature by hashing the message and checking if the signature matches the public key.
6. The `serialize_private_key` function converts a private key to bytes and encodes them as a hexadecimal string.
7. The `deserialize_private_key` function decodes a serialized private key from a hexadecimal string and converts it back to a private key.
8. The `serialize_public_key` function converts a public key to bytes and encodes them as a hexadecimal string.
9. The `deserialize_public_key` function decodes a serialized public key from a hexadecimal string and converts it back to a public key.
10. The `serialize_signature` function converts a signature to bytes and encodes them as a hexadecimal string.
11. The `deserialize_signature` function decodes a serialized signature from a hexadecimal string and converts it back to a signature.
12. The `main` function demonstrates the usage of the implemented functions by generating a key pair, signing a message, verifying the signature, and performing serialization/deserialization of keys and signatures.

The tests check various aspects of the functions: key pair generation, message signing and verification, serialization and deserialization of the private key, public key, and digital signature. They make sure that the implemented functions behave correctly and return the expected results.
