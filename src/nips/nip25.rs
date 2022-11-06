use crate::{events::EventPrepare, nostr_client::Client, utils::get_timestamp, Identity};

impl Client {
    /// React to an event
    ///
    /// '+' = Like\
    /// '-' = Dislike\
    /// Emoji = React with an emoji
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec!["wss://nostr-pub.wellorder.net"]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///
    /// // Here we react to an event
    /// client.react_to(&identity, "342060554ca30a9792f6e6959675ae734aed02c23e35037d2a0f72ac6316e83d", "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6", "+").unwrap();
    /// ```
    pub fn react_to(
        &mut self,
        identity: &Identity,
        event_id: &str,
        event_pub_key: &str,
        reaction: &str,
    ) -> Result<(), String> {
        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 7,
            tags: vec![
                vec!["e".to_string(), event_id.to_string()],
                vec!["p".to_string(), event_pub_key.to_string()],
            ],
            content: reaction.to_string(),
        }
        .to_event(identity);

        self.publish_event(&event)?;
        Ok(())
    }

    /// Add a like to an event
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec!["wss://nostr-pub.wellorder.net"]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// client.like(&identity, "342060554ca30a9792f6e6959675ae734aed02c23e35037d2a0f72ac6316e83d", "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6").unwrap();
    /// ```
    pub fn like(
        &mut self,
        identity: &Identity,
        event_id: &str,
        event_pub_key: &str,
    ) -> Result<(), String> {
        self.react_to(identity, event_id, event_pub_key, "+")
    }

    /// Add a dislike to an event
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec!["wss://nostr-pub.wellorder.net"]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// client.dislike(&identity, "342060554ca30a9792f6e6959675ae734aed02c23e35037d2a0f72ac6316e83d", "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6").unwrap();
    /// ```
    pub fn dislike(
        &mut self,
        identity: &Identity,
        event_id: &str,
        event_pub_key: &str,
    ) -> Result<(), String> {
        self.react_to(identity, event_id, event_pub_key, "-")
    }
}
