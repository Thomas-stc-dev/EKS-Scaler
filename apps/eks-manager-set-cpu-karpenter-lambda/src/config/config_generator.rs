use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct Cluster {
    #[serde(rename = "certificate-authority-data")]
    certificate_authority_data: String,
    server: String,
}

#[derive(Serialize)]
struct Context {
    cluster: String,
    user: String,
}

#[derive(Serialize)]
struct User {
    #[serde(rename = "client-certificate-data")]
    client_certificate_data: String,
    #[serde(rename = "client-key-data")]
    client_key_data: String,
}

#[derive(Serialize)]
struct KubeConfig {
    apiVersion: String,
    kind: String,
    clusters: Vec<NamedCluster>,
    contexts: Vec<NamedContext>,
    users: Vec<NamedUser>,
    #[serde(rename = "current-context")]
    current_context: String,
}

#[derive(Serialize)]
struct NamedCluster {
    name: String,
    cluster: Cluster,
}

#[derive(Serialize)]
struct NamedContext {
    name: String,
    context: Context,
}

#[derive(Serialize)]
struct NamedUser {
    name: String,
    user: User,
}

pub fn generate_kubeconfig(
    cluster_name: &str,
    server: &str,
    ca_data: &str,
    user_name: &str,
    client_cert_data: &str,
    client_key_data: &str,
    context_name: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let kubeconfig = KubeConfig {
        apiVersion: "v1".to_string(),
        kind: "Config".to_string(),
        clusters: vec![NamedCluster {
            name: cluster_name.to_string(),
            cluster: Cluster {
                certificate_authority_data: ca_data.to_string(),
                server: server.to_string(),
            },
        }],
        contexts: vec![NamedContext {
            name: context_name.to_string(),
            context: Context {
                cluster: cluster_name.to_string(),
                user: user_name.to_string(),
            },
        }],
        users: vec![NamedUser {
            name: user_name.to_string(),
            user: User {
                client_certificate_data: client_cert_data.to_string(),
                client_key_data: client_key_data.to_string(),
            },
        }],
        current_context: context_name.to_string(),
    };

    let yaml = serde_yaml::to_string(&kubeconfig)?;
    let mut file = File::create(output_path)?;
    file.write_all(yaml.as_bytes())?;

    Ok(())
}

pub fn config_generator() {
    let result = generate_kubeconfig(
        "my-cluster",
        "https://my-cluster.example.com",
        "base64-ca-data",
        "my-user",
        "base64-client-cert",
        "base64-client-key",
        "my-context",
        "kubeconfig.yaml",
    );

    match result {
        Ok(_) => println!("Kubeconfig generated successfully!"),
        Err(e) => eprintln!("Error generating kubeconfig: {}", e),
    }
}
