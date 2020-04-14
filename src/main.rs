use clap::Clap;
use meuse::token;
use reqwest;
use std::process;

#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K.")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// Manage Meuse tokens
    #[clap(name = "token")]
    Token(Token),
}

#[derive(Clap)]
struct Token {
    #[clap(subcommand)]
    subcmd: TokenSubCommand,
}

#[derive(Clap)]
enum TokenSubCommand {
    /// Create a new Token
    #[clap(name = "create")]
    TokenCreate(TokenCreate),
}

#[derive(Clap)]
struct TokenCreate {
    /// Print debug info
    #[clap(long = "name")]
    name: String,
    #[clap(long = "password", env = "MEUSE_PASSWORD")]
    password: String,
    #[clap(long = "user")]
    user: String,
    #[clap(long = "validity")]
    validity: i32,
}

fn main() {

    let opts: Opts = Opts::parse();

    let c = reqwest::blocking::Client::new();
    let client = meuse::client::Client::new("http", "localhost", 8855, c);

    match opts.subcmd {
        SubCommand::Token(sub_cmd) => {
            match sub_cmd.subcmd {
                TokenSubCommand::TokenCreate(token_create) => {
                    let token_req = token::CreateToken::new(
                        token_create.name,
                        token_create.password,
                        token_create.user,
                        token_create.validity);
                    let result = token::create_token(client, &token_req);
                    match result {
                        Ok(body) => println!("{}", body),
                        Err(e) => {
                            eprintln!("request failed: {}", e);
                            process::exit(1);
                        }
                    }
                }
            }
        }
    }
    process::exit(0);
}
