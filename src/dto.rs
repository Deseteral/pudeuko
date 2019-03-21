use crate::domain::{Item, Link};
use chrono::Utc;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentDTO {
    pub text: String,
}

impl From<ContentDTO> for Item {
    fn from(content: ContentDTO) -> Self {
        Self {
            id: nanoid::generate(8),
            text: content.text.to_owned(),
            link: Link {
                url: content.text.to_owned(),
            },
            created_at: Utc::now().to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ContentDTO, Item};

    #[test]
    fn should_create_item_from_content_text() {
        // given
        let content = ContentDTO {
            text: String::from("https://example.com"),
        };

        // when
        let item = Item::from(content);

        // then
        assert_eq!(item.id.len(), 8);
        assert_eq!(item.text, "https://example.com");
        assert_eq!(item.link.url, "https://example.com");
    }
}
