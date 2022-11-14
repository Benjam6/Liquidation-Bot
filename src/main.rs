use std::fmt::Debug;
use std::panic;
use std::process;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::{Provider, Signer, SignerMiddleware, Wallet, Ws};
use ethers::providers::Http;
use eyre::Result;

mod account;
mod config;
mod exactly_events;
mod market;
mod protocol;

mod fixed_point_math;
mod liquidation;

mod generate_abi;

pub use account::*;
pub use exactly_events::*;
use log::debug;
use log::error;
use log::info;
pub use market::Market;
pub use protocol::Protocol;
pub use protocol::ProtocolError;
use sentry::integrations::log::SentryLogger;
use tokio::sync::Mutex;

use crate::config::Config;

type ExactlyProtocol = Protocol<Provider<Ws>, Provider<Http>, Wallet<SigningKey>>;

fn retries<T, U>(provider_result: Result<T, U>, retries: usize) -> T
where
    U: Debug,
{
    let mut counter = 0;
    loop {
        match provider_result {
            Ok(provider) => {
                break provider;
            }
            Err(ref e) => {
                if counter == retries {
                    panic!(
                        "Failed to connect to provider after {} retries\nerror:{:?}",
                        retries, e
                    );
                }
                counter += 1;
                thread::sleep(Duration::from_secs(1));
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}

async fn create_client(
    config: &Config,
) -> (
    Arc<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>,
    Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) {
    let provider_ws = retries(
        Provider::<Ws>::connect(config.rpc_provider.clone()).await,
        10,
    );
    let provider_https = retries(
        Provider::<Http>::try_from(config.rpc_provider_relayer.clone()),
        10,
    );
    let wallet = config.wallet.clone().with_chain_id(config.chain_id);
    (
        Arc::new(SignerMiddleware::new(provider_ws, wallet.clone())),
        Arc::new(SignerMiddleware::new(provider_https, wallet)),
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut log_builder = pretty_env_logger::formatted_builder();
    log_builder.parse_filters("warn,info,debug");
    let logger = SentryLogger::with_dest(log_builder.build());

    log::set_boxed_logger(Box::new(logger)).unwrap();
    if cfg!(debug_assertions) {
        log::set_max_level(log::LevelFilter::Debug);
    } else {
        log::set_max_level(log::LevelFilter::Info);
    }

    panic::set_hook(Box::new(|panic_info| {
        error!("panic: {:?}", panic_info);
        if let Some(client) = sentry::Hub::current().client() {
            client.close(Some(Duration::from_secs(2)));
        }
        process::abort();
    }));

    let config = Config::default();

    let _guard = config.sentry_dsn.clone().map(|sentry_dsn| {
        sentry::init((
            sentry_dsn,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                debug: true,
                attach_stacktrace: true,
                default_integrations: true,
                ..Default::default()
            },
        ))
    });

    dbg!(&config);

    let mut credit_service: Option<Arc<Mutex<ExactlyProtocol>>> = None;
    let mut update_client = false;
    let mut last_client = None;
    loop {
        if let Some((client, client_relayer)) = &last_client {
            if let Some(service) = &mut credit_service {
                if update_client {
                    info!("Updating client");
                    service
                        .lock()
                        .await
                        .update_client(Arc::clone(client), Arc::clone(client_relayer), &config)
                        .await;
                    update_client = false;
                }
            } else {
                info!("creating service");
                credit_service = Some(Arc::new(Mutex::new(
                    Protocol::new(Arc::clone(client), Arc::clone(client_relayer), &config).await?,
                )));
            }
            if let Some(service) = &credit_service {
                match Protocol::launch(Arc::clone(service)).await {
                    Ok(()) => {
                        break;
                    }
                    Err(e) => {
                        error!("credit service error: {:?}", e);
                        match e {
                            ProtocolError::SignerMiddlewareError(e) => {
                                debug!("client will be recreated");
                                error!("ProtocolError::SignerMiddlewareError: {:?}", e);
                                update_client = true;
                                last_client = None;
                            }
                            _ => {
                                println!("ERROR");
                                break;
                            }
                        }
                    }
                }
            }
        } else {
            last_client = Some(create_client(&config).await);
        }
    }
    Ok(())
}
