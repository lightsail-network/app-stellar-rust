//! Token contract call formatting utilities
//!
//! This module provides specialized formatting for known token contract calls,
//! displaying token symbols and properly formatted amounts with decimals.

extern crate alloc;

use crate::formatter::DataEntry;
use crate::parser::{InvokeContractArgs, ScAddress, ScVal, Uint256};
use crate::serialize::scval_to_key_string;
use crate::tokens::{format_token_amount, get_token_info, TokenInfo};
use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;

/// Attempts to format a contract call as a known token contract interaction
///
/// Returns Some(entries) if the contract is a known token and the function is recognized,
/// None otherwise to fall back to default formatting.
pub fn try_format_token_contract_call(args: &InvokeContractArgs) -> Option<Vec<DataEntry>> {
    // Extract contract address bytes
    let contract_bytes = match &args.contract_address {
        ScAddress::ScAddressTypeContract(contract_id) => {
            let Uint256(bytes) = contract_id;
            bytes
        }
        _ => return None,
    };

    // Check if this is a known token contract
    let token_info = get_token_info(contract_bytes)?;

    // Get function name
    let function_name = args.function_name.to_string();

    // Format based on token function type
    match function_name.as_str() {
        "transfer" => format_token_transfer(args, token_info),
        "approve" => format_token_approve(args, token_info),
        "transfer_from" => format_token_transfer_from(args, token_info),
        "mint" => format_token_mint(args, token_info),
        "burn" => format_token_burn(args, token_info),
        "clawback" => format_token_clawback(args, token_info),
        "burn_from" => format_token_burn_from(args, token_info),
        _ => None, // Unknown function, use default formatting
    }
}

/// Formats a token transfer call: transfer(from: Address, to: Address, amount: i128)
fn format_token_transfer(
    args: &InvokeContractArgs,
    token_info: &TokenInfo,
) -> Option<Vec<DataEntry>> {
    let args_slice = args.args.as_slice();
    if args_slice.len() != 3 {
        return None;
    }

    let mut entries = Vec::new();

    let from = extract_address_from_scval(&args_slice[0])?;
    let to = extract_address_from_scval(&args_slice[1])?;
    let amount = extract_i128_from_scval(&args_slice[2])?;

    let formatted_amount = format_token_amount(amount, token_info.decimals);
    entries.push(DataEntry::new(
        "Transfer",
        format!(
            "{} {}@{}",
            formatted_amount, token_info.symbol, args.contract_address
        ),
    ));
    entries.push(DataEntry::new("From", from));
    entries.push(DataEntry::new("To", to));

    Some(entries)
}

/// Formats a token approve call: approve(from: Address, spender: Address, amount: i128, expiration_ledger: u32)
fn format_token_approve(
    args: &InvokeContractArgs,
    token_info: &TokenInfo,
) -> Option<Vec<DataEntry>> {
    let args_slice = args.args.as_slice();
    if args_slice.len() != 4 {
        return None;
    }

    let mut entries = Vec::new();

    let from = extract_address_from_scval(&args_slice[0])?;
    let spender = extract_address_from_scval(&args_slice[1])?;
    let amount = extract_i128_from_scval(&args_slice[2])?;
    let exp_ledger = extract_u32_from_scval(&args_slice[3])?;

    let formatted_amount = format_token_amount(amount, token_info.decimals);
    entries.push(DataEntry::new(
        "Approve",
        format!(
            "{} {}@{}",
            formatted_amount, token_info.symbol, args.contract_address
        ),
    ));

    entries.push(DataEntry::new("From", from));
    entries.push(DataEntry::new("Spender", spender));
    entries.push(DataEntry::new("Exp Ledger", format!("{}", exp_ledger)));

    Some(entries)
}

