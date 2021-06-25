use async_graphql_reverse::*;
use clap::Clap;
use env_logger;

#[derive(Clap)]
#[clap(version = "0.1.5", author = "tacogips")]
struct Opts {
    #[clap(long, short)]
    input_schema: String,

    #[clap(long, short)]
    config: Option<String>,

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

    let config = match opts.config {
        Some(config_path) => match RendererConfig::load(&config_path) {
            Ok(config) => config,
            Err(e) => {
                println!("failed to load config toml file:{}", e);
                std::process::exit(1);
            }
        },
        None => RendererConfig::default(),
    };

    match parse_schema_file(&opts.input_schema) {
        Ok(structured_schema) => match output(&opts.output_dir, structured_schema, config) {
            Ok(()) => {
                println!("files outputed in {}", opts.output_dir);
            }

            Err(e) => {
                println!("{}", e);
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
