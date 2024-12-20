#![allow(dead_code)]

use gcloud_sdk::GoogleRestApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Debug logging
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        //.with_env_filter("gcloud_sdk=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Detect Google project ID using environment variables PROJECT_ID/GCP_PROJECT_ID
    // or GKE metadata server when the app runs inside GKE
    let google_project_id = gcloud_sdk::GoogleEnvironment::detect_google_project_id().await
        .expect("No Google Project ID detected. Please specify it explicitly using env variable: PROJECT_ID");

    let google_rest_client = gcloud_sdk::GoogleRestApi::new().await?;

    let client = google_rest_client.create_google_lustre_v1alpha_config().await?;
    println!("JDP");
    let response = gcloud_sdk::google_rest_apis::lustre_v1alpha::projects_api::autopush_lustre_sandbox_projects_locations_instances_list(
        &client,
        gcloud_sdk::google_rest_apis::lustre_v1alpha::projects_api::AutopushLustreSandboxPeriodProjectsPeriodLocationsPeriodInstancesPeriodListParams  {
            parent: format!("v1alpha/projects/{google_project_id}/locations/us-central1-a/instances"),
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

    println!("{:?}", response);

    Ok(())
}