/// Formats a token transfer_from call: transfer_from(spender: Address, from: Address, to: Address, amount: i128)
fn format_token_transfer_from(
    args: &InvokeContractArgs,
    token_info: &TokenInfo,
) -> Option<Vec<DataEntry>> {
    let args_slice = args.args.as_slice();
    if args_slice.len() != 4 {
        return None;
    }

    let mut entries = Vec::new();

    let spender = extract_address_from_scval(&args_slice[0])?;
    let from = extract_address_from_scval(&args_slice[1])?;
    let to = extract_address_from_scval(&args_slice[2])?;
    let amount = extract_i128_from_scval(&args_slice[3])?;

    let formatted_amount = format_token_amount(amount, token_info.decimals);
    entries.push(DataEntry::new(
        "Transfer",
        format!(
            "{} {}@{}",
            formatted_amount, token_info.symbol, args.contract_address
        ),
    ));
    entries.push(DataEntry::new("From", from));
    entries.push(DataEntry::new("To", to));
    entries.push(DataEntry::new("Spender", spender));

    Some(entries)
}

/// Formats a token mint call: mint(to: Address, amount: i128)
fn format_token_mint(args: &InvokeContractArgs, token_info: &TokenInfo) -> Option<Vec<DataEntry>> {
    let args_slice = args.args.as_slice();
    if args_slice.len() != 2 {
        return None;
    }

    let mut entries = Vec::new();

    let to = extract_address_from_scval(&args_slice[0])?;
    let amount = extract_i128_from_scval(&args_slice[1])?;

    let formatted_amount = format_token_amount(amount, token_info.decimals);
    entries.push(DataEntry::new(
        "Mint",
        format!(
            "{} {}@{}",
            formatted_amount, token_info.symbol, args.contract_address
        ),
    ));

    entries.push(DataEntry::new("To", to));

    Some(entries)
}

/// Formats a token burn call: burn(from: Address, amount: i128)
fn format_token_burn(args: &InvokeContractArgs, token_info: &TokenInfo) -> Option<Vec<DataEntry>> {
    let args_slice = args.args.as_slice();
    if args_slice.len() != 2 {
        return None;
    }

    let mut entries = Vec::new();

    let from = extract_address_from_scval(&args_slice[0])?;
    let amount = extract_i128_from_scval(&args_slice[1])?;

    let formatted_amount = format_token_amount(amount, token_info.decimals);
    entries.push(DataEntry::new(
        "Burn",
        format!(
            "{} {}@{}",
            formatted_amount, token_info.symbol, args.contract_address
        ),
    ));

    entries.push(DataEntry::new("From", from));

    Some(entries)
}

/// Formats a token burn_from call: burn_from(spender: Address, from: Address, amount: i128)
fn format_token_burn_from(
    args: &InvokeContractArgs,
    token_info: &TokenInfo,
) -> Option<Vec<DataEntry>> {
    let args_slice = args.args.as_slice();
    if args_slice.len() != 3 {
        return None;
    }

    let mut entries = Vec::new();

    let spender = extract_address_from_scval(&args_slice[0])?;
    let from = extract_address_from_scval(&args_slice[1])?;
    let amount = extract_i128_from_scval(&args_slice[2])?;

    let formatted_amount = format_token_amount(amount, token_info.decimals);
    entries.push(DataEntry::new(
        "Burn",
        format!(
            "{} {}@{}",
            formatted_amount, token_info.symbol, args.contract_address
        ),
    ));

    entries.push(DataEntry::new("From", from));
    entries.push(DataEntry::new("Spender", spender));

    Some(entries)
}

/// Formats a token clawback call: clawback(from: Address, amount: i128)
fn format_token_clawback(
    args: &InvokeContractArgs,
    token_info: &TokenInfo,
) -> Option<Vec<DataEntry>> {
    let args_slice = args.args.as_slice();
    if args_slice.len() != 2 {
        return None;
    }

    let mut entries = Vec::new();

    let from = extract_address_from_scval(&args_slice[0])?;
    let amount = extract_i128_from_scval(&args_slice[1])?;

    let formatted_amount = format_token_amount(amount, token_info.decimals);
    entries.push(DataEntry::new(
        "Clawback",
        format!(
            "{} {}@{}",
            formatted_amount, token_info.symbol, args.contract_address
        ),
    ));
    entries.push(DataEntry::new("From", from));

    Some(entries)
}

