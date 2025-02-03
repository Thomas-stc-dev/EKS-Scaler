use lambda_http::{Body, Error, Request, RequestExt, Response};
use aws_sdk_eks::config::Region;
use aws_sdk_eks as eks;

/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
  

    // get region from event and set it to the config
    let region = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("region"))
        .unwrap_or("ap-northeast-1")
        .to_string();
    let region_provider = Region::new(region);

    let config = aws_config::from_env().region(region_provider).load().await;
    


    let client = eks::Client::new(&config);

    // List all clusters
    let clusters = client
    .list_clusters().send().await?;
    let cluster_names = clusters.clusters();
    // convert clusternames to a vector of strings
    let cluster_names = cluster_names.iter().map(|c| c.clone()).collect::<Vec<String>>();

    println!("Clusters: {:?}", cluster_names);

    // return the cluster names as a response
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&cluster_names).unwrap().into())
        .map_err(Box::new)?;

    Ok(resp)
}
