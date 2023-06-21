mod buy_source;
mod check_store;
mod error;
mod pb;
mod service;
mod static_map;
mod typedb;

use tonic_web::GrpcWebLayer;
use tonicserver;

const WEVE_ESI_NAMESPACE: &str = "WEVE_ESI";
const NAMESPACE: &str = "BUYBACK";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let firestoredb_fut = firestoredb::FirestoreDb::new(NAMESPACE);
    let weve_esi_addr = tonicserver::service_url(WEVE_ESI_NAMESPACE)?;
    let weve_esi_client_fut = pb::WeveEsiClient::connect(weve_esi_addr);

    let service_addr = tonicserver::service_address(NAMESPACE)?;
    let type_db = typedb::new_type_db(NAMESPACE)?;
    let price_model_store = buy_source::new_price_model_store();
    let market_name_store = buy_source::new_market_name_store();
    let token_store = buy_source::new_token_store();

    tonicserver::new_tonic_server(NAMESPACE)?
        .accept_http1(true)
        .layer(GrpcWebLayer::new())
        .add_service(pb::buyback_server::BuybackServer::new(service::Service {
            market_name_store,
            price_model_store,
            token_store,
            type_db,
            weve_esi_client: weve_esi_client_fut.await?,
            firestoredb: firestoredb_fut.await?,
        }))
        .serve(service_addr)
        .await?;

    Ok(())
}
