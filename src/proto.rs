// Include generated code from proto files
include!(concat!(env!("OUT_DIR"), "/proto/mod.rs"));

pub trait ToProto<T> {
    fn to_proto(&self) -> T;
}
