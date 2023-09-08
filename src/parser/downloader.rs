use anyhow::{Result, Ok};

pub fn download() -> Result<String> {
    let body = reqwest::blocking::get("https://core.telegram.org/bots/api")?.text()?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use crate::parser::downloader::download;

    #[test]
    fn has_title_and_doctype() {
        let page = download().unwrap();
        let mut iter = page.split("\n");
        let doctype = iter.next().unwrap();
        let title = iter.skip(3).next().unwrap().trim();
        assert_eq!(doctype, "<!DOCTYPE html>");
        assert_eq!(title, "<title>Telegram Bot API</title>");
    }
}