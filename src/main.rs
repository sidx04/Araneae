use anyhow::{anyhow, Result};
use html_parser::{Dom, Element, Node};
use std::process;

fn is_node(node: &Node) -> bool {
    match node {
        Node::Element(..) => true,
        _ => false,
    }
}

fn is_text(node: &Node) -> bool {
    match node {
        Node::Text(_) => true,
        _ => false,
    }
}

/* turn relative URLs into absolute URLS */
fn get_url(url: &str, root_url: &str) -> String {
    if url.starts_with("https://") || url.starts_with("http://") {
        return url.into();
    }

    format!(
        "{}/{}",
        root_url.strip_suffix("/").unwrap_or(root_url),
        url.strip_prefix("/").unwrap_or(url)
    )
}

fn crawl_element(elem: Element, root_url: &str) -> Result<Vec<String>> {
    let mut links = Vec::new();

    /* check for a link on the node */
    if elem.name == "a" {
        let href_attribute = elem
            .attributes
            .iter()
            .filter(|(name, _)| name.as_str() == "href")
            .last()
            .ok_or_else(|| anyhow!("missing href"));

        match href_attribute {
            Ok((_, Some(value))) => {
                log::info!("Link found: {:?}", value.clone());
                links.push(get_url(&value, &root_url));
            }
            _ => {
                log::error!("No links for {}...", elem.name);
            }
        }
    }

    for node in elem.children.iter().filter(|c| is_node(c)) {
        match node {
            Node::Element(elem) => {
                let mut children_links = crawl_element(elem.clone(), root_url)?;
                links.append(&mut children_links);
            }
            _ => {
                todo!();
            }
        }
    }

    Ok(links)
}

async fn crawl_url(url: &str) -> Result<Vec<String>> {
    let html = reqwest::get(url).await?.text().await?;
    let dom = Dom::parse(&html).unwrap();

    /* crawls all nodes */
    for child in dom.children {
        match child {
            Node::Element(elem) => {
                log::info!("{:?}:{:#?}", elem.name.clone(), crawl_element(elem, &url));
            }
            _ => {
                todo!();
            }
        }
    }

    let res = Vec::new();

    Ok(res)
}

async fn try_main() -> Result<()> {
    let _ = crawl_url("https://www.google.com").await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    match try_main().await {
        Ok(_) => {
            log::info!("Finished...");
        }
        Err(err) => {
            log::error!("Error: {:?}", err);
            process::exit(-1);
        }
    }
}
