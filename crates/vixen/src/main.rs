

// parsing pipeline:
use yellowstone_vixen_core::{AccountUpdate, ParseResult, Parser, Prefilter, ProgramParser};
use std::borrow::Cow;

/**
 * Parser: Defines the parsing logic for the specific program.
 * The prefilter method sets up filters for the accounts owned by the target program,
 * which are used to build the underlying Dragon's Mouth subscription.
 * The parse method contains the logic to transform raw account data into the desired structure.
 */
pub struct CustomParser;

impl yellowstone_vixen_core::Parser for CustomParser {
    type Input = AccountUpdate;
    type Output = CustomParsedData; // Replace with the actual data type

    fn id(&self) -> Cow<str> {
        "yellowstone_vixen_parser::token_extensions::TokenExtensionProgramAccParser".into()
    }

    async fn parse(&self, acct: &AccountUpdate) -> ParseResult<Self::Output> {
        let inner = acct.account.as_ref().ok_or(ProgramError::InvalidArgument)?;
        TokenExtensionState::try_unpack(&inner.data)
    }

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([CUSTOM_PROGRAM_ID]) // Replace with the actual program ID
            .build()
            .unwrap()
    }

    async fn parse(&self, acct: &AccountUpdate) -> ParseResult<Self::Output> {
        // Implement parsing logic here
        // Example: Ok(CustomParsedData::from(acct))
        unimplemented!()
    }
}
fn main() {
    println!("Hello, world!");
}
