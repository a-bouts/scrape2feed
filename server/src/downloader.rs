use reqwest::Result;
use reqwest::header::USER_AGENT;

pub fn download(url: String) -> Result<String> {
    let resp = reqwest::blocking::Client::new()
        .get(&url)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.16; rv:84.0) Gecko/20100101 Firefox/84.0",
        )
        .send()?;
    assert!(resp.status().is_success());

    Ok(resp.text()?)
}