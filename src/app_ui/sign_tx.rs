/*****************************************************************************
 *   Ledger App Stellar Rust.
 *   (c) 2025 overcat
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *****************************************************************************/

use crate::settings::Settings;
use crate::{icons, sw::AppSW};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use ledger_device_sdk::nbgl::{Field, NbglReview};
use stellarlib::{
    format_operation, format_transaction_signature_payload, get_operation_intent, DataEntry,
    Operation, Parser, TransactionSignaturePayload, XdrParse,
};

pub fn ui_sign_tx(raw_data: &[u8], signer: &[u8]) -> Result<bool, AppSW> {
    let config = Settings.to_format_config();
    let signer = stellar_strkey::ed25519::PublicKey::from_payload(&signer)
        .expect("caller guarantees 32-byte Stellar public key")
        .to_string();
    let mut parser = Parser::new(raw_data);
    let tx_signature_payload =
        TransactionSignaturePayload::parse(&mut parser).map_err(|_| AppSW::DataParsingFail)?;

    let mut data_entries =
        format_transaction_signature_payload(&tx_signature_payload, &config, &signer)
            .map_err(|_| AppSW::DataFormattingFail)?;

    let op_count = match &tx_signature_payload.tagged_transaction {
        stellarlib::TaggedTransaction::EnvelopeTypeTx(tx) => tx.op_count,
        stellarlib::TaggedTransaction::EnvelopeTypeTxFeeBump(fee_bump_tx) => {
            match &fee_bump_tx.inner_tx {
                stellarlib::InnerTransaction::EnvelopeTypeTx(tx) => tx.op_count,
            }
        }
    };

    let mut intent: Option<String> = None;

    for i in 0..op_count {
        if op_count > 1 {
            data_entries.push(DataEntry::new(
                "Operation",
                format!("{} of {}", i + 1, op_count),
            ));
        }
        let op = Operation::parse(&mut parser).map_err(|_| AppSW::DataParsingFail)?;
        let mut op_entries =
            format_operation(&op, &config).map_err(|_| AppSW::DataFormattingFail)?;
        data_entries.append(&mut op_entries);

        if op_count == 1 {
            intent = get_operation_intent(&op.body);
        }
    }

    let (title, finish_title) = match &tx_signature_payload.tagged_transaction {
        stellarlib::TaggedTransaction::EnvelopeTypeTx(_) => match &intent {
            Some(intent) => (
                format!("Review transaction to {}", intent),
                format!("Sign transaction to {}?", intent),
            ),
            None => (
                "Review transaction".to_string(),
                "Sign transaction?".to_string(),
            ),
        },
        stellarlib::TaggedTransaction::EnvelopeTypeTxFeeBump(_) => (
            "Review fee bump transaction".to_string(),
            "Sign fee bump transaction?".to_string(),
        ),
    };

    let review = NbglReview::new()
        .titles(&title, "", &finish_title)
        .glyph(&icons::STELLAR);

    let fields: Vec<Field> = data_entries
        .iter()
        .map(|entry| Field {
            name: entry.title.as_str(),
            value: entry.content.as_str(),
        })
        .collect();

    let review_result = review.show(&fields);
    Ok(review_result)
}
