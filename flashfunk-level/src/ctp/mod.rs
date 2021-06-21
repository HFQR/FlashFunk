#![allow(
    dead_code,
    unused_variables,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_imports
)]

mod CtpMd;
mod CtpTd;
pub(crate) mod mdapi;
// fixme: 暂时开放用以快速生成tickdata
pub mod sys;
pub(crate) mod tdapi;
