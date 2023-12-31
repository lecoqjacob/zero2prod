use validator::validate_email;

#[derive(Debug, Clone, Default)]
pub struct SubscriberEmail(String);

impl TryFrom<&str> for SubscriberEmail {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if validate_email(value) {
            Ok(Self(value.into()))
        } else {
            Err(format!("{value} is not a valid subscriber email."))
        }
    }
}

impl TryFrom<String> for SubscriberEmail {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if validate_email(&value) {
            Ok(Self(value))
        } else {
            Err(format!("{value} is not a valid subscriber email."))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SubscriberEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;

    use claims::assert_err;
    use fake::{faker::internet::en::SafeEmail, Fake};
    use quickcheck::Arbitrary;

    #[test]
    fn empty_string_is_rejected() {
        let email = String::new();
        assert_err!(SubscriberEmail::try_from(email.as_str()));
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::try_from(email.as_str()));
    }
    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(SubscriberEmail::try_from(email.as_str()));
    }

    #[derive(Debug, Clone)]
    struct ValidEmail(String);

    impl Arbitrary for ValidEmail {
        fn arbitrary(_: &mut quickcheck::Gen) -> Self {
            Self(SafeEmail().fake())
        }

        // fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        //     Self(SafeEmail().fake_with_rng(g))
        // }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmail) -> bool {
        SubscriberEmail::try_from(valid_email.0.as_str()).is_ok()
    }
}
// c9a2fd1726b896e88e4b763a9ebbbd27-us21
