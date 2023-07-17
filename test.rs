#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key_pair() {
        let (private_key, public_key) = generate_key_pair();
        assert!(private_key.to_bytes().len() > 0);
        assert!(public_key.to_bytes().len() > 0);
    }

    #[test]
    fn test_sign_and_verify_message() {
        let (private_key, public_key) = generate_key_pair();
        let message = "Test message";

        let signature = sign_message(message, &private_key);
        assert!(verify_signature(message, &signature, &public_key));
    }

    #[test]
    fn test_serialize_and_deserialize_private_key() {
        let (private_key, _) = generate_key_pair();

        let serialized_private_key = serialize_private_key(&private_key);
        let deserialized_private_key = deserialize_private_key(&serialized_private_key).unwrap();

        assert_eq!(private_key, deserialized_private_key);
    }

    #[test]
    fn test_serialize_and_deserialize_public_key() {
        let (_, public_key) = generate_key_pair();

        let serialized_public_key = serialize_public_key(&public_key);
        let deserialized_public_key = deserialize_public_key(&serialized_public_key).unwrap();

        assert_eq!(public_key, deserialized_public_key);
    }

    #[test]
    fn test_serialize_and_deserialize_signature() {
        let (private_key, _) = generate_key_pair();
        let message = "Test message";

        let signature = sign_message(message, &private_key);

        let serialized_signature = serialize_signature(&signature);
        let deserialized_signature = deserialize_signature(&serialized_signature).unwrap();

        assert_eq!(signature, deserialized_signature);
    }
}
