use anyhow::{anyhow, Result};
use ipsecret_sync::config::{Config, RegistryAuth};
use k8s_openapi::api::core::v1::{LocalObjectReference, Namespace, Secret, ServiceAccount};
use kube::{
    api::{ListParams, Meta, PatchParams, PostParams},
    Api, Client,
};
use serde_json::json;

pub struct SyncWorker<'a> {
    cfg_ns: &'a str,
    cfg_name: &'a str,
    cfg_data_key: &'a str,
    sa_name: &'a str,
    client: Client,
}

impl<'a> SyncWorker<'a> {
    pub fn new(client: Client, cfg_ns: &'a str, cfg_name: &'a str) -> Self {
        SyncWorker {
            client,
            cfg_ns,
            cfg_name,
            cfg_data_key: "registry_secrets", // todo
            sa_name: "default",               // todo
        }
    }

    async fn ensure(&self, all_ns: Vec<String>, configs: Vec<Config>) -> Result<()> {
        unimplemented!()
    }

    pub async fn watch_ns(&self) -> Result<()> {
        unimplemented!()
    }

    pub async fn watch_cfg_secret(&self) -> Result<()> {
        unimplemented!()
    }

    async fn get_all_ns(&self) -> Result<Vec<String>> {
        let ns_api = Api::<Namespace>::all(self.client.clone());
        let lp = ListParams::default().fields("status.phase=Active");

        let all_ns = ns_api.list(&lp).await?;

        Ok(all_ns.items.iter().map(|item| item.name()).collect())
    }

    async fn ensure_registry_secret(&self, ns: &str, cfg: Config) -> Result<()> {
        if !cfg.namespaces.contains(&format!("*")) && !cfg.namespaces.contains(&format!("{}", ns)) {
            return Err(anyhow!("{} don't need sync to {}", cfg.server, ns));
        }

        let auth = RegistryAuth::new(
            cfg.username.clone(),
            cfg.password.clone(),
            cfg.server.clone(),
        );

        let key = ".dockerconfigjson";

        let secret_api = Api::<Secret>::namespaced(self.client.clone(), self.cfg_ns);
        match secret_api.get(&cfg.server).await {
            Ok(s) => {
                let p = serde_json::to_vec(&json!({ "data": {key: auth.base64_encode() } }))?;
                let pp = PatchParams::default();
                secret_api.patch(&cfg.server, &pp, p).await?;
            }
            Err(kube::Error::Api(e)) => {
                if e.code == 404 {
                    let s: Secret = serde_json::from_value(json!({
                            "apiVersion": "v1",
                            "data": {
                                ".dockerconfigjson": auth.base64_encode(),
                            },
                            "kind": "Secret",
                            "metadata": {
                                "name": cfg.server,
                                "namespace": ns,
                            },
                            "type": "kubernetes.io/dockerconfigjson"
                        }
                    ))?;

                    let pp = PostParams::default();

                    secret_api.create(&pp, &s).await?;
                }
            }
            Err(e) => return Err(anyhow!("query {} err: {}", cfg.server, e)),
        }

        Ok(())
    }

    async fn ensure_patch_sa(&self, ns: &str, secret_name: &str) -> Result<()> {
        let sa_api = Api::<ServiceAccount>::namespaced(self.client.clone(), ns);

        let mut found = false;
        let mut new_secrets: Vec<LocalObjectReference> = Vec::new();

        match sa_api.get(self.sa_name).await {
            Ok(sa) => {
                if let Some(ipss) = sa.image_pull_secrets {
                    for item in ipss {
                        if item.name == Some(String::from(secret_name)) {
                            found = true
                        }
                        new_secrets.push(item);
                    }
                }
            }
            Err(e) => return Err(anyhow!("get {}/default sa err: {}", ns, e)),
        }

        if !found {
            let p = serde_json::to_vec(&json!({ "imagePullSecrets": new_secrets }))?;
            let pp = PatchParams::default();
            sa_api.patch(self.sa_name, &pp, p).await?;
        }

        Ok(())
    }

    async fn read_config(&self) -> Result<Vec<Config>> {
        let secret_api = Api::<Secret>::namespaced(self.client.clone(), self.cfg_ns);
        let secret = secret_api.get(self.cfg_name).await?;

        self.read_data(secret).await
    }

    async fn read_data(&self, secret: Secret) -> Result<Vec<Config>> {
        match secret.data {
            Some(map) => match map.get(self.cfg_data_key) {
                Some(byte_str) => {
                    let config: Vec<Config> = serde_yaml::from_slice(&byte_str.0)?;
                    return Ok(config);
                }
                None => Err(anyhow!("read secret data field {} err", self.cfg_data_key)),
            },
            None => Err(anyhow!("read secret data err")),
        }
    }
}
