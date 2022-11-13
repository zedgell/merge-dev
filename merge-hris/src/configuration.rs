use merge_config::configuration::Configuration;

pub type HRISConfig = Configuration;

#[cfg(test)]
mod tests {
    use std::fmt::{Display, Formatter};
    use super::*;

    #[test]
    fn it_works_with_str() {
        let config = HRISConfig::new("someKey", "someToken");
        assert_eq!(config.api_key, "someKey");
        assert_eq!(config.access_token, "someToken")
    }

    #[test]
    fn it_works_with_string() {
        let config = HRISConfig::new("someKey".to_string(), "someToken".to_string());
        assert_eq!(config.api_key, "someKey");
        assert_eq!(config.access_token, "someToken")
    }

    #[test]
    fn it_works_with_custom_structs() {
        struct AccessKey {
            key: String
        }
        impl Display for AccessKey {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.key)
            }
        }
        struct AccessToken {
            key: String
        }
        impl Display for AccessToken {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.key)
            }
        }
        let access_key = AccessKey {
            key: "someKey".to_string()
        };
        let access_token = AccessToken {
            key: "someToken".to_string()
        };
        let config = HRISConfig::new(access_key, access_token);
        assert_eq!(config.api_key, "someKey");
        assert_eq!(config.access_token, "someToken")
    }
}