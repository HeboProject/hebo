// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[derive(Debug, Default, Clone)]
pub struct Topic {
    topic: String,
    parts: Vec<TopicPart>,
}

#[derive(Debug)]
pub enum TopicError {
    EmptyTopic,
    TooManyData,
    InvalidChar,
    ContainsWildChar,
}

impl PartialEq for Topic {
    fn eq(&self, other: &Self) -> bool {
        self.topic.eq(&other.topic)
    }
}

impl Topic {
    // TODO(Shaohua): Replace with `std::str::FromStr` trait.
    pub fn parse(s: &str) -> Result<Topic, TopicError> {
        let parts = Topic::parse_parts(s)?;
        Ok(Topic {
            topic: s.to_string(),
            parts,
        })
    }

    fn parse_parts(s: &str) -> Result<Vec<TopicPart>, TopicError> {
        s.split('/').map(|part| TopicPart::parse(part)).collect()
    }

    pub fn is_match(&self, s: &str) -> bool {
        for (index, part) in s.split('/').into_iter().enumerate() {
            if self.parts.len() - 1 < index {
                return false;
            }
            match self.parts[index] {
                TopicPart::Empty => return false,
                TopicPart::Normal(ref s_part) => {
                    if s_part != part {
                        return false;
                    }
                }
                TopicPart::Internal(ref s_part) => {
                    if s_part != part {
                        return false;
                    }
                }
                TopicPart::SingleWildcard => {
                    // Continue
                }
                TopicPart::MultiWildcard => return true,
            }
        }
        true
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn len(&self) -> usize {
        self.topic.len()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.topic.as_bytes()
    }
}

impl Topic {
    /// Validate topic filter.
    /// Rules are defined in `MQTT chapter-4.7 Topic Name and Filters`
    /// ```
    /// use codec::Topic;
    /// let name = "sport/tennis/player/#";
    /// assert!(Topic::validate_sub_topic(name).is_ok());
    ///
    /// let name = "sport/tennis/player#";
    /// assert!(Topic::validate_sub_topic(name).is_err());
    ///
    /// let name = "#";
    /// assert!(Topic::validate_sub_topic(name).is_ok());
    ///
    /// let name = "sport/#/player/ranking";
    /// assert!(Topic::validate_sub_topic(name).is_err());
    ///
    /// let name = "+";
    /// assert!(Topic::validate_sub_topic(name).is_ok());
    ///
    /// let name = "sport+";
    /// assert!(Topic::validate_sub_topic(name).is_err());
    /// ```
    pub fn validate_sub_topic(topic: &str) -> Result<(), TopicError> {
        if topic.is_empty() {
            return Err(TopicError::EmptyTopic);
        }
        if topic == "#" {
            return Ok(());
        }
        let bytes = topic.as_bytes();
        for (index, b) in bytes.iter().enumerate() {
            if b == &b'#' {
                // Must have a prefix level separator.
                if index > 0 && bytes[index - 1] != b'/' {
                    return Err(TopicError::InvalidChar);
                }

                // Must be the last wildcard.
                if index != bytes.len() - 1 {
                    return Err(TopicError::InvalidChar);
                }
            } else if b == &b'+' {
                // Must have a prefix level separator.
                if index > 0 && bytes[index - 1] != b'/' {
                    return Err(TopicError::InvalidChar);
                }
            }
        }

        Ok(())
    }

    /// Check whether topic name contains wildchard characters.
    /// ```
    /// use codec::Topic;
    /// let name = "sport/tennis/player/#";
    /// assert!(Topic::validate_pub_topic(name).is_err());
    ///
    /// let name = "sport/tennis/player/ranking";
    /// assert!(Topic::validate_pub_topic(name).is_ok());
    /// ```
    pub fn validate_pub_topic(topic: &str) -> Result<(), TopicError> {
        if topic.is_empty() {
            return Err(TopicError::EmptyTopic);
        }
        if topic.len() > u16::MAX as usize {
            return Err(TopicError::TooManyData);
        }

        if topic.as_bytes().iter().find(|c| c == &&b'+' || c == &&b'#') == None {
            Ok(())
        } else {
            Err(TopicError::InvalidChar)
        }
    }
}

// TODO(Shaohua): Impl internal reference to `topic` String.
#[derive(Debug, Clone)]
pub enum TopicPart {
    /// Special internal part, like `$SYS`.
    /// Topics start will `$` char will be traited as internal topic, even so
    /// only `$SYS` is used currently.
    Internal(String),

    /// Normal part.
    Normal(String),

    /// Empty part.
    Empty,

    /// `#` char, to match any remaining parts.
    MultiWildcard,

    /// `+` char, to match right part.
    SingleWildcard,
}

impl TopicPart {
    fn has_wildcard(s: &str) -> bool {
        s.contains(|c| c == '#' || c == '+')
    }

    fn is_internal(s: &str) -> bool {
        s.starts_with('$')
    }

    fn parse(s: &str) -> Result<Self, TopicError> {
        match s {
            "" => Ok(TopicPart::Empty),
            "+" => Ok(TopicPart::SingleWildcard),
            "#" => Ok(TopicPart::MultiWildcard),
            _ => {
                if TopicPart::has_wildcard(s) {
                    Err(TopicError::ContainsWildChar)
                } else if TopicPart::is_internal(s) {
                    Ok(TopicPart::Internal(s.to_string()))
                } else {
                    Ok(TopicPart::Normal(s.to_string()))
                }
            }
        }
    }
}

impl Default for TopicPart {
    fn default() -> Self {
        TopicPart::Empty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let t_sys = Topic::parse("$SYS/uptime");
        assert!(t_sys.is_ok());
    }

    #[test]
    fn test_topic_match() {
        let t_sys = Topic::parse("$SYS");
        assert!(t_sys.is_ok());
        let t_sys = t_sys.unwrap();

        let t_any = Topic::parse("#").unwrap();
        assert!(t_any.is_match(t_sys.str()));

        let t_dev = Topic::parse("dev/#").unwrap();
        assert!(t_dev.is_match("dev/cpu/0"));
    }
}