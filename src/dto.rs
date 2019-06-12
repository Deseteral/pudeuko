use crate::domain::{Item, Link};
use chrono::Utc;
use message_meta::parse;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentDTO {
    pub text: String,
}

impl From<ContentDTO> for Item {
    fn from(content: ContentDTO) -> Self {
        let meta = parse(content.text.to_owned());
        let link = if !meta.links.is_empty() {
            Some(Link {
                url: meta.links[0].url.to_owned(),
            })
        } else {
            None
        };

        Self {
            id: nanoid::generate(8),
            text: meta.message,
            link,
            created_at: Utc::now().to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ContentDTO, Item};

    #[test]
    fn should_create_item_from_simple_content_text() {
        // given
        let content = ContentDTO {
            text: String::from("https://example.com"),
        };

        // when
        let item = Item::from(content);

        // then
        assert_eq!(item.id.len(), 8);
        assert_eq!(item.text, "https://example.com");
        assert_eq!(item.link.unwrap().url, "https://example.com");
    }

    #[test]
    fn should_create_item_from_content_text() {
        // given
        let content = ContentDTO {
            text: String::from("Check out this link: https://example.com it is awesome!"),
        };

        // when
        let item = Item::from(content);

        // then
        assert_eq!(item.id.len(), 8);
        assert_eq!(
            item.text,
            "Check out this link: https://example.com it is awesome!"
        );
        assert_eq!(item.link.unwrap().url, "https://example.com");
    }

    #[test]
    fn should_create_item_without_link() {
        // given
        let content = ContentDTO {
            text: String::from("This is something worth remembering!"),
        };

        // when
        let item = Item::from(content);

        // then
        assert_eq!(item.id.len(), 8);
        assert_eq!(item.text, "This is something worth remembering!");
        assert_eq!(item.link.is_none(), true);
    }
}
