use async_graphql_reverse::*;
use clap::Clap;
use env_logger;
use log;

#[derive(Clap)]
#[clap(version = "0.0.1", author = "tacogips")]
struct Opts {
    #[clap(long, short)]
    input_schema: String,

    #[clap(long, short)]
    output_dir: String,
}

fn setup_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .target(env_logger::Target::Stdout)
        .init();
}

fn main() {
    setup_logger();
    let opts: Opts = Opts::parse();
    match parse_schema_file(&opts.input_schema) {
        Ok(structured_schema) => {
            let config = RendererConfig {
                custom_datasource_using: None,
            };
            match output(&opts.output_dir, structured_schema, config) {
                Ok(()) => {
                    log::info!("files outputed in {}", opts.output_dir);
                }

                Err(e) => {
                    log::error!("{}", e);
                }
            }
        }
        Err(e) => {
            log::error!("{}", e);
        }
    }
}
