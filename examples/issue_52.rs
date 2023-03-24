//! # Download an XML document and extract portions of it using XPath
//!
//! This example was requested by Protoculos in issue 52

use std::rc::Rc;
use log::info;
use reqwest::Client;
//use tokio::stream::StreamExt;
use futures::StreamExt;
use html_parser::{Dom, Node as DomNode};
use xrust::value::Value;
use xrust::item::{Item, Node, NodeType};
use xrust::evaluate::{Evaluator, Constructor};
use xrust::intmuttree::{RNode, NodeBuilder};
use xrust::xpath::parse;
use xrust::qname::QualifiedName;

// Setting a constant for the HTTP client timeout (in seconds)
const CLIENT_TIMEOUT: u64 = 10;

// Creating a Site structure to store the site URL and the vector of xpath expressions
//#[derive(Debug)]
struct Site {
    url: String,
    xpaths: Vec<(String, Vec<Constructor<RNode>>)>,
}

// Take a HTML document and return an xrust RNode.
// HTML documents are not well-formed, so we have to parse it with an HTML parser first, and then convert to a RNode.
fn make_doc(html: String) -> RNode {
    let d = NodeBuilder::new(NodeType::Document).build();
    Dom::parse(&html).expect("unable to parse HTML").children.iter()
	.for_each(|c| add_node(d.clone(), c.clone()));
    d
}
fn add_node(mut r: RNode, n: DomNode) {
    match n {
	DomNode::Element(e) => {
	    let t = NodeBuilder::new(NodeType::Element)
		.name(QualifiedName::new(None, None, e.name))
		.build();
	    e.attributes.iter()
		.for_each(|(k, v)| {
		    t.add_attribute(
			NodeBuilder::new(NodeType::Attribute)
			    .name(QualifiedName::new(None, None, k.to_string()))
			    .value(Value::from(v.clone().unwrap_or(String::new())))
			    .build()
		    ).expect("unable to add attribute")
		});
	    r.push(t).expect("unable to add element node");
	    e.children.iter()
		.for_each(|c| add_node(r.clone(), c.clone()))
	}
	DomNode::Text(s) => {
	    r.push(
		NodeBuilder::new(NodeType::Text)
		    .value(Value::from(s))
		    .build()
	    ).expect("unable to add text node")
	}
	_ => {}
    }
}

// Creating an asynchronous function for parsing one site
async fn parse_site(client: &Client, site: Site) -> Result<(), reqwest::Error> {
    // Getting an HTML page by URL
    let response = client.get(&site.url).send().await?;

    // Checking the response status
    if response.status().is_success() {
        // Convert the response to a string
        let body = response.text().await?;

	// Convert the string (HTML) to a xrust Document

        // Creating a Document object from the string
        let document = Rc::new(Item::Node(make_doc(body)));

        // Iterate over xpath expressions from the Site structure
        for (xpath, cons) in site.xpaths {
            // We find all the elements by the current xpath expression
	    let ev = Evaluator::new();
            if let Ok(nodes) = ev.evaluate(
		Some(vec![document.clone()]),
		Some(0),
		&cons,
		&NodeBuilder::new(NodeType::Document).build()
	    ) {
                // Output the number of elements found
                println!(
                    "{} elements found by {} on {}",
                    nodes.len(),
                    xpath,
                    site.url
                );
                // We output the text content of each element
                for node in nodes {
                    let text = node.to_string();
                    println!("{}", text);
                }
            } else {
                // We output an error message in case of an invalid xpath expression
                println!("Invalid xpath expression: {}", xpath);
            }
        }
    } else {
        // We output an error message in case of an unsuccessful request
        println!("Couldn't get the page: {}", site.url);
    }

    Ok(())
}

// Creating an asynchronous function to run parallel tasks for parsing a list of sites
async fn run(sites: Vec<Site>) -> Result<(), reqwest::Error> {
    info!("Starting the crawler"); // we record an informational message about the start of work

    // Creating an HTTP client with a timeout from a constant CLIENT_TIMEOUT
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(CLIENT_TIMEOUT))
        .build()
        .unwrap();

    // We create a stream from sites and convert it into a stream of tasks for parsing each site using the HTTP client
    let tasks = sites.into_iter().map(|site| parse_site(&client, site));

    // We perform all tasks at the same time and wait for their completion
    futures::future::join_all(tasks).await;

    info!("Finishing the crawler"); // we record an informational message about the completion of the work

    Ok(())
}

// Launching an asynchronous function using the tokio framework
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Initialize the env_logger logger with settings from the RUST_LOG environment variable
    env_logger::init();

    // Creating a vector of sites with URLs and xpath expressions for parsing
    // NB. xrust doesn't support abbreviated syntax (yet) so all of the paths must be written out in full :-(
    let sites = vec![
        Site {
            url: "https://www.rust-lang.org/learn".to_string(),
            xpaths: vec![
		("/descendant::title".to_string(), parse("/descendant::title").expect("unable to parse XPath expression")),
		("/descendant::h2".to_string(), parse("/descendant::h2").expect("unable to parse XPath expression"))
	    ],
        },
        Site {
            url: "https://www.python.org/about/".to_string(),
            xpaths: vec![
		("/descendant::title".to_string(), parse("/descendant::title").expect("unable to parse XPath expression")),
		("/descendant::h1".to_string(), parse("/descendant::h1").expect("unable to parse XPath expression"))
	    ],
        },
        Site {
            url: "https://www.haskell.org/".to_string(),
            xpaths: vec![
		("/descendant::title".to_string(), parse("/descendant::title").expect("unable to parse XPath expression")),
		("/descendant::a[attribute::class eq 'readmore']".to_string(), parse("/descendant::a[attribute::class eq 'readmore']").expect("unable to parse XPath expression"))
	    ],
        },
    ];

    // Run the run function with the passed vector of sites
    run(sites).await?;
    Ok(())
}
