use stellarlib::tokens::{format_token_amount, get_token_info, KNOWN_TOKENS};

#[test]
fn test_get_token_info_known_tokens() {
    // Test USDC lookup
    let usdc_address = [
        0xad, 0xef, 0xce, 0x59, 0xae, 0xe5, 0x29, 0x68, 0xf7, 0x60, 0x61, 0xd4, 0x94, 0xc2, 0x52,
        0x5b, 0x75, 0x65, 0x9f, 0xa4, 0x29, 0x6a, 0x65, 0xf4, 0x99, 0xef, 0x29, 0xe5, 0x64, 0x77,
        0xe4, 0x96,
    ];

    let token_info = get_token_info(&usdc_address);
    assert!(token_info.is_some());

    let token_info = token_info.unwrap();
    assert_eq!(token_info.symbol, "USDC");
    assert_eq!(token_info.decimals, 7);

    // Test BTC lookup
    let btc_address = [
        0x1d, 0xf1, 0x8d, 0x2d, 0x33, 0x1d, 0x88, 0x3e, 0x38, 0x1e, 0x3a, 0x9a, 0xe7, 0xb8, 0x22,
        0xb9, 0x48, 0x06, 0x4c, 0x32, 0xc0, 0x4b, 0x44, 0x54, 0x74, 0xe2, 0xc1, 0xc8, 0x2c, 0x35,
        0xa4, 0x00,
    ];

    let token_info = get_token_info(&btc_address);
    assert!(token_info.is_some());
    assert_eq!(token_info.unwrap().symbol, "BTC");
}

#[test]
fn test_get_token_info_unknown_token() {
    let unknown_address = [0u8; 32];
    assert!(get_token_info(&unknown_address).is_none());

    let random_address = [0xFF; 32];
    assert!(get_token_info(&random_address).is_none());
}

#[test]
fn test_format_token_amount_standard_decimals() {
    // Standard 7 decimals (most Stellar tokens)
    assert_eq!(format_token_amount(10_000_000, 7), "1");
    assert_eq!(format_token_amount(1_000_000, 7), "0.1");
    assert_eq!(format_token_amount(100_000, 7), "0.01");
    assert_eq!(format_token_amount(10_000, 7), "0.001");
    assert_eq!(format_token_amount(1_000, 7), "0.0001");
    assert_eq!(format_token_amount(100, 7), "0.00001");
    assert_eq!(format_token_amount(10, 7), "0.000001");
    assert_eq!(format_token_amount(1, 7), "0.0000001");
}

#[test]
fn test_format_token_amount_various_decimals() {
    // 0 decimals
    assert_eq!(format_token_amount(1000, 0), "1,000");

    // 2 decimals (like traditional currencies)
    assert_eq!(format_token_amount(100, 2), "1");
    assert_eq!(format_token_amount(1234, 2), "12.34");

    // 6 decimals
    assert_eq!(format_token_amount(1_000_000, 6), "1");
    assert_eq!(format_token_amount(123_456, 6), "0.123456");

    // 18 decimals (Ethereum-style)
    assert_eq!(format_token_amount(1_000_000_000_000_000_000, 18), "1");
}

#[test]
fn test_format_token_amount_large_numbers() {
    // Test with commas in large numbers
    assert_eq!(format_token_amount(10_000_000_000_000, 7), "1,000,000");
    assert_eq!(
        format_token_amount(123_456_789_012_345, 7),
        "12,345,678.9012345"
    );
}

#[test]
fn test_format_token_amount_negative() {
    assert_eq!(format_token_amount(-10_000_000, 7), "-1");
    assert_eq!(format_token_amount(-1_234_567, 7), "-0.1234567");
    assert_eq!(
        format_token_amount(-123_456_789_000_000, 7),
        "-12,345,678.9"
    );
}

#[test]
fn test_format_token_amount_edge_cases() {
    // Zero
    assert_eq!(format_token_amount(0, 7), "0");
    assert_eq!(format_token_amount(0, 0), "0");

    // Maximum precision
    assert_eq!(format_token_amount(12_345_678, 7), "1.2345678");

    // Very small amounts
    assert_eq!(format_token_amount(1, 7), "0.0000001");
    assert_eq!(format_token_amount(9, 7), "0.0000009");
}

#[test]
fn test_all_tokens_have_unique_addresses() {
    use std::collections::HashSet;
    let mut seen = HashSet::new();

    for token in KNOWN_TOKENS.iter() {
        assert!(
            seen.insert(token.contract_address),
            "Duplicate contract address found for token: {}",
            token.symbol
        );
    }

    // Verify we have the expected number of unique tokens
    assert_eq!(seen.len(), KNOWN_TOKENS.len());
}
