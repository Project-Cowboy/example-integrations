use risc0_zkvm::{guest::env, serde};

// According to chatgpt, tiktok influencers are split into tiers. We'll define each one
#[derive(Debug)]
#[repr(u8)]
enum InfluencerType {
    /// Less than 1,000 followers.
    None,
    /// 1,000–10,000 followers.
    Nano,
    /// 10,001–50,000 followers.
    Micro,
    /// 50,001–500,000 followers.
    MidTier,
    /// 500,001–1,000,000 followers.
    Macro,
    /// 1,000,001+ followers.
    Mega,
}

// The following code's execution trace will be proven with a zk stark
fn main() {
    // Receive journal, for verifying the core proof of the tls notary signatures
    let journal: [u8; 32] = env::read();
    // Receive the entire TLS Notary representation
    // TODO: Hash this with accelerated sha2 and compare to hash above(journal value)
    let tlsn_representation_bytes: Vec<u8> = env::read();
    // Program id for core tlsn verifier 
    let tlsn_prover_id = [1103052681, 2953117475, 3732232465, 3183722244, 307322261, 1784411821, 2489196935, 3129733399];
    // Verify previous proof of our tls notary proof validity
    env::verify(tlsn_prover_id, &serde::to_vec(&journal).unwrap()).unwrap();

    let json_bytes = extract_follower_num_object(&tlsn_representation_bytes).unwrap();
    let json_str = core::str::from_utf8(json_bytes).unwrap();
    let wrapped = format!("{{{}}}", json_str);
    let parsed = json::parse(&wrapped).unwrap();
    let follower_val = parsed["follower_num"]["value"].as_u32().unwrap();


    // Identify the type of account based on amount of followers
    let influencer_type = match follower_val {
        0..=999 => InfluencerType::None,
        1000..=10_000 => InfluencerType::Nano,
        10_001..=50_000 => InfluencerType::Micro,
        50_001..=500_000 => InfluencerType::MidTier,
        500_001..=1_000_000 => InfluencerType::Macro,
        _ => InfluencerType::Mega,
    };

    // "Commit" the underlying value, making it public. Ensures that other parties can understand our return value, the influencer type
    env::commit(&(influencer_type as u8));
}

// After taking a look at the api response JSON, we identified the unique field we want to return. That field is a nested field `follower_num`.
// We note that the same exists in the UTF-8 bytes of the response field of the session, which is added to the full bytes of the proof. 
// That means we can search for it from the top-level(for example code simplicity).
fn extract_follower_num_object(utf8_bytes: &[u8]) -> Option<&[u8]> {
    // Identify the field that we are interested in. `follower_num` is a json field object with a two keys underneath
    let needle = b"\"follower_num\"";
    let start_idx = utf8_bytes.windows(needle.len())
        .position(|window| window == needle)?;

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
