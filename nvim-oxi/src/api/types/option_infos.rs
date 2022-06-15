use std::fmt;

use nvim_types::Object;
use serde::{de, Deserialize};

use crate::object::{self, FromObject};

/// Returned by `crate::api::get_all_options_info`. Unlike in the Lua api the
/// `type` field is omitted because it's included in the definition of
/// `default`.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct OptionInfos {
    ///
    pub allows_duplicates: bool,

    /// List of comma-separated values.
    pub commalist: bool,

    /// The default value for the option.
    pub default: OptionDefault,

    pub flaglist: bool,

    /// Whether a window or buffer option also has a global value.
    pub global_local: bool,

    /// Channel id where the option was set (`0` for local).
    pub last_set_chan: u32,

    /// The line number where the option was set.
    pub last_set_linenr: usize,

    /// Last set script id (if any).
    pub last_set_sid: u32,

    /// Name of the option (like `"filetype"`).
    pub name: String,

    /// Scope of the option.
    pub scope: OptionScope,

    /// Shortened name of the  option (like `"ft"`).
    pub shortname: String,

    /// Whether the option was set.
    pub was_set: bool,
}

impl FromObject for OptionInfos {
    fn from_obj(obj: Object) -> crate::Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub enum OptionScope {
    #[serde(rename = "buf")]
    Buffer,

    #[serde(rename = "global")]
    Global,

    #[serde(rename = "win")]
    Window,
}

impl OptionScope {
    #[inline]
    pub const fn is_buffer(&self) -> bool {
        matches!(self, OptionScope::Buffer)
    }

    #[inline]
    pub const fn is_global(&self) -> bool {
        matches!(self, OptionScope::Global)
    }

    #[inline]
    pub const fn is_window(&self) -> bool {
        matches!(self, OptionScope::Window)
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum OptionDefault {
    Boolean(bool),
    Number(i64),
    String(String),
}

impl OptionDefault {
    #[inline]
    pub const fn as_boolean(&self) -> Option<bool> {
        match self {
            OptionDefault::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    #[inline]
    pub const fn as_number(&self) -> Option<i64> {
        match self {
            OptionDefault::Number(n) => Some(*n),
            _ => None,
        }
    }

    #[inline]
    pub fn as_string(&self) -> Option<&str> {
        match &self {
            OptionDefault::String(s) => Some(s),
            _ => None,
        }
    }

    #[inline]
    pub const fn is_boolean(&self) -> bool {
        matches!(self, OptionDefault::Boolean(_))
    }

    #[inline]
    pub const fn is_number(&self) -> bool {
        matches!(self, OptionDefault::Number(_))
    }

    #[inline]
    pub const fn is_string(&self) -> bool {
        matches!(self, OptionDefault::String(_))
    }
}

impl<'de> de::Deserialize<'de> for OptionDefault {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct OptionDefaultVisitor;

        impl<'de> de::Visitor<'de> for OptionDefaultVisitor {
            type Value = OptionDefault;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a boolean, a number or a string")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Self::Value::Boolean(v))
            }

            fn visit_i64<E>(self, n: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Self::Value::Number(n))
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Self::Value::String(s.to_owned()))
            }
        }

        deserializer.deserialize_str(OptionDefaultVisitor)
    }
}