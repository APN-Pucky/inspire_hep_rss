use reqwest;
use reqwest::Error;
use serde::Deserialize;
use rss::{ChannelBuilder, Item, ItemBuilder};

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
}

#[derive(Deserialize, Debug)]
struct Metadata {
    title: Option<Vec<String>>,
    authors: Option<Vec<Author>>,
    abstracts: Option<Vec<Abstract>>,
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
struct Links {
    json: Option<String>,
    latex_eu: Option<String>,
    latex_us: Option<String>,
    bibtex: Option<String>,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://inspirehep.net/api/literature?sort=mostcited&size=10&q=a%20E.Witten.1";
    let response = reqwest::get(url).await?.json::<ApiResponse>().await?;

    // Convert each `Hit` into an RSS `Item`
    let items: Vec<Item> = response.hits.hits.iter().map(|hit| convert_to_rss_item(hit)).collect();

    // Build the RSS Channel
    let channel = ChannelBuilder::default()
        .title("InspireHEP Literature on E. Witten")
        .link("https://inspirehep.net")
        .description("Most cited literature on E. Witten")
        .items(items)
        .build()
        //.unwrap()
        ;

    // Write the RSS feed to stdout (or save it to a file if preferred)
    let rss_string = channel.to_string();
    println!("{}", rss_string);

    Ok(())
}

// Function to convert each API Hit to an RSS Item
fn convert_to_rss_item(hit: &Hit) -> Item {
    let title = hit.metadata.title.as_ref()
        .and_then(|titles| titles.get(0).cloned())
        .unwrap_or_else(|| "No Title".to_string());
    
    let link = hit.links.json.clone().unwrap_or_else(|| "https://inspirehep.net".to_string());

    let description = hit.metadata.abstracts.as_ref()
        .and_then(|abstracts| abstracts.get(0).and_then(|abs| abs.value.clone()))
        .unwrap_or_else(|| "No abstract available.".to_string());

    let author_names = hit.metadata.authors.as_ref()
        .map(|authors| {
            authors.iter().filter_map(|author| author.full_name.clone()).collect::<Vec<String>>().join(", ")
        })
        .unwrap_or_else(|| "Unknown Authors".to_string());

    ItemBuilder::default()
        .title(Some(title))
        .link(Some(link))
        .description(Some(description))
        .author(Some(author_names))
        .build()
        //.unwrap()
}


//#[tokio::main]
//async fn main() -> Result<(), Error> {
//    let url = "https://inspirehep.net/api/literature?sort=mostcited&size=10&q=a%20E.Witten.1";
//    
//    let response = reqwest::get(url).await?.json::<ApiResponse>().await?;
//    
//    // Print the parsed response for demonstration
//    for hit in response.hits.hits {
//        println!("Control Number: {:?}", hit.metadata.control_number);
//        if let Some(authors) = hit.metadata.authors {
//            for author in authors {
//                println!("Author: {:?}", author.full_name);
//            }
//        }
//        if let Some(abstracts) = hit.metadata.abstracts {
//            for abs in abstracts {
//                println!("Abstract: {:?}", abs.value);
//            }
//        }
//        println!("Citations: {:?}", hit.metadata.citation_count);
//        println!("Links: {:?}", hit.links.json);
//        println!("----------");
//    }
//
//    Ok(())
//}