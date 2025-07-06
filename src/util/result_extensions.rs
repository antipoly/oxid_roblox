use async_trait::async_trait;

#[async_trait]
pub(crate) trait ResultExtensions<T, E> {
    async fn map_async<U, Future, Mapper>(self, mapper: Mapper) -> Result<U, E>
    where
        Future: std::future::Future<Output = U> + Send + 'async_trait,
        Mapper: FnOnce(T) -> Future + Send;
}

#[async_trait]
impl<T: Send, E: Send> ResultExtensions<T, E> for Result<T, E> {
    async fn map_async<U, Future, Mapper>(self, mapper: Mapper) -> Result<U, E>
    where
        Future: std::future::Future<Output = U> + Send + 'async_trait,
        Mapper: FnOnce(T) -> Future + Send,
    {
        match self {
            Ok(value) => Ok(mapper(value).await),
            Err(value) => Err(value),
        }
    }
}
