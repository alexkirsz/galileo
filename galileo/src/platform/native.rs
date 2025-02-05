use crate::error::GalileoError;
use crate::platform::PlatformService;
use crate::primitives::DecodedImage;
use async_trait::async_trait;
use bytes::Bytes;
use log::info;

#[derive(Debug, Clone)]
pub struct NativePlatformService {
    http_client: reqwest::Client,
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl PlatformService for NativePlatformService {
    fn new() -> Self {
        let http_client = reqwest::Client::builder()
            .user_agent("galileo/0.1")
            .build()
            .unwrap();

        Self { http_client }
    }

    async fn load_image_url(&self, url: &str) -> Result<DecodedImage, GalileoError> {
        let image_source = self.load_from_web(url).await?;
        DecodedImage::new(&image_source)
    }

    async fn load_bytes_from_url(&self, url: &str) -> Result<Bytes, GalileoError> {
        self.load_from_web(url).await
    }
}

impl NativePlatformService {
    async fn load_from_web(&self, url: &str) -> Result<Bytes, GalileoError> {
        let response = self.http_client.get(url).send().await?;
        if !response.status().is_success() {
            info!(
                "Failed to load {url}: {}, {:?}",
                response.status(),
                response.text().await
            );
            return Err(GalileoError::IO);
        }

        Ok(response.bytes().await?)
    }
}
