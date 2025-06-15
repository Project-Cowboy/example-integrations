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

// https://www.tiktok.com/aweme/v2/data/insight/?locale=en&aid=1988&priority_region=US&region=US&tz_name=America%2FNew_York&app_name=tiktok_creator_center&app_language=en&device_platform=web_pc&channel=tiktok_web&device_id=7397803382726641198&os=mac&screen_width=1440&screen_height=900&browser_language=en-US&browser_platform=MacIntel&browser_name=Mozilla&browser_version=5.0+(Macintosh%3B+Intel+Mac+OS+X+10_15_7)+AppleWebKit%2F537.36+(KHTML,+like+Gecko)+Chrome%2F137.0.0.0+Safari%2F537.36&tz_offset=-14400&type_requests=[%7B%22insigh_type%22:%22follower_num_history%22,%22days%22:16,%22end_days%22:1%7D,%7B%22insigh_type%22:%22follower_num%22,%22days%22:16,%22end_days%22:1%7D,%7B%22insigh_type%22:%22net_follower_history%22,%22days%22:16,%22end_days%22:1%7D]

// The following code's execution trace will be proven with a zk stark
fn main() {
    // Receive journal, for verifying the core proof of the tls notary signatures
    let journal: [u8; 32] = env::read();
    // Receive the entire TLS Notary representation
    // TODO: Hash this with accelerated sha2 and compare to hash above(journal value)
    let tlsn_representation_bytes: Vec<u8> = env::read();
    // Program id for core tlsn verifier 
    let tlsn_prover_id = [1363182423, 1400773265, 918797330, 1419448088, 1135916865, 604038609, 1578339176, 834135541] ;
    
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
