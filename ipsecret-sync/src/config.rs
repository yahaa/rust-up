use k8s_openapi::api::core::v1::Secret;
use kube::{Api, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    server: String,
    username: String,
    password: String,
    // if namespaces.len()==0 || namespaces.contains("*")
    // then this config effect all namespace, else this config
    // only effect on the specify namespaces.
    namespaces: Vec<String>,
}

#[derive(Debug)]
pub enum Error {
    ApiErr { source: kube::Error },
    NotFoundErr { msg: String },
}

pub struct ConfigFactory {
    client: Client,
    namespace: String,
    secret: String,
}

impl ConfigFactory {
    pub fn new(client: Client, secret: String, namespace: String) -> Self {
        ConfigFactory {
            client,
            namespace,
            secret,
        }
    }

    pub async fn read_config(&self) -> Result<Config, Error> {
        let secret_api: Api<Secret> = Api::namespaced(self.client.clone(), &self.namespace);

        let secret: Secret = secret_api
            .get(&self.secret)
            .await
            .expect("get secret error");

        if let Some(data) = secret.data {
            let byte_str = data.get("registry_secrets").unwrap();

            let config: Config = serde_yaml::from_slice(&byte_str.0).expect("msg");
            return Ok(config);
        }

        Err(Error::NotFoundErr {
            msg: "not found".to_string(),
        })
    }
}
