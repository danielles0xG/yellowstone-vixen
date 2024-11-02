/*

The vixen consumes event streams from the Solana rpc nodes,
decode/parses them, then filters whats needed and
encodeds it into protobuffers for fast streaming.
Any clients (js apps) can subscribe to the vixen server and get the data
in json format or raw protobuffers data


Vixen: crate runtime is the yellowstone-vixen aka the "vixen server"
Parser: A module responsible for transforming raw Solana data into a program-specific format.
Handler: A module that processes the parsed data, performing tasks such as logging, storing in a database, or triggering other actions.
HandlerManager: Manages multiple handlers for different types of data (e.g., accounts, transactions).
Configuration: A TOML file specifying the settings and parameters for Vixen. src/crate/vixen/Vixen.toml

ray amm: 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
ray cpmm : CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C
meteora DLMM: LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo
meteora pools program: Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB
orca whirlpool : whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc
fluxbeam program: FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X

*/

use clap::Parser as _;
use spl_pod::solana_program::program_error::ProgramError;
use std::borrow::Cow;
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use yellowstone_vixen::{self as vixen, Handler, HandlerResult, Pipeline};
use yellowstone_vixen_core::{
    AccountUpdate, ParseResult, Parser, Prefilter, ProgramParser, Pubkey, TransactionUpdate,
};
use yellowstone_vixen_parser::{
    token_extension_program::{
        account_parser::TokenExtensionProgramAccParser, account_parser::TokenExtensionState,
        ix_parser::TokenExtensionProgramIxParser,
    },
    token_program::{account_parser::TokenProgramAccParser, ix_parser::TokenProgramIxParser},
};

/**
 * Parser: Defines the parsing logic for the specific program.
 * The prefilter method sets up filters for the accounts owned by the target program,
 * which are used to build the underlying Dragon's Mouth subscription.
 * The parse method contains the logic to transform raw account data into the desired structure.
 */
pub struct CustomParser;

const RAYDIAMOND_PROGRAM_ID: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

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
            .account_owners([RAYDIAMOND_PROGRAM_ID]) // Replace with the actual program ID
            .build()
            .unwrap()
    }
}

/**
 * Custom HAndler
 * Handler: A module that processes the parsed data, performing tasks such as logging,
 * storing in a database, or triggering other actions.
 *
 *  Defines how the parsed data should be handled.
 *  This could involve logging the data, storing it in a database,
 *  or triggering other actions
 */

pub struct CustomHandler;

impl<H: std::fmt::Debug + Sync> Handler<H> for CustomHandler {
    async fn handle(&self, value: &H) -> HandlerResult<()> {
        // Implement handling logic here
        // Example: tracing::info!(?value);
        unimplemented!()
    }
}

/**
 * Main: Sets up the tracing subscriber, reads the configuration file,
 * and runs the Vixen framework with the specified handlers, managers and metrics.
 */

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");

    vixen::Runtime::builder()
        .account(Pipeline::new(TokenExtensionProgramAccParser, [CustomHandler]))
        .account(Pipeline::new(TokenProgramAccParser, [CustomHandler]))
        .instruction(Pipeline::new(TokenExtensionProgramIxParser, [CustomHandler]))
        .instruction(Pipeline::new(TokenProgramIxParser, [CustomHandler]))
        .build(config)
        .run();
}


/*
Run exmaples stream-parser

cd exampl
RUST_LOG=info cargo run -- --config "$(pwd)/../../Vixen.toml"

cd crates/proto/proto
grpcurl -plaintext -import-path ./ -proto stream.proto -proto parser.proto -proto solana-token/accounts.proto -d '{"program": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"}' 127.0.0.1:3030 vixen.stream.ProgramStreams/Subscribe
*/