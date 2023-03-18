use reqwest::Result;
use reqwest::header::USER_AGENT;
use html5ever::{ParseOpts, parse_document};
use html5ever::tree_builder::{TreeBuilderOpts, TreeSink};
use rcdom::{Handle, NodeData, RcDom, SerializableHandle};
use html5ever::serialize::{serialize};
use html5ever::tendril::TendrilSink;

pub async fn download(url: String) -> Result<String> {
    let resp = reqwest::Client::new()
        .get(&url)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.16; rv:84.0) Gecko/20100101 Firefox/84.0",
        )
        .send().await?;
    assert!(resp.status().is_success());

    Ok(clean(resp.text().await?))
}

fn clean(data: String) -> String {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let mut dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut data.as_bytes())
        .unwrap();

    let document = dom.get_document();

    dom.walk_clean(&document);

    let mut buf = Vec::new();

    let doc: SerializableHandle = document.into();
    serialize(&mut buf, &doc, Default::default());

    std::str::from_utf8(buf.as_slice()).unwrap().to_string()
}

trait Walk {
    fn remove_from_parent2(&mut self, target: &Handle);
    fn to_delete(&mut self, handle: &Handle) -> bool {
        let node = handle;

        match node.data {
            NodeData::Document => {}
            NodeData::Doctype {
                name: _,
                public_id: _,
                system_id: _,
            } => {}
            NodeData::Text { contents: _ } => {}
            NodeData::Comment { contents: _ } => {}
            NodeData::ProcessingInstruction { .. } => {}

            NodeData::Element {
                ref name,
                
                ..
            } => {
                if name.local.to_lowercase() == String::from("noscript") || name.local.to_lowercase() == String::from("script") {
                    return true
                }
            },
        }

        false
    }
    fn walk_clean(&mut self, handle: &Handle) {
        let node = handle;

        let mut children = node.children.borrow_mut();

        children.retain(|x| !self.to_delete(x));

        for child in children.iter() {
            self.walk_clean(child);
        }
    }
}

impl Walk for RcDom {
    fn remove_from_parent2(&mut self, target: &Handle) {
        self.remove_from_parent(target);
    }
}
