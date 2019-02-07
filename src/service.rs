use crate::domain::{ContentDTO, Item, ItemList, Link};

pub fn convert_content_to_item(content: ContentDTO) -> Item {
    Item {
        created_at: "".to_string(),
        link: Link { url: "".to_string() },
        text: "".to_string(),
    }
}

pub fn add_item_to_list(item: Item, mut list: ItemList) -> ItemList {
    list.insert(0, item);
    list
}

#[cfg(test)]
mod tests {
    use crate::domain::{Item, ItemList, Link};
    use super::add_item_to_list;

    #[test]
    fn should_add_item_to_the_beginning_of_the_list() {
        // given
        let list: ItemList = vec![
            Item {
                created_at: String::from("2018-02-07T20:43:44"),
                link: Link { url: String::from("https://example.com") },
                text: String::from("Some link"),
            },
            Item {
                created_at: String::from("2018-01-29T21:10:00"),
                link: Link { url: String::from("https://example.com/second") },
                text: String::from("Second link"),
            },
        ];

        let item = Item {
            created_at: String::from("2018-02-10T10:00:00"),
            link: Link { url: String::from("https://example.com/new-link") },
            text: String::from("Next link"),
        };

        // when
        let list = add_item_to_list(item, list);

        // then
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].text, String::from("Next link"));
        assert_eq!(list[1].text, String::from("Some link"));
        assert_eq!(list[2].text, String::from("Second link"));
    }
}
