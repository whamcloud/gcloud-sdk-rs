#![allow(dead_code)]
use gcloud_sdk::google_rest_apis::lustre_v1alpha::Instance;

async fn instances_create(
    project: &str,
    location: &str,
    instance_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let google_rest_client = gcloud_sdk::GoogleRestApi::new().await?;

    let instance = Instance {
        capacity_gib: Some("16384".to_string()),
        create_time: None,
        description: Some("Jon's test lustre instance".to_string()),
        filesystem: Some(instance_name.to_string()),
        labels: None,
        mount_point: None,
        name: Some(instance_name.to_string()),
        network: Some("projects/8589356148/global/networks/default".to_string()),
        state: None,
        update_time: None,
    };

    let response = gcloud_sdk::google_rest_apis::lustre_v1alpha::projects_api::autopush_lustre_sandbox_projects_locations_instances_create(
        &google_rest_client.create_google_lustre_v1alpha_config().await?,
        gcloud_sdk::google_rest_apis::lustre_v1alpha::projects_api::AutopushLustreSandboxPeriodProjectsPeriodLocationsPeriodInstancesPeriodCreateParams  {
            parent: format!("projects/{project}/locations/{location}"),
            access_token: None,
            alt: None,
            callback: None,
            fields: None,
            key: None,
            oauth_token: None,
            pretty_print: None,
            quota_user: None,
            upload_protocol: None,
            upload_type: None,
            dollar_xgafv: None,
            instance_id: Some(instance_name.to_string()),
            instance: Some(instance),
            request_id: None,
        }
    ).await?;
    println!("{:#?}", response);
    Ok(())
}

async fn instances_delete(
    project: &str,
    location: &str,
    instance_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let google_rest_client = gcloud_sdk::GoogleRestApi::new().await?;

    let response = gcloud_sdk::google_rest_apis::lustre_v1alpha::projects_api::autopush_lustre_sandbox_projects_locations_instances_delete(
        &google_rest_client.create_google_lustre_v1alpha_config().await?,
        gcloud_sdk::google_rest_apis::lustre_v1alpha::projects_api::AutopushLustreSandboxPeriodProjectsPeriodLocationsPeriodInstancesPeriodDeleteParams {
            name: format!("projects/{project}/locations/{location}/instances/{instance_name}"),
            access_token: None,
            alt: None,
            callback: None,
            fields: None,
            key: None,
            oauth_token: None,
            pretty_print: None,
            quota_user: None,
            upload_protocol: None,
            upload_type: None,
            dollar_xgafv: None,
            request_id: None,
        }
    ).await?;
    println!("{:#?}", response);
    Ok(())
}

async fn instances_get(
    project: &str,
    location: &str,
    instance_name: &str,
) -> Result<gcloud_sdk::google_rest_apis::lustre_v1alpha::Instance, Box<dyn std::error::Error>> {
    let google_rest_client = gcloud_sdk::GoogleRestApi::new().await?;

    let response = gcloud_sdk::google_rest_apis::lustre_v1alpha::projects_api::autopush_lustre_sandbox_projects_locations_instances_get(
        &google_rest_client.create_google_lustre_v1alpha_config().await?,
        gcloud_sdk::google_rest_apis::lustre_v1alpha::projects_api::AutopushLustreSandboxPeriodProjectsPeriodLocationsPeriodInstancesPeriodGetParams  {
            name: format!("projects/{project}/locations/{location}/instances/{instance_name}"),
            access_token: None,
            alt: None,
            callback: None,
            fields: None,
            key: None,
            oauth_token: None,
            pretty_print: None,
            quota_user: None,
            upload_protocol: None,
            upload_type: None,
            dollar_xgafv: None,
        }
    ).await?;
    Ok(response)
}

async fn instances_list(
    project: &str,
    location: &str,
) -> Result<Vec<gcloud_sdk::google_rest_apis::lustre_v1alpha::Instance>, Box<dyn std::error::Error>>
{
    let google_rest_client = gcloud_sdk::GoogleRestApi::new().await?;

    let response = gcloud_sdk::google_rest_apis::lustre_v1alpha::projects_api::autopush_lustre_sandbox_projects_locations_instances_list(
        &google_rest_client.create_google_lustre_v1alpha_config().await?,
        gcloud_sdk::google_rest_apis::lustre_v1alpha::projects_api::AutopushLustreSandboxPeriodProjectsPeriodLocationsPeriodInstancesPeriodListParams  {
            parent: format!("projects/{project}/locations/{location}"),
            access_token: None,
            alt: None,
            callback: None,
            fields: None,
            key: None,
            oauth_token: None,
            pretty_print: None,
            quota_user: None,
            upload_protocol: None,
            upload_type: None,
            dollar_xgafv: None,
            filter: None,
            order_by: None,
            page_size: None,
            page_token: None,
        }
    ).await?;

    println!("{:#?}", response);

    Ok(vec![])
}
//"projects/ddn-e2e-testing/locations/us-central1-a/operations/operation-1736279872873-62b23290da4da-6d3fcf0c-8eef803c",

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Debug logging
    //let subscriber = tracing_subscriber::fmt()
    //    .with_max_level(tracing::Level::TRACE)
    //    .with_env_filter("gcloud_sdk=debug")
    //    .finish();
    //tracing::subscriber::set_global_default(subscriber)?;

    // Detect Google project ID using environment variables PROJECT_ID/GCP_PROJECT_ID
    // or GKE metadata server when the app runs inside GKE
    let google_project_id = gcloud_sdk::GoogleEnvironment::detect_google_project_id().await
        .expect("No Google Project ID detected. Please specify it explicitly using env variable: PROJECT_ID");

    let location = "us-central1-a";
    //dbg!(instances_get(&google_project_id, &location, "emf-sandbox").await);
    //dbg!(instances_create(&google_project_id, &location, "emfjdp").await);
    //dbg!(instances_delete(&google_project_id, &location, "emfjdp").await);
    instances_list(&google_project_id, &location).await?;

    Ok(())
}
