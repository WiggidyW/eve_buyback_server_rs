// @generated
/// Generated client implementations.
pub mod weve_esi_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct WeveEsiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WeveEsiClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> WeveEsiClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> WeveEsiClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            WeveEsiClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        ///
        pub async fn active_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::ActiveOrdersReq>,
        ) -> Result<tonic::Response<super::ActiveOrdersRep>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/weve_esi_proto.WeveEsi/ActiveOrders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn adjusted_price(
            &mut self,
            request: impl tonic::IntoRequest<super::AdjustedPriceReq>,
        ) -> Result<tonic::Response<super::AdjustedPriceRep>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/weve_esi_proto.WeveEsi/AdjustedPrice",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn assets(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetsReq>,
        ) -> Result<tonic::Response<super::AssetsRep>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/weve_esi_proto.WeveEsi/Assets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn industry_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::IndustryJobsReq>,
        ) -> Result<tonic::Response<super::IndustryJobsRep>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/weve_esi_proto.WeveEsi/IndustryJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn market_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketOrdersReq>,
        ) -> Result<tonic::Response<super::MarketOrdersRep>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/weve_esi_proto.WeveEsi/MarketOrders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn skills(
            &mut self,
            request: impl tonic::IntoRequest<super::SkillsReq>,
        ) -> Result<tonic::Response<super::SkillsRep>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/weve_esi_proto.WeveEsi/Skills",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn system_index(
            &mut self,
            request: impl tonic::IntoRequest<super::SystemIndexReq>,
        ) -> Result<tonic::Response<super::SystemIndexRep>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/weve_esi_proto.WeveEsi/SystemIndex",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::TransactionsReq>,
        ) -> Result<tonic::Response<super::TransactionsRep>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/weve_esi_proto.WeveEsi/Transactions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
