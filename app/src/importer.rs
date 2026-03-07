use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::Value;
use thiserror::Error;
use url::Url;

use crate::models::SongMetadata;

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("URL inválida")]
    InvalidUrl,
    #[error("Apenas links de música do Suno são aceitos")]
    UnsupportedHost,
    #[error("Falha ao baixar página: {0}")]
    Download(String),
}

pub fn validate_suno_song_url(source_url: &str) -> Result<Url, ImportError> {
    let url = Url::parse(source_url.trim()).map_err(|_| ImportError::InvalidUrl)?;

    let is_suno = url
        .host_str()
        .map(|host| host == "suno.com" || host.ends_with(".suno.com"))
        .unwrap_or(false);

    let is_song_path = url.path().starts_with("/song/");

    if !is_suno || !is_song_path {
        return Err(ImportError::UnsupportedHost);
    }

    Ok(url)
}

fn normalize_text(raw: &str) -> Option<String> {
    let normalized = raw
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string();

    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

fn parse_metadata_from_html(source_url: &str, html: &str) -> SongMetadata {
    let doc = Html::parse_document(html);

    let title_selector = Selector::parse("title").expect("selector válido");
    let og_title_selector = Selector::parse("meta[property='og:title']").expect("selector válido");
    let og_audio_selector = Selector::parse("meta[property='og:audio']").expect("selector válido");
    let description_selector =
        Selector::parse("meta[name='description'], meta[property='og:description']")
            .expect("selector válido");
    let ld_json_selector =
        Selector::parse("script[type='application/ld+json']").expect("selector válido");

    let title = doc
        .select(&og_title_selector)
        .next()
        .and_then(|n| n.value().attr("content"))
        .and_then(normalize_text)
        .or_else(|| {
            doc.select(&title_selector)
                .next()
                .map(|n| n.text().collect::<String>())
                .and_then(|t| normalize_text(&t))
        });

    let audio_url = doc
        .select(&og_audio_selector)
        .next()
        .and_then(|n| n.value().attr("content"))
        .and_then(normalize_text);

    let mut lyrics = doc
        .select(&description_selector)
        .next()
        .and_then(|n| n.value().attr("content"))
        .and_then(normalize_text);

    for script in doc.select(&ld_json_selector) {
        let payload = script.text().collect::<String>();
        let parsed: Value = match serde_json::from_str(&payload) {
            Ok(value) => value,
            Err(_) => continue,
        };

        let candidate = parsed
            .get("lyrics")
            .and_then(Value::as_str)
            .or_else(|| parsed.get("articleBody").and_then(Value::as_str))
            .or_else(|| parsed.get("description").and_then(Value::as_str))
            .and_then(normalize_text);

        if lyrics.is_none() && candidate.is_some() {
            lyrics = candidate;
        }

        if audio_url.is_none() {
            if let Some(audio) = parsed
                .get("audio")
                .and_then(Value::as_object)
                .and_then(|audio| audio.get("contentUrl"))
                .and_then(Value::as_str)
                .and_then(normalize_text)
            {
                return SongMetadata {
                    source_url: source_url.to_string(),
                    title,
                    lyrics,
                    audio_url: Some(audio),
                };
            }
        }
    }

    SongMetadata {
        source_url: source_url.to_string(),
        title,
        lyrics,
        audio_url,
    }
}

pub async fn import_from_url(
    client: &Client,
    source_url: &str,
) -> Result<SongMetadata, ImportError> {
    let validated = validate_suno_song_url(source_url)?;

    let html = client
        .get(validated.as_str())
        .send()
        .await
        .map_err(|e| ImportError::Download(e.to_string()))?
        .error_for_status()
        .map_err(|e| ImportError::Download(e.to_string()))?
        .text()
        .await
        .map_err(|e| ImportError::Download(e.to_string()))?;

    Ok(parse_metadata_from_html(validated.as_str(), &html))
}

#[cfg(test)]
mod tests {
    use super::{parse_metadata_from_html, validate_suno_song_url, ImportError};

    #[test]
    fn validates_only_suno_song_urls() {
        assert!(validate_suno_song_url("https://suno.com/song/abc").is_ok());
        assert!(validate_suno_song_url("https://app.suno.com/song/abc").is_ok());

        assert!(matches!(
            validate_suno_song_url("notaurl"),
            Err(ImportError::InvalidUrl)
        ));
        assert!(matches!(
            validate_suno_song_url("https://example.com/song/abc"),
            Err(ImportError::UnsupportedHost)
        ));
        assert!(matches!(
            validate_suno_song_url("https://suno.com/playlist/abc"),
            Err(ImportError::UnsupportedHost)
        ));
    }

    #[test]
    fn extracts_metadata_from_html() {
        let html = r#"
        <html>
          <head>
            <title>  Minha Música  </title>
            <meta property='og:title' content='Minha Música OG'>
            <meta property='og:audio' content='https://cdn.suno.com/audio.mp3'>
            <meta name='description' content='  Letra resumida  '>
            <script type='application/ld+json'>
              {"lyrics": "Verso 1   Verso 2"}
            </script>
          </head>
        </html>
        "#;

        let song = parse_metadata_from_html("https://suno.com/song/abc", html);

        assert_eq!(song.title.as_deref(), Some("Minha Música OG"));
        assert_eq!(
            song.audio_url.as_deref(),
            Some("https://cdn.suno.com/audio.mp3")
        );
        assert_eq!(song.lyrics.as_deref(), Some("Letra resumida"));
    }
}
