use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use tokio::fs;

use crate::custom_servers::models::CustomServer;
use crate::utils::download_file;
use crate::{HTTP_CLIENT, LAUNCHER_DIRECTORY};

/// Placeholder struct for API endpoints implementation
pub struct ForgeProvider;

static FORGE_MODRINTH_API_BASE: &str = "https://meta.modrinth.com/forge";
static FORGE_MAVEN_REPO_BASE: &str = "https://files.minecraftforge.net/net/minecraftforge/forge";

impl ForgeProvider {
    /// Request all available minecraft versions
    pub async fn get_manifest() -> Result<ForgeManifest> {
        Self::request_from_endpoint(FORGE_MODRINTH_API_BASE, "v0/manifest.json").await
    }

    pub async fn download_installer_jar<F>(custom_server: &CustomServer, on_progress: F) -> Result<()> where F : Fn(u64, u64) {
        let path = LAUNCHER_DIRECTORY.data_dir().join("custom_servers").join("installers").join(format!("forge-{}-{}.jar", custom_server.mc_version, custom_server.loader_version));
        let url = format!("{}/{mc}-{loader}/forge-{mc}-{loader}-installer.jar", FORGE_MAVEN_REPO_BASE, mc = custom_server.mc_version, loader = custom_server.loader_version);
        let content = download_file(&url, on_progress).await?;
        let _ = fs::write(path, content).await.map_err(|e| e);
        Ok(())
    }

    pub async fn create_eula_file(custom_server: &CustomServer) -> Result<()> {
        let path = LAUNCHER_DIRECTORY.data_dir().join("custom_servers").join(&custom_server.id).join("eula.txt");
        let content = "# USER HAS AGREED TO THIS THROUGH THE GUI OF THE NRC LAUNCHER!\neula=true";
        let _ = fs::write(path, Vec::from(content)).await.map_err(|e| e);
        Ok(())
    }

    /// Request JSON formatted data from launcher API
    pub async fn request_from_endpoint<T: DeserializeOwned>(base: &str, endpoint: &str) -> Result<T> {
        let url = format!("{}/{}", base, endpoint);
        println!("URL: {}", url); // Den formatierten String ausgeben
        Ok(HTTP_CLIENT.get(url)
            .send().await?
            .error_for_status()?
            .json::<T>()
            .await?
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForgeManifest {
    #[serde(rename = "gameVersions")]
    pub game_versions: Vec<ForgeGameVersion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForgeGameVersion {
    pub id: String,
    pub stable: bool,
    pub loaders: Vec<ForgeLoaderVersion>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForgeLoaderVersion {
    pub id: String,
}