/// Attempts to extract an i128 value from an ScVal
///
/// Returns Some(i128) only if the ScVal is of I128 type, None otherwise.
/// This ensures strict type matching - if the expected type is i128 but
/// a different type (like u32, u64, etc.) is provided, we return None
/// to fall back to default formatting.
fn extract_i128_from_scval(scval: &ScVal) -> Option<i128> {
    match scval {
        ScVal::I128(parts) => {
            let hi = parts.hi as i128;
            let lo = parts.lo as i128;
            Some((hi << 64) | (lo & 0xFFFF_FFFF_FFFF_FFFF))
        }
        _ => None,
    }
}

/// Attempts to extract an Address from an ScVal
///
/// Returns Some(formatted_address) only if the ScVal is of Address type, None otherwise.
/// This ensures strict type matching - if the expected type is Address but
/// a different type is provided, we return None to fall back to default formatting.
fn extract_address_from_scval(scval: &ScVal) -> Option<alloc::string::String> {
    match scval {
        ScVal::Address(_) => Some(scval_to_key_string(scval)),
        _ => None,
    }
}

/// Attempts to extract a u32 value from an ScVal
///
/// Returns Some(u32) only if the ScVal is of U32 type, None otherwise.
/// This ensures strict type matching - if the expected type is u32 but
/// a different type is provided, we return None to fall back to default formatting.
fn extract_u32_from_scval(scval: &ScVal) -> Option<u32> {
    match scval {
        ScVal::U32(value) => Some(*value),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Int128Parts, PublicKey, ScAddress, Uint256};

    #[test]
    fn test_extract_i128_from_scval() {
        // Test I128 - should extract successfully
        let scval = ScVal::I128(Int128Parts { hi: 0, lo: 1000000 });
        assert_eq!(extract_i128_from_scval(&scval), Some(1000000));

        // Test U64 - should return None due to type mismatch
        let scval = ScVal::U64(1000000);
        assert_eq!(extract_i128_from_scval(&scval), None);

        // Test I64 - should return None due to type mismatch
        let scval = ScVal::I64(-1000000);
        assert_eq!(extract_i128_from_scval(&scval), None);

        // Test U32 - should return None due to type mismatch
        let scval = ScVal::U32(100);
        assert_eq!(extract_i128_from_scval(&scval), None);
    }

    #[test]
    fn test_extract_address_from_scval() {
        // Test Address - should extract successfully
        let bytes: [u8; 32] = [0u8; 32];
        let public_key = PublicKey::PublicKeyTypeEd25519(Uint256(&bytes));
        let address = ScAddress::ScAddressTypeAccount(public_key);
        let scval = ScVal::Address(address);
        assert!(extract_address_from_scval(&scval).is_some());

        // Test U32 - should return None due to type mismatch
        let scval = ScVal::U32(100);
        assert_eq!(extract_address_from_scval(&scval), None);

        // Test I128 - should return None due to type mismatch
        let scval = ScVal::I128(Int128Parts { hi: 0, lo: 1000000 });
        assert_eq!(extract_address_from_scval(&scval), None);
    }

    #[test]
    fn test_extract_u32_from_scval() {
        // Test U32 - should extract successfully
        let scval = ScVal::U32(12345);
        assert_eq!(extract_u32_from_scval(&scval), Some(12345));

        // Test I128 - should return None due to type mismatch
        let scval = ScVal::I128(Int128Parts { hi: 0, lo: 1000000 });
        assert_eq!(extract_u32_from_scval(&scval), None);

        // Test U64 - should return None due to type mismatch
        let scval = ScVal::U64(12345);
        assert_eq!(extract_u32_from_scval(&scval), None);

        // Test Address - should return None due to type mismatch
        let bytes: [u8; 32] = [0u8; 32];
        let public_key = PublicKey::PublicKeyTypeEd25519(Uint256(&bytes));
        let address = ScAddress::ScAddressTypeAccount(public_key);
        let scval = ScVal::Address(address);
        assert_eq!(extract_u32_from_scval(&scval), None);
    }
}
