use types::Type;
use types::Value;
use util::StringConversion;
use types::CurriedFunc;
use types::Fun;

pub mod environment;
pub mod type_analyzer;
pub mod expression_fold;
pub mod pattern_helper;
pub mod type_resolution;