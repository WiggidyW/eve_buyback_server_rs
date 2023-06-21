use crate::{
    buy_source::{self, MarketNameStore, PriceModelStore, TokenStore},
    check_store, pb,
    typedb::TypeDb,
};
use firestoredb::FirestoreDb;
use tonic::{self, async_trait};

pub struct Service<MNS, PMS, TS, TDB> {
    pub market_name_store: MNS,
    pub price_model_store: PMS,
    pub token_store: TS,
    pub type_db: TDB,
    pub weve_esi_client: pb::WeveEsiClient,
    pub firestoredb: FirestoreDb,
}

#[async_trait]
impl<MNS, PMS, TS, TDB> pb::buyback_server::Buyback for Service<MNS, PMS, TS, TDB>
where
    MNS: MarketNameStore,
    PMS: PriceModelStore,
    TS: TokenStore,
    TDB: TypeDb,
{
    async fn check(
        &self,
        request: tonic::Request<pb::buyback::CheckReq>,
    ) -> Result<tonic::Response<pb::buyback::Rep>, tonic::Status> {
        let req = request.into_inner();
        Ok(tonic::Response::new(
            check_store::read(&self.firestoredb, &req.hash, &req.language, &self.type_db)
                .await?
                .unwrap_or(pb::buyback::Rep {
                    items: Vec::new(),
                    hash: "".to_string(),
                    sum: 0.0,
                    timestamp: 0,
                    version: "".to_string(),
                    location: "".to_string(),
                }),
        ))
    }

    async fn buy(
        &self,
        request: tonic::Request<pb::buyback::BuyReq>,
    ) -> Result<tonic::Response<pb::buyback::Rep>, tonic::Status> {
        let req = request.into_inner();
        let rep = buy_source::buy(
            req,
            &self.type_db,
            &self.market_name_store,
            &self.token_store,
            &self.price_model_store,
            &self.weve_esi_client,
        )
        .await?;
        if rep.sum > 0.0 {
            check_store::write(&self.firestoredb, &rep.hash, &rep).await?;
        }
        Ok(tonic::Response::new(rep))
    }
}
