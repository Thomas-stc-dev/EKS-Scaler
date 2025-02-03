use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::Client as Ec2Client;
use lambda_http::{Body, Error, Request, RequestExt, Response};


pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let cluster_name = event
    .query_string_parameters_ref()
    .and_then(|params| params.first("cluster-name"))
    .unwrap_or("");

    if cluster_name.is_empty() {
        return failure_response("cluster-name param is required", 400);
    }

    let instances = terminate_running_instance_ids_from_cluster(cluster_name).await;
    
    // respond with the instance IDs as a JSON array
    let resp = match instances {
        Ok(ids) => Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(Body::from(format!("{:?} instances are terminating successfully", ids)))
            .unwrap(),
        Err(e) => failure_response(&format!("Failed to terminate instance IDs: {}", e), 500)?,
    };
    Ok(resp)
}

pub async fn terminate_running_instance_ids_from_cluster(cluster_name: &str) -> Result<Vec<String>, Error> {
    // Load AWS configuration
    let region_provider = RegionProviderChain::default_provider();
    let config = aws_config::from_env().region(region_provider).load().await;

    // Create EC2 client
    let client = Ec2Client::new(&config);

    // Describe EC2 instances
    let result = client.describe_instances().send().await?;

    // Collect instance IDs
    let mut instance_ids = Vec::new();
    if let Some(reservations) = result.reservations {
        for reservation in reservations {
            if let Some(instances) = reservation.instances {
                // all instances in the reservation
                for instance in instances {
                    let instance_id = instance.instance_id.as_deref().unwrap();
                    let is_running = instance.state.unwrap().name.unwrap() == "running".into();
                    // check is the instance is running
                    if is_running {
                        // print all tags
                        if let Some(tags) = instance.tags {
                            for tag in tags {
                                if tag.key.as_deref() == Some("karpenter.sh/discovery") {
                                    // check if the tag value is the same as the cluster name
                                    if tag.value.unwrap() == cluster_name {
                                        // add the instance id to the list
                                        instance_ids.push(instance_id.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // terminate instances
    for instance_id in &instance_ids {
        let instance_terminate_result = client.terminate_instances().instance_ids(instance_id).send().await?;
        if instance_terminate_result.terminating_instances.unwrap().len() == 0 {
            println!("Failed to terminate instance: {}", instance_id);
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to terminate instances")))
        } else {
            println!("Terminated instance: {}", instance_id);
        }
    }

    Ok(instance_ids)
    
}

fn failure_response(message: &str, status: u16) -> Result<Response<Body>, Error> {
    return Ok(Response::builder()
        .status(status)
        .body(message.into())
        .unwrap());
}