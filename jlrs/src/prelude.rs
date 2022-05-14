//! Reexports structs and traits you're likely to need.

#[cfg(feature = "pyplot")]
pub use crate::pyplot::{AccessPlotsModule, PyPlot};
#[cfg(feature = "async-std-rt")]
pub use crate::runtime::async_rt::async_std_rt::*;
#[cfg(feature = "tokio-rt")]
pub use crate::runtime::async_rt::tokio_rt::*;
#[cfg(any(feature = "async-rt", feature = "sync-rt"))]
pub use crate::runtime::builder::RuntimeBuilder;
#[cfg(feature = "sync-rt")]
pub use crate::runtime::sync_rt::Julia;
#[cfg(feature = "async-rt")]
pub use crate::runtime::{async_rt::AsyncJulia, builder::AsyncRuntimeBuilder};
#[cfg(feature = "async")]
pub use crate::{
    async_util::task::{yield_task, AsyncTask, PersistentTask},
    call::CallAsync,
    memory::frame::AsyncGcFrame,
};
pub use crate::{
    call::{Call, CallExt},
    convert::into_jlrs_result::IntoJlrsResult,
    error::JlrsResult,
    memory::{
        frame::{Frame, GcFrame},
        global::Global,
        scope::{PartialScope, Scope},
    },
    named_tuple,
    wrappers::{
        inline::{bool::Bool, char::Char, nothing::Nothing, tuple::*},
        ptr::{
            array::{dimensions::Dims, Array, TypedArray},
            datatype::DataType,
            module::Module,
            string::JuliaString,
            symbol::Symbol,
            value::Value,
            ArrayRef, DataTypeRef, ModuleRef, Ref, StringRef, TypedArrayRef, ValueRef, Wrapper,
        },
    },
};
#[cfg(feature = "ccall")]
pub use crate::{ccall::CCall, memory::frame::NullFrame};
#[cfg(feature = "async")]
pub use async_trait::async_trait;
#[cfg(feature = "jlrs-derive")]
pub use jlrs_derive::*;
