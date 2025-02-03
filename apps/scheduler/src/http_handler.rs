use crate::date_handler::parse_time;
use anyhow::{anyhow, Result};
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::types::{AttributeValue, ReturnConsumedCapacity};
use chrono::{DateTime, Timelike, Utc};
use chrono_tz::Asia::Seoul;
use chrono_tz::Tz;

use lambda_http::{Body, Error, Request, Response};
use serde::{Deserialize, Serialize};
use std::env;
use tracing::error;

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

pub(crate) async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    let configs = get_configs_vec(&client).await?;
    println!("Valid Configs: {:?}", configs);
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
    let table_name: String = env::var("TABLE_NAME").clone().unwrap_or_default();
    // query to get items where disabled = false

    let results = client
        .scan()
        .table_name(table_name)
        .filter_expression("disabled = :disabled")
        .expression_attribute_values(":disabled", AttributeValue::Bool(false))
        .send()
        .await?;

    if let Some(items) = results.items {
        for v in items.iter() {
            if let (Some(cluster), Some(start), Some(end), Some(kind)) = (
                v.get("cluster").and_then(|attr| attr.as_s().ok()),
                v.get("start").and_then(|attr| attr.as_s().ok()),
                v.get("end").and_then(|attr| attr.as_s().ok()),
                v.get("kind").and_then(|attr| attr.as_s().ok()),
            ) {
                // println!("{:?}", parse_time(&start));
                // let start_time = match parse_time(&start) {
                //     Ok(parsed_time) => parsed_time.to_string(),
                //     Err(e) => {
                //         error!("Error parsing start time: {}", e);
                //         start.clone()
                //     }
                // };
                // let end_time = match parse_time(&end) {
                //     Ok(parsed_time) => parsed_time.to_string(),
                //     Err(e) => {
                //         error!("Error parsing end time: {}", e);
                //         end.clone()
                //     }
                // };
                all_cluster_names.push(cluster.clone());
                schedule_items_vec.push(ScheduleItem {
                    cluster_name: cluster.clone(),
                    time: start.clone(),
                    event_type: "start".to_string(),
                    kind: kind.clone(),
                });
                schedule_items_vec.push(ScheduleItem {
                    cluster_name: cluster.clone(),
                    time: end.clone(),
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
    // remove duplicates
    schedule_items_vec.dedup_by(|a, b| a.cluster_name == b.cluster_name && a.time == b.time);

    Ok(schedule_items_vec)
}

async fn invoke_scaler(schedules: Vec<ScheduleItem>) {
    for schedule in schedules.iter() {
        let db_time = parse_time(&schedule.time);
        // // let db_time = DateTime::parse_from_rfc3339(corrected_time.as_str()).unwrap();
        let [hours, mins] = match db_time {
            Ok([hours, mins]) => [hours, mins],
            Err(e) => {
                error!("Error parsing time: {}", e);
                continue;
            }
        };
        // let current_time: DateTime<Tz> = Utc::now().with_timezone(&Seoul);

        let current_time: DateTime<Tz> = Utc::now().with_timezone(&Seoul);
        let scheduled_time = current_time.with_hour(hours).unwrap().with_minute(mins).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap();
        let difference_in_seconds = scheduled_time.timestamp() - current_time.timestamp();
        // TODO: Change this to 1 min after testing
        // if difference is -600 to + 600 then scale - setting window of 5mins to scale
        println!("Current Time:{:?}", current_time);
        println!("Schedule Time:{:?}", scheduled_time);
        println!("Difference:{:?}", difference_in_seconds);
        if difference_in_seconds >= -60 && difference_in_seconds <= 60 {
            println!(
                "Scaling ({}) for cluster: {} at {}",
                schedule.event_type,
                schedule.cluster_name,
                current_time.format("%Y-%m-%d %H:%M:%S")
            );
            scaler(schedule.event_type.as_str(), schedule.cluster_name.as_str()).await;
        } else {
            println!(
                "No scaling ({}) for cluster: {} at {}",
                schedule.event_type,
                schedule.cluster_name,
                current_time.format("%Y-%m-%d %H:%M:%S")
            );
        }
    }
    // scaler("start", "wave-eks-istio-amd").await;
}

async fn scaler(event_type: &str, cluster_name: &str) {
    let eks_set_cpu_url = env::var("EKS_SET_CPU_URL").clone().unwrap_or_default();
    let eks_terminate_ec2_url = env::var("EKS_TEMRINATE_EC2_URL")
        .clone()
        .unwrap_or_default();

    println!("{:?}", eks_set_cpu_url);
    if event_type == "start" {
        let params = [("cluster-name", cluster_name), ("cpu-limit", "1000")];
        // add map to request payload
        let url = reqwest::Url::parse_with_params(&eks_set_cpu_url, &params).unwrap();
        let resp = reqwest::get(url).await;
        println!("{:?}", resp);
    } else {
        let params = [("cluster-name", cluster_name), ("cpu-limit", "0")];
        let ec2_params = [("cluster-name", cluster_name)];
        // add map to request payload
        let url = reqwest::Url::parse_with_params(&eks_set_cpu_url, &params).unwrap();
        let ec2_url = reqwest::Url::parse_with_params(&eks_terminate_ec2_url, &ec2_params).unwrap();

        let resp = reqwest::get(url).await;
        let ec2_resp = reqwest::get(ec2_url).await;
        println!("{:?}", resp);
        println!("{:?}", ec2_resp);
        //Change the custom schedule in the DB to default
        let _update_custom_schedule = reset_custom_schedule(cluster_name).await;
    }
}

async fn reset_custom_schedule(cluster_name: &str) -> Result<()> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    let table_name: String = env::var("TABLE_NAME").clone().unwrap_or_default();

    let scan_attr = AttributeValue::S(cluster_name.to_string() + "_default");
    let results = client
        .scan()
        .table_name(table_name.clone())
        .filter_expression("id = :id")
        .expression_attribute_values(":id", scan_attr)
        .send()
        .await
        .map_err(|e| anyhow!("Failed to scan DynamoDB: {}", e))?;
    let default_schedule = results
        .items
        .into_iter()
        .nth(0)
        .ok_or_else(|| anyhow!("No default schedule found"))?;
    //check if default schedule exists

    let start = AttributeValue::S(String::from(
        default_schedule[0].get("start").unwrap().as_s().unwrap(),
    ));
    let end = AttributeValue::S(String::from(
        default_schedule[0].get("end").unwrap().as_s().unwrap(),
    ));
    let kind = AttributeValue::S(String::from(
        "custom",
    ));
    let disabled = AttributeValue::Bool(
        *default_schedule[0]
            .get("disabled")
            .unwrap()
            .as_bool()
            .unwrap(),
    );
    let cluster = AttributeValue::S(String::from(
        default_schedule[0].get("cluster").unwrap().as_s().unwrap(),
    ));
    let id = AttributeValue::S(String::from(cluster_name.to_string() + "_custom"));

    let request = client
        .put_item()
        .table_name(table_name)
        .item("id", id)
        .item("start", start)
        .item("end", end)
        .item("kind", kind)
        .item("cluster", cluster)
        .item("disabled", disabled)
        .return_consumed_capacity(ReturnConsumedCapacity::Total);

    let resp = request.send().await?;
    println!("Updated custom schedule to default for {}", cluster_name);
    println!("{:?}", resp);

    Ok(())
}
