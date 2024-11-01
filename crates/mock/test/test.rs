



#[cfg(test)]
mod tests {
    use yellowstone_vixen_mock::{account_fixture, run_account_parse, tx_fixture, run_ix_parse, FixtureData};

    // using token program and token extension program parsers
    use yellowstone_vixen_parser::{
        token_extension_program::{
            AccountParser as TokenExtensionProgramAccParser,
            InstructionParser as TokenExtensionProgramIxParser,
            TokenExtensionProgramIx

        },
        token_program::{
            AccountParser as TokenProgramAccParser, InstructionParser as TokenProgramIxParser,
            TokenProgramState
        };
    };

    // test account parsing
    #[tokio::test]
    async fn test_mint_account_parsing() {
        let parser = TokenProgramAccParser;

        let account = account_fixture!("3SmPYPvZfEmroktLiJsgaNENuPEud3Z52zSfLQ1zJdkK", &parser);

        let TokenProgramState::Mint(mint) = account else {
            panic!("Invalid Account");
        };

        assert_eq!(mint.decimals, 10);
    }

    // test instruction parsing
    #[tokio::test]
    async fn test_mint_to_checked_ix_parsing() {
        let parser = InstructionParser;

        let ixs = tx_fixture!("44gWEyKUkeUabtJr4eT3CQEkFGrD4jMdwUV6Ew5MR5K3RGizs9iwbkb5Q4T3gnAaSgHxn3ERQ8g5YTXuLP1FrWnt",&parser);

        let TokenExtensionProgramIx::TokenProgramIx(TokenProgramIx::MintToChecked(_accts, data)) =
            &ixs[0]
        else {
            panic!("Invalid Instruction");
        };

        assert_eq!(data.decimals, 9);
        assert_eq!(data.amount, 100.mul(10u64.pow(data.decimals.into())));
    }

}