use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use reqwest::Error as ReqwestError;
use rss::{ChannelBuilder, Item, ItemBuilder};
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use chrono::{DateTime, TimeZone, Utc};

#[derive(Deserialize, Debug)]
struct ApiResponse {
    hits: Hits,
}

#[derive(Deserialize, Debug)]
struct Hits {
    hits: Vec<Hit>,
}

#[derive(Deserialize, Debug)]
struct Hit {
    metadata: Metadata,
    links: Links,
    created: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Metadata {
    title: Option<Vec<String>>,
    authors: Option<Vec<Author>>,
    abstracts: Option<Vec<Abstract>>,
    titles: Option<Vec<Title>>,
    citation_count: Option<u32>,
    #[serde(rename = "control_number")]
    control_number: Option<u32>,
}

#[derive(Deserialize, Debug)]
struct Author {
    full_name: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Abstract {
    source: Option<String>,
    value: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Title{
    title: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Links {
    json: Option<String>,
    latex_eu: Option<String>,
    latex_us: Option<String>,
    bibtex: Option<String>,
}

#[tokio::main]
async fn main() {
    // Define a router with a single route that handles GET requests
    let app = Router::new().route("/", get(handle_request));

    // Define the address and port to bind the server to
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    // Run the HTTP server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler function for GET requests to "/"
async fn handle_request(Query(params): Query<HashMap<String, String>>) -> Result<Response, StatusCode> {
    // Construct the InspireHEP API URL with all query parameters forwarded
    let base_url = "https://inspirehep.net/api/literature";
    let url = format!("{}?{}", base_url, serde_urlencoded::to_string(&params).unwrap());
    println!("Fetching data from: {}", url);

    // Fetch data from the InspireHEP API
    let api_response = match fetch_data(&url).await {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Convert the data to RSS format
    let rss_feed = generate_rss_feed(api_response);

    // print the RSS feed to stdout
    println!("{}", rss_feed);

    // Return the RSS feed as an XML response
    Ok(rss_feed.into_response())
}

// Function to fetch data from InspireHEP API
async fn fetch_data(url: &str) -> Result<ApiResponse, ReqwestError> {
    let response = reqwest::get(url).await?;
    let api_response = response.json::<ApiResponse>().await?;
    Ok(api_response)
}

// Function to convert the API response to an RSS feed
fn generate_rss_feed(api_response: ApiResponse) -> String {
    let items: Vec<Item> = api_response.hits.hits.iter().map(|hit| convert_to_rss_item(hit)).collect();

    let channel = ChannelBuilder::default()
        .title("InspireHEP Literature")
        .link("https://inspirehep.net")
        .description("Literature of specified request")
        .items(items)
        .build();

    channel.to_string()
}

// Convert each API Hit to an RSS Item
fn convert_to_rss_item(hit: &Hit) -> Item {
    let title = hit.metadata.titles.as_ref()
        .and_then(|titles| titles.get(0).and_then(|ttl| ttl.title.clone()))
        .unwrap_or_else(|| "No Title".to_string());

    //let link = hit.links.json.clone().unwrap_or_else(|| "https://inspirehep.net".to_string());
    let link = format!("https://inspirehep.net/literature/{}", hit.metadata.control_number.unwrap_or(0));


    let description = hit.metadata.abstracts.as_ref()
        .and_then(|abstracts| abstracts.get(0).and_then(|abs| abs.value.clone()))
        .unwrap_or_else(|| "No abstract available.".to_string());

    let author_names = hit.metadata.authors.as_ref()
        .map(|authors| {
            authors.iter().filter_map(|author| author.full_name.clone()).collect::<Vec<String>>().join(", ")
        })
        .unwrap_or_else(|| "Unknown Authors".to_string());

    // Parse the timestamp as a DateTime<Utc>
    let datetime: DateTime<Utc> = hit.created
        .as_deref()
        .and_then(|s| s.parse::<DateTime<Utc>>().ok())
        .unwrap_or_else(|| Utc.timestamp_opt(0,0).single().unwrap());
    
    // Format it to the RFC 822 format required by RSS (like "Tue, 02 Apr 1991 00:00:00 +0000")
    let rss_date = datetime.format("%a, %d %b %Y %H:%M:%S %z").to_string();

    ItemBuilder::default()
        .title(Some(title))
        .link(Some(link))
        .description(Some(description))
        .author(Some(author_names))
        .pub_date(Some(rss_date))
        .build()
}

