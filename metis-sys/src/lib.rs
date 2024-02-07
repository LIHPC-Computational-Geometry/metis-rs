#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(not(feature = "vendored"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "vendored")]
include!("../gen/bindings.rs");
