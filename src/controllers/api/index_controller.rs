use meilisearch_sdk::{
    indexes::*,
    client::*,
    search::*,
    settings::*
};
use serde::{Serialize, Deserialize};
use std::{io::prelude::*, fs::File};
use futures::executor::block_on;

struct IndexController {

}

struct IndexIndex {

}

impl IndexController {


    fn index() -> IndexIndex {
        block_on(async move {
            let client = Client::new("http://localhost:7700", Some("aSampleMasterKey"));

            // Reading and parsing the file
            let mut file = File::open("movies.json")
                .unwrap();
            let mut content = String::new();
            file
                .read_to_string(&mut content)
                .unwrap();
            let movies_docs: Vec<Movie> = serde_json::from_str(&content)
                .unwrap();

            // Adding documents
            client
                .index("movies")
                .add_documents(&movies_docs, None)
                .await
                .unwrap();
        });

        IndexIndex {}
    }
}