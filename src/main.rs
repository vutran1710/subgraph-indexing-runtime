mod asc;
mod bignumber;
mod chain;
mod common;
mod config;
mod database;
mod errors;
mod from_to;
mod manifest_loader;
mod messages;
mod source_serializer;
mod subgraph;
mod wasm_host;

use config::Config;
use database::Database;
use errors::SwrError;
use manifest_loader::LoaderTrait;
use manifest_loader::ManifestLoader;
use source_serializer::SourceSerializer;
use subgraph::DatasourceWasmInstance;
use subgraph::Subgraph;
use wasm_host::create_wasm_host;

#[tokio::main]
async fn main() -> Result<(), SwrError> {
    // TODO: impl CLI
    let config = Config::load()?;

    // TODO: impl Source Consumer

    // TODO: impl IPFS Loader
    let manifest = ManifestLoader::new(&config.manifest).await?;

    // TODO: impl raw-data serializer
    let serializer = SourceSerializer::new(config.clone())?;

    // TODO: impl Actual DB Connection
    let database = Database::new(&config).await?;

    let subgraph_id = config
        .subgraph_id
        .clone()
        .unwrap_or(config.subgraph_name.clone());

    let mut subgraph = Subgraph::new_empty(&config.subgraph_name, &subgraph_id);

    for datasource in manifest.datasources() {
        let api_version = datasource.mapping.apiVersion.to_owned();
        let wasm_bytes = manifest.load_wasm(&datasource.name).await?;
        let dbstore_agent = database.agent();
        let wasm_host = create_wasm_host(api_version, wasm_bytes, dbstore_agent)?;
        let subgraph_source = DatasourceWasmInstance::try_from((wasm_host, datasource))?;
        subgraph.add_source(subgraph_source);
    }

    let (_subgraph_msg_sender, subgraph_receiver) = kanal::bounded_async(1);

    let subscriber_run = async move { Ok::<(), SwrError>(()) };
    let swr_run = subgraph.run_async(subgraph_receiver);

    ::tokio::select! {
        result = subscriber_run => result,
        result = swr_run => result.map_err(SwrError::from),
        // TODO: impl prometheus
    }
}
