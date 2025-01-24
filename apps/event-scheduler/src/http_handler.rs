
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::{self as dynamodb, types::AttributeValue};
use chrono::{Local, DateTime};
use lambda_http::{tracing::subscriber::filter, Body, Error, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};
use tracing::error;
use std::collections::HashMap;
use std::env;
use crate::date_handler::parse_time;

pub const TABLE_NAME: &str = "EKS-Schedule";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Schedule {
    kind: String,
    start: String,
    end: String,
    cluster: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ScheduleItem {
    cluster_name: String,
    time: String,
    event_type: String,
    kind: String,
}

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    let configs = get_configs_vec(&client).await?;
    invoke_scaler(configs).await;
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("message".into())
        .map_err(Box::new)?;
    Ok(resp)
}

async fn get_configs_vec(
    client: &aws_sdk_dynamodb::Client,
) -> Result<Vec<ScheduleItem>, SdkError<aws_sdk_dynamodb::operation::scan::ScanError>> {
    let mut schedule_items_vec: Vec<ScheduleItem> = Vec::new();
    let mut all_cluster_names: Vec<String> = Vec::new();
    // query to get all items
    let results = client.scan().table_name(TABLE_NAME).send().await?;
    // println!("{:?}", results);

    if let Some(items) = results.items {
        for v in items.iter() {
            if let (Some(cluster), Some(start), Some(end), Some(kind)) = (
                v.get("cluster").and_then(|attr| attr.as_s().ok()),
                v.get("start").and_then(|attr| attr.as_s().ok()),
                v.get("end").and_then(|attr| attr.as_s().ok()),
                v.get("kind").and_then(|attr| attr.as_s().ok()),
            ) {
                let start_time = match parse_time(&start) {
                    Ok(parsed_time) => parsed_time.to_string(),
                    Err(e) => {
                        error!("Error parsing start time: {}", e);
                        start.clone()
                    }
                };
                let end_time = match parse_time(&end) {
                    Ok(parsed_time) => parsed_time.to_string(),
                    Err(e) => {
                        error!("Error parsing end time: {}", e);
                        start.clone()
                    }
                };
                all_cluster_names.push(cluster.clone());
                schedule_items_vec.push(ScheduleItem {
                    cluster_name: cluster.clone(),
                    time: start_time,
                    event_type: "start".to_string(),
                    kind: kind.clone(),
                });
                schedule_items_vec.push(ScheduleItem {
                    cluster_name: cluster.clone(),
                    time: end_time,
                    event_type: "end".to_string(),
                    kind: kind.clone(),
                });
            } else {
                error!("Missing expected attributes in DynamoDB item: {:?}", v);
            }
        }
    }

    // loop for each cluster name
    for cluster_name in all_cluster_names.iter() {
        // get length of schedule_items_vec where cluster_name == item.cluster_name
        let count = schedule_items_vec
            .iter()
            .filter(|&x| x.cluster_name == cluster_name.to_string() && x.kind == "custom")
            .count();
        // if custom exists remove default
        // if count (no of items with same cluster name and kind == custom) > 1 then remove all items with same cluster name and kind == deault
        if count > 1 {
            let index = schedule_items_vec
                .iter()
                .position(|x| *x.cluster_name == cluster_name.to_string() && x.kind == "default")
                .unwrap();
            schedule_items_vec.remove(index);
        }

    }

    Ok(schedule_items_vec)
}

async fn invoke_scaler(schedules: Vec<ScheduleItem>) {    

    for schedule in schedules.iter() {
        let schedule_time = schedule.time.parse::<DateTime<Local>>().unwrap();
        let current_time = Local::now();
        let difference_in_seconds = schedule_time.timestamp() - current_time.timestamp();
        // TODO: Change this to 1 min after testing
        // if difference is -600 to + 600 then scale - setting window of 10mins to scale
        if difference_in_seconds >= -600 && difference_in_seconds <= 600 {
            scaler(schedule.event_type.as_str(), schedule.cluster_name.as_str()).await;
        }
    }
}


async fn scaler(event_type: &str, cluster_name: &str) {
    let eks_set_cpu_url = env::var("EKS_SET_CPU_URL").clone().unwrap_or_default();
    let eks_terminate_ec2_url = env::var("EKS_TEMRINATE_EC2_URL").clone().unwrap_or_default();

    // let eks_set_cpu_url_value = eks_set_cpu_url.clone().unwrap_or_default();
 
    // match eks_set_cpu_url {
    //     Ok(val) => println!("EKS_SET_CPU_URL: {:?}", val),
    //     Err(e) => println!("Error EKS_SET_CPU_URL: {}", e),
    // }
    // let eks_terminate_ec2_url = env::var("EKS_TEMRINATE_EC2_URL");
    // match eks_terminate_ec2_url {
    //     Ok(val) => println!("EKS_TEMRINATE_EC2_URL: {:?}", val),
    //     Err(e) => println!("Error EKS_TEMRINATE_EC2_URL: {}", e),
    // }
    if event_type == "start" {
        // make a http request to EKS_SET_CPU_URL
        let mut map = HashMap::new();
        map.insert("cluster-name", cluster_name);
        map.insert("cpu-limit", "1000");

        let resp = reqwest::get(&eks_set_cpu_url).await;
        println!("{:?}", resp);
    } else {
           // make a http request to EKS_SET_CPU_URL
           let mut map = HashMap::new();
           map.insert("cluster-name", cluster_name);
           map.insert("cpu-limit", "0");
   
           let resp = reqwest::get(&eks_set_cpu_url).await;
           println!("{:?}", resp);
        //    TODO: Change the custom schedule in the DB to default
    }
}

