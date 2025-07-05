use risc0_zkvm::{guest::env, serde};

use primitives::AccountType;

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

    let json_bytes = extract_follower_num_object(&tlsn_representation_bytes).unwrap();
    env::log("Guest: Extracted json of tiktok account owner's followers");
    let json_str = core::str::from_utf8(json_bytes).unwrap();
    env::log("Guest: Interpreted json string");
    let wrapped = format!("{{{}}}", json_str);
    let parsed = json::parse(&wrapped).unwrap();
    let follower_val = parsed["follower_num"]["value"].as_u32().unwrap();
    env::log(
        &format!("Guest: got follower count {:?} from data", follower_val)
    );

    // Identify the account follower tier based on the number of followers
    let account_type = AccountType::from_follower_count(follower_val);

    // "Commit" the underlying value, making it public. Ensures that other parties can understand our return value, the account type. We emit this in an event field onchain.
    env::commit(&(account_type as u8));
}

// After taking a look at the api response JSON, we identified the unique field we want to return. That field is a nested field `follower_num`.
// We note that the same exists in the UTF-8 bytes of the response field of the session, which is added to the full bytes of the proof. 
// That means we can search for it from the top-level(for example code simplicity), though in your implementation you may wish to search only the received bytes field.
fn extract_follower_num_object(utf8_bytes: &[u8]) -> Option<&[u8]> {
    // Identify the field that we are interested in. `follower_num` is a json field object with a two keys underneath
    let desired_field = b"\"follower_num\"";

    let start_idx = utf8_bytes.windows(desired_field.len())
        .position(|window| window == desired_field)?;

    // Find the opening `{` after the key
    let mut brace_start = utf8_bytes[start_idx..].iter().position(|&b| b == b'{')?;
    brace_start += start_idx;

    // Now scan forward to find the matching `}`
    let mut depth = 0;
    let mut end_idx = brace_start;

    for (i, &b) in utf8_bytes[brace_start..].iter().enumerate() {
        match b {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    end_idx = brace_start + i + 1;
                    break;
                }
            }
            _ => {}
        }
    }

    Some(&utf8_bytes[start_idx..end_idx])
}
