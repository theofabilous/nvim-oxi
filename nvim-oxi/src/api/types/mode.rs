use nvim_types::{Object, String as NvimString};
use serde::{Deserialize, Serialize};

use crate::object::{self, ToObject};

/// TODO: docs
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "c")]
    CmdLine,

    #[serde(rename = "i")]
    Insert,

    #[serde(rename = "!")]
    InsertCmdLine,

    #[serde(rename = "l")]
    Langmap,

    /// SEGFAULT: passing this to `Buffer::get_keymap` causes a segfault?
    #[serde(rename = "")]
    NormalVisualOperator,

    #[serde(rename = "n")]
    Normal,

    #[serde(rename = "o")]
    OperatorPending,

    #[serde(rename = "s")]
    Select,

    #[serde(rename = "t")]
    Terminal,

    #[serde(rename = "x")]
    Visual,

    #[serde(rename = "v")]
    VisualSelect,
}

macro_rules! is_mode {
    ($fn_name:ident, $variant:ident) => {
        #[inline(always)]
        pub fn $fn_name(&self) -> bool {
            *self == Mode::$variant
        }
    };
}

impl Mode {
    is_mode!(is_cmd_line, CmdLine);
    is_mode!(is_insert, Insert);
    is_mode!(is_langmap, Langmap);
    is_mode!(is_nvo, NormalVisualOperator);
    is_mode!(is_normal, Normal);
    is_mode!(is_op_pending, OperatorPending);
    is_mode!(is_select, Select);
    is_mode!(is_terminal, Terminal);
    is_mode!(is_visual, Visual);
    is_mode!(is_visual_select, VisualSelect);
}

impl ToObject for Mode {
    fn to_obj(self) -> crate::Result<Object> {
        self.serialize(object::Serializer)
    }
}

impl From<Mode> for NvimString {
    fn from(mode: Mode) -> Self {
        unsafe {
            NvimString::try_from(mode.to_obj().unwrap_unchecked())
                .unwrap_unchecked()
        }
    }
}