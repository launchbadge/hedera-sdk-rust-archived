#![allow(renamed_and_removed_lints, clippy::all, clippy::pedantic, unreachable_pub)]

// Include generated code from proto files
include!(concat!(env!("OUT_DIR"), "/proto/mod.rs"));

use failure::Error;

pub trait ToProto<T> {
    fn to_proto(&self) -> Result<T, Error>;
}
