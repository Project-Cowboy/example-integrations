use risc0_zkvm::{guest::env, serde};
use sha2::{Sha256, Digest};

// The following code's execution trace will be proven with a zk stark
fn main() {
    // Receive journal, for verifying the core proof of the tls notary signatures
    let journal: [u8; 32] = env::read();
    // Receive the entire TLS Notary representation
    // *SECURITY* TODO: Hash this with accelerated sha2 and assert it's equal to journal's value
    let tlsn_representation_bytes: Vec<u8> = env::read();

    // Program id for core tlsn verifier 
    let tlsn_prover_id = [1997738335, 3678040518, 3768798354, 2157753599, 1334048300, 48065757, 3864380607, 3289501451];

    // Verify previous proof of our tls notary proof validity
    env::verify(tlsn_prover_id, &serde::to_vec(&journal).unwrap()).unwrap();

    env::log("Guest: Core proof verified");

    // Get value of `screen_name` key
    let screen_name_bytes = extract_value_from_key(&tlsn_representation_bytes, "screen_name").unwrap();

    env::log(&format!("Guest: debug bytes: {:?} ", screen_name_bytes));

    let screen_name = String::from_utf8(screen_name_bytes.clone()).unwrap();

    env::log(&format!("Guest: Extracted screen name: {:?} ", screen_name));

    let cool_keywords = ["zk", "0x", "partner", "berg"];
    let is_cool = cool_keywords.iter().any(|keyword| screen_name.contains(keyword));
    // Prove we are cool 😎
    assert!(is_cool);

    // Bind to a new identity credential mapped directly from the unique x.com username
    let identity = Sha256::digest(&screen_name_bytes);

    // "Commit" the underlying value, making it public. Ensures that other parties can understand our return value, the account type. We emit this in an event field onchain.
    env::commit(&(identity.as_slice()));
}

// Get the value next to a JSON-like key/value
fn extract_value_from_key(bytes: &[u8], key: &str) -> Option<Vec<u8>> {
    let mut full_key = String::new();
    // Format the key according to the representation
    full_key.push('"');
    full_key.push_str(key);
    full_key.push_str("\":\"");
    let full_key = full_key.as_bytes();
    let key_len = full_key.len();

    // Step 1: Find the start index of the key
    let start = bytes.windows(key_len).position(|window| window == full_key)? + key_len;

    // Step 2: Find the end quote after the key
    let end = bytes[start..]
        .iter()
        .position(|&b| b == b'"')?
        + start;

    // Step 3: Return the slice containing the screen name bytes
    Some(bytes[start..end].to_vec())
}
