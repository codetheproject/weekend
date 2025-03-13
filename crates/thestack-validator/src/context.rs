use crate::Validator;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use thestack::new;

pub struct ValidatorContext<T, V> {
    validator: V,

    after_validator: Option<Box<dyn Fn(&T) -> Result<(), Error>>>,
    before_validator: Option<Box<dyn Fn(&T) -> Result<(), Error>>>,
}

impl<T, V> ValidatorContext<T, V>
where
    V: Validator<T>,
{
    pub fn new(validator: V) -> Self {
        ValidatorContext {
            validator,
            after_validator: None,
            before_validator: None,
        }
    }
    pub fn validate(&self, payload: &T) -> Result<(), Error> {
        if let Some(ref h) = self.before_validator {
            h(payload)?;
        }

        self.validator.validate(payload)?;

        self.after_validator
            .as_ref()
            .map_or(Ok(()), |h| h(payload))
    }

    // If any of the handler return error we return the error
    pub fn after_handler<F>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(&T) -> Result<(), Error> + 'static,
    {
        self.after_validator
            .replace(Box::new(handler));
        self
    }

    pub fn before_handler<F>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(&T) -> Result<(), Error> + 'static,
    {
        self.before_validator
            .replace(Box::new(handler));
        self
    }
}

#[derive(Debug, new, Serialize, Deserialize)]
// TODO -> Put serde beind a feature
pub struct Error {
    reason: Cow<'static, str>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<inner_error::InnerError>,
}

mod inner_error {
    use serde::{
        de::{self, Visitor},
        Deserialize, Deserializer, Serialize,
    };
    use std::{borrow::Cow, collections::HashMap, fmt};

    #[derive(Debug)]
    pub enum InnerError {
        Single(Cow<'static, str>),
        Multiple(HashMap<Cow<'static, str>, InnerError>),
    }

    impl Serialize for InnerError {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                InnerError::Single(cow) => cow.serialize(serializer),
                InnerError::Multiple(hash_map) => hash_map.serialize(serializer),
            }
        }
    }
    struct ErrorsVisitor;
    impl<'de> Visitor<'de> for ErrorsVisitor {
        type Value = InnerError;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or a map of errors")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(InnerError::Single(Cow::Owned(value.to_string())))
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            let mut result_map = HashMap::new();
            while let Some((key, value)) = map.next_entry()? {
                result_map.insert(key, value);
            }
            Ok(InnerError::Multiple(result_map))
        }
    }

    impl<'de> Deserialize<'de> for InnerError {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(ErrorsVisitor)
        }
    }
}
