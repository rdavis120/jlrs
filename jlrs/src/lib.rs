//! jlrs provides a reasonably safe interface to the Julia C API that lets you call code written
//! in Julia from Rust and vice versa. Currently this crate is only tested on Linux and Windows in
//! combination with Julia 1.6 and is not compatible with earlier versions of Julia.
//!
//! # Features
//!
//! An incomplete list of features that are currently supported by jlrs:
//!
//!  - Access arbitrary Julia modules and their contents.
//!  - Call arbitrary Julia functions, including functions that take keyword arguments.
//!  - Include and call your own Julia code.
//!  - Load a custom system image.
//!  - Create values that Julia can use, and convert them back to Rust, from Rust.
//!  - Access the type information and fields of values and check their properties.
//!  - Create and use n-dimensional arrays.
//!  - Support for mapping Julia structs to Rust structs which can be generated with `JlrsReflect.jl`.
//!  - Structs that can be mapped to Rust include those with type parameters and bits unions.
//!  - Use these features when calling Rust from Julia through `ccall`.
//!  - Offload long-running functions to another thread and `.await` the result with the (experimental) async runtime.
//!
//!
//! # Generating the bindings
//!
//! This crate depends on `jl-sys` which contains the raw bindings to the Julia C API, these are
//! generated by `bindgen`. You can find the requirements for using `bindgen` in [their User Guide].
//!
//! #### Linux
//!
//! The recommended way to install Julia is to download the binaries from the official website,
//! which is distributed in an archive containing a directory called `julia-x.y.z`. This directory
//! contains several other directories, including a `bin` directory containing the `julia`
//! executable.
//!
//! In order to ensure the `julia.h` header file can be found, either `/usr/include/julia/julia.h`
//! must exist, or you have to set the `JULIA_DIR` environment variable to `/path/to/julia-x.y.z`.
//! This environment variable can be used to override the default. Similarly, in order to load
//! `libjulia.so` you must add `/path/to/julia-x.y.z/lib` to the `LD_LIBRARY_PATH` environment
//! variable.
//!
//! #### Windows
//!
//! The recommended way to install Julia is to download the installer from the official website,
//! which will install Julia in a folder called `Julia-x.y.z`. This folder contains several other
//! folders, including a `bin` folder containing the `julia.exe` executable. You must set the
//! `JULIA_DIR` environment variable to the `Julia-x.y.z` folder and add `Julia-x.y.z\bin` to the
//! `PATH` environment variable. For example, if Julia is installed at `D:\Julia-x.y.z`,
//! `JULIA_DIR` must be set to `D:\Julia-x.y.z` and `D:\Julia-x.y.z\bin` must be added to `PATH`.
//!
//! Additionally, MinGW must be installed through Cygwin. To install this and all potentially
//! required dependencies, follow steps 1-4 of
//! [the instructions for compiling Julia on Windows using Cygwin and MinGW].
//! You must set the `CYGWIN_DIR` environment variable to the installation folder of Cygwin; this
//! folder contains some icons, `Cygwin.bat` and folders with names like `usr` and `bin`. For
//! example, if Cygwin is installed at `D:\cygwin64`, `CYGWIN_DIR` must be set to `D:\cygwin64`.
//!
//! Julia is compatible with the GNU toolchain on Windows. If you use rustup, you can set the
//! toolchain for a project that depends on `jl-sys` by calling the command
//! `rustup override set stable-gnu` in the project root folder.
//!
//!
//! # Using this crate
//!
//! The first thing you should do is `use` the [`prelude`]-module with an asterisk, this will
//! bring all the structs and traits you're likely to need into scope. If you're calling Julia
//! from Rust, you must initialize Julia before you can use it. You can do this by calling
//! [`Julia::init`]. Note that this method can only be called once, if you drop [`Julia`] you won't
//! be able to create a new one and have to restart the entire program. If you want to use a
//! custom system image, you must call [`Julia::init_with_image`] instead of [`Julia::init`].
//! If you're calling Rust from Julia everything has already been initialized, you can use `CCall`
//! instead.
//!
//!
//! ## Calling Julia from Rust
//!
//! After initialization you have your [`Julia`] instance, [`Julia::include`] can be used to
//! include files with custom Julia code. In order to call Julia functions and create new values
//! that can be used by these functions, [`Julia::scope`] and [`Julia::scope_with_slots`] must be
//! used. These two methods take a closure with two arguments, a [`Global`] and a mutable reference
//! to a [`GcFrame`], which is called inside the `scope(_with_slots)`. [`Global`] is a token that
//! lets you access Julia modules, their contents and other global values, while the frame is used
//! to root local values. Rooting a value in a frame prevents it from being freed by the garbage
//! collector until that frame has been dropped.
//!
//! Julia data is represented as a [`Value`]. There are several ways to create a new `Value`. The
//! simplest is to call [`Value::eval_string`], which takes two arguments. Something that
//! implements the [`Scope`] trait and a string. The string has to be valid Julia, mutable
//! references to frames implement [`Scope`]. When a frame is used as a scope, the [`Value`] that
//! is returned is rooted in that frame, and can be used until that scope ends. This method is
//! relatively limited, it can be used to evaluate simple function call like `sqrt(2.0)`, but it's
//! major use is importing installed packages by evaluating an `import` or `using` statement. To
//! create a `Value` directly, other methods like [`Value::new`] are available. [`Value::new`]
//! supports converting primitive types from Rust to Julia, but can also be used with some more
//! complex types like `String`s.
//!
//! Julia functions are `Value`s too. In fact, all `Value`s can be called as functions, whether
//! this will succeed depends on the value actually being a function that is implemented for the
//! arguments it's called with. In order to call some `Value` as a Julia function three things are
//! needed: the function you want to call, something that implements [`Scope`] to root the result,
//! and possibly some arguments to call the function with. The function can be acquired through
//! the module that defines it with [`Module::function`]; [`Module::base`] and [`Module::core`]
//! provide access to Julia's `Base` and `Core` module respectively, while everything you include
//! through [`Julia::include`] is made available relative to the `Main` module, which can be
//! access by calling [`Module::main`].
//!
//! Because a `Value` must only be used while it's rooted, it's not possible to return one from
//! the closure due to lifetime constraints on the output. In order to convert a `Value` to
//! another type, like `u8` or `String`, [`Value::cast`] must be used. This method checks if the
//! value can be converted to the type it's cast to and performs the conversion if it is, which
//! generally amounts to a pointer dereference, but for builtin types like [`DataType`] and
//! [`Array`] it's a  pointer cast. These builtin types are subject to the same lifetime
//! constraints as the `Value` is and can't be returned from the closure, but much of their
//! functionality is exposed as methods on their more specific type. For example, in order to
//! access the data in an `Array` from Rust, the `Value` must be cast to an `Array` first.
//!
//! As a simple example, let's create two values and add them:
//!
//! ```no_run
//! # use jlrs::prelude::*;
//! # use jlrs::util::JULIA;
//! # fn main() {
//! let mut julia = unsafe { Julia::init().unwrap() };
//! julia.scope(|global, frame| {
//!     // Create the two arguments. Note that the first argument, something that
//!     // implements Scope, is taken by value and mutable references don't implement
//!     // Copy, so it's necessary to mutably reborrow the frame.
//!     let i = Value::new(&mut *frame, 2u64)?;
//!     let j = Value::new(&mut *frame, 1u32)?;
//!
//!     // The `+` function can be found in the base module.
//!     let func = Module::base(global).function("+")?;
//!
//!     // Call the function and cast the result to `u64`. The result is a nested
//!     // `Result`; the outer error does not contain to any Julia data, while the
//!     // inner error contains the exception if one is thrown.
//!     func.call2(&mut *frame, i, j)?
//!         .into_jlrs_result()?
//!         .cast::<u64>()
//! }).unwrap();
//! # }
//! ```
//!
//! Scopes can be nested, this is especially useful when you need to create several temporary
//! values to create a new `Value` or call a Julia function, because the each scope has its own
//! `GcFrame`. This means these temporary values will not be protected from garbage collection
//! after returning from the closure. There are three methods that create a nested scope,
//! [`ScopeExt::scope`], [`Scope::value_scope`] and [`Scope::result_scope`]. The first is very
//! similar to the previous example and has the same major limitation: its return value can be
//! anything, as long as its guaranteed to live at least as long as the parent scope. This means
//! you can't create a `Value` or call a Julia function and return its result to the parent scope
//! with this method. The other two methods support those use-cases.
//!
//! Another implementation of [`Scope`] appears here: the closure that `value_scope` and
//! `result_scope` take has two arguments, an `Output` and a mutable reference to a `GcFrame`. The
//! frame can be used to create temporary values. The output must be converted to an
//! [`OutputScope`] before creating the value that must be rooted in an earlier frame.
//!
//! For example, the two values from the previous example can be treated as temporary values rooted
//! rooted in some `child_frame, while their sum is rooted in the `parent_frame`:
//!
//! ```
//! # use jlrs::prelude::*;
//! # use jlrs::util::JULIA;
//! # fn main() {
//! # JULIA.with(|j| {
//! # let mut julia = j.borrow_mut();
//!   julia.scope(|global, parent_frame| {
//!       let sum_value = parent_frame.result_scope(|output, child_frame| {
//!           // i and j are rooted in child_frame...
//!           let i = Value::new(&mut *child_frame, 1u64)?;
//!           let j = Value::new(&mut *child_frame, 2i32)?;
//!           let func = Module::base(global).function("+")?;
//!
//!           // ... while the result is rooted in the parent frame
//!           // after returning from this closure.
//!           let output_scope = output.into_scope(child_frame);
//!           func.call2(output_scope, i, j)
//!       })?.into_jlrs_result()?;
//!
//!       assert_eq!(sum_value.cast::<u64>()?, 3);
//!
//!       Ok(())
//!   }).unwrap();
//! # });
//! # }
//! ```
//!
//! This is only a small example, other things can be done with [`Value`] as well: their fields
//! and type information can be accessed for example, and keywords can be provided to call
//! functions with keyword arguments.
//!
//!
//! ## Calling Rust from Julia
//!
//! Julia's `ccall` interface can be used to call `extern "C"` functions defined in Rust. There
//! are two major ways to use `ccall`, with a pointer to the function or a
//! `(:function, "library")` pair.
//!
//! A function can be cast to a void pointer and converted to a [`Value`]:
//!
//! ```no_run
//! # use jlrs::prelude::*;
//! // This function will be provided to Julia as a pointer, so its name can be mangled.
//! unsafe extern "C" fn call_me(arg: bool) -> isize {
//!     if arg {
//!         1
//!     } else {
//!         -1
//!     }
//! }
//!
//! # fn main() {
//! let mut julia = unsafe { Julia::init().unwrap() };
//! julia.scope(|global, frame| {
//!     // Cast the function to a void pointer
//!     let call_me_val = Value::new(&mut *frame, call_me as *mut std::ffi::c_void)?;
//!
//!     // Value::eval_string can be used to create new functions.
//!     let func = Value::eval_string(
//!         &mut *frame,
//!         "myfunc(callme::Ptr{Cvoid})::Int = ccall(callme, Int, (Bool,), true)"
//!     )?.unwrap();
//!
//!     // Call the function and unbox the result.  
//!     let output = func.call1(&mut *frame, call_me_val)?
//!         .into_jlrs_result()?
//!         .cast::<isize>()?;
//!
//!     assert_eq!(output, 1);
//!     
//!     Ok(())
//! }).unwrap();
//! # }
//! ```
//!
//! You can also use functions defined in `dylib` and `cdylib` libraries. In order to create such
//! a library you need to add
//!
//! ```toml
//! [lib]
//! crate-type = ["dylib"]
//! ```
//!
//! or  
//!
//! ```toml
//! [lib]
//! crate-type = ["cdylib"]
//! ```
//!
//! respectively to your crate's `Cargo.toml`. Use a `dylib` if you want to use the crate in other
//! Rust crates, but if it's only intended to be called through `ccall` a `cdylib` is the better
//! choice. On Linux, compiling such a crate will be compiled to `lib<crate_name>.so`, on Windows
//! `lib<crate_name>.dll`.
//!
//! The functions you want to use with `ccall` must be both `extern "C"` functions to ensure the C
//! ABI is used, and annotated with `#[no_mangle]` to prevent name mangling. Julia can find
//! libraries in directories that are either on the default library search path or included by
//! setting the `LD_LIBRARY_PATH` environment variable on Linux, or `PATH` on Windows. If the
//! compiled library is not directly visible to Julia, you can open it with `Libdl.dlopen` and
//! acquire function pointers with `Libdl.dlsym`. These pointers can be called the same way as
//! the pointer in the previous example.
//!
//! If the library is visible to Julia you can access it with the library name. If `call_me` is
//! defined in a crate called `foo`, the following should workif the function is annotated with
//! `#[no_mangle]`:
//!
//! ```julia
//! ccall((:call_me, "libfoo"), Int, (Bool,), false)
//! ```
//!
//! One important aspect of calling Rust from other languages in general is that panicking across
//! an FFI boundary is undefined behaviour. If you're not sure your code will never panic, wrap it
//! with `std::panic::catch_unwind`.
//!
//! Most features provided by jlrs including accessing modules, calling functions, and borrowing
//! array data require a [`Global`] or a frame. You can access these by creating a [`CCall`]
//! first.
//!
//!
//! ## Async runtime
//!
//! The experimental async runtime runs Julia in a separate thread and allows multiple tasks to
//! run in parallel by offloading functions to a new thread in Julia and waiting for them to
//! complete without blocking the runtime. To use this feature you must enable the `async` feature
//! flag:
//!
//! ```toml
//! [dependencies]
//! jlrs = { version = "0.10", features = ["async"] }
//! ```
//!
//! This features is only supported on Linux.
//!
//! The struct [`AsyncJulia`] is exported by the prelude and lets you initialize the runtime in
//! two ways, either as a task or as a thread. The first type should be used if you want to
//! integrate the async runtime into a larger project that uses `async_std`. In order for the
//! runtime to work correctly the `JULIA_NUM_THREADS` environment variable must be set to a value
//! larger than 1.
//!
//! In order to call Julia with the async runtime you must implement the [`JuliaTask`] trait. The
//! `run`-method of this trait is similar to the closures that are used in the examples
//! above for the sync runtime; it provides you with a [`Global`] and an [`AsyncGcFrame`] which
//! implements the [`Frame`] trait. The [`AsyncGcFrame`] is required to call [`Value::call_async`]
//! which calls a Julia function on another thread by using `Base.Threads.@spawn` and returns a
//! `Future`. While awaiting the result the runtime can handle another task. If you don't use
//! [`Value::call_async`] tasks are executed sequentially.
//!
//! It's important to keep in mind that allocating memory in Julia uses a lock, so if you execute
//! multiple functions at the same time that allocate new values frequently the performance will
//! drop significantly. The garbage collector can only run when all threads have reached a
//! safepoint, which is the case whenever a function needs to allocate memory. If your function
//! takes a long time to complete but needs to allocate rarely, you should periodically call
//! `GC.safepoint` in Julia to ensure the garbage collector can run.
//!
//! You can find fully commented basic examples in [the examples directory of the repo].
//!
//!
//! # Testing
//!
//! The restriction that Julia can be initialized must be taken into account when running tests
//! that use `jlrs`. The recommended approach is to create a thread-local static `RefCell`:
//!
//! ```no_run
//! use jlrs::prelude::*;
//! use std::cell::RefCell;
//! thread_local! {
//!     pub static JULIA: RefCell<Julia> = {
//!         let julia = RefCell::new(unsafe { Julia::init().unwrap() });
//!         julia.borrow_mut().scope(|_global, _frame| {
//!             /* include everything you need to use */
//!             Ok(())
//!         }).unwrap();
//!         julia
//!     };
//! }
//! ```
//!
//! Tests that use this construct can only use one thread for testing, so you must use
//! `cargo test -- --test-threads=1`, otherwise the code above will panic when a test
//! tries to call `Julia::init` a second time from another thread.
//!
//! If these tests also involve the async runtime, the `JULIA_NUM_THREADS` environment
//! variable must be set to a value larger than 1.
//!
//! If you want to run jlrs's tests, both these requirements must be taken into account:
//! `JULIA_NUM_THREADS=2 cargo test -- --test-threads=1`
//!
//!
//! # Custom types
//!
//! In order to map a struct in Rust to one in Julia you can derive [`JuliaStruct`]. This will
//! implement [`Cast`], [`JuliaType`], [`ValidLayout`], and [`JuliaTypecheck`] for that type. If
//! the struct in Julia has no type parameters and is a bits type you can also derive
//! [`IntoJulia`], which lets you use the type in combination with [`Value::new`].
//!
//! You should not implement these structs manually. The `JlrsReflect.jl` package can generate
//! the correct Rust struct for types that have no tuple or union fields with type parameters.
//! The reason for this restriction is that the layout of tuple and union fields can be very
//! different depending on these parameters in a way that can't be nicely expressed in Rust.
//!
//! These custom types can also be used when you call Rust from Julia through `ccall`.
//!
//! [their User Guide]: https://rust-lang.github.io/rust-bindgen/requirements.html
//! [the instructions for compiling Julia on Windows using Cygwin and MinGW]: https://github.com/JuliaLang/julia/blob/v1.5.2/doc/build/windows.md#cygwin-to-mingw-cross-compiling
//! [the examples directory of the repo]: https://github.com/Taaitaaiger/jlrs/tree/v0.8/examples
//! [`IntoJulia`]: ./convert/into_julia/traits.IntoJulia.html
//! [`JuliaType`]: ./layout/julia_type/traits.JuliaType.html
//! [`JuliaTypecheck`]: ./layout/julia_typecheck/traits.JuliaTypecheck.html
//! [`ValidLayout`]: ./layout/valid_layout/traits.ValidLayout.html
//! [`Cast`]: ./convert/cast/traits.Cast.html
//! [`JuliaStruct`]: ./value/traits/julia_struct/traits.JuliaStruct.html
//! [`AsyncGcFrame`]: ./memory/frame/struct.AsyncGcFrame.html
//! [`Frame`]: ./memory/traits/frame/trait.Frame.html
//! [`JuliaTask`]: ./multitask/julia_task/trait.JuliaTask.html
//! [`AsyncJulia`]: ./multitask/struct.AsyncJulia.html
//! [`DataType`]: ./value/datatype/struct.DataType.html
//! [`TypedArray`]: ./value/array/struct.TypedArray.html
//! [`OutputScope`]: ./memory/output/struct.OutputScope.html
//! [`Scope`]: ./memory/traits/scope/struct.Scope.html

pub mod convert;
pub mod error;
#[doc(hidden)]
pub mod jl_sys_export;
pub mod layout;
pub mod memory;
#[cfg(all(feature = "async", target_os = "linux"))]
pub mod multitask;
pub mod prelude;
pub(crate) mod private;
#[doc(hidden)]
pub mod util;
pub mod value;

use error::{JlrsError, JlrsResult};
use jl_sys::{
    jl_atexit_hook, jl_init, jl_init_with_image__threading, jl_is_initialized, uv_async_send,
};
use memory::frame::{GcFrame, NullFrame};
use memory::global::Global;
use memory::mode::Sync;
use memory::stack::StackPage;
use prelude::IntoJlrsResult;
use std::ffi::{c_void, CString};
use std::io::{Error as IOError, ErrorKind};
use std::mem::MaybeUninit;
use std::path::Path;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering};
use value::array::Array;
use value::module::Module;
use value::traits::call::Call;
use value::Value;

pub(crate) static INIT: AtomicBool = AtomicBool::new(false);

pub(crate) static JLRS_JL: &'static str = include_str!("jlrs.jl");

/// This struct can be created only once during the lifetime of your program. You must create it
/// with [`Julia::init`] or [`Julia::init_with_image`] before you can do anything related to
/// Julia. While this struct exists Julia is active, dropping it causes the shutdown code to be
/// called but this doesn't leave Julia in a state from which it can be reinitialized.
pub struct Julia {
    page: StackPage,
}

impl Julia {
    /// Initialize Julia, this method can only be called once. If it's called a second time it
    /// will return an error. If this struct is dropped, you will need to restart your program to
    /// be able to call Julia code again.
    ///
    /// This method is unsafe because this crate provides you with a way to execute arbitrary
    /// Julia code which can't be checked for correctness.
    pub unsafe fn init() -> JlrsResult<Self> {
        if jl_is_initialized() != 0 || INIT.swap(true, Ordering::SeqCst) {
            return Err(JlrsError::AlreadyInitialized.into());
        }

        jl_init();
        let mut jl = Julia {
            page: StackPage::default(),
        };

        jl.scope_with_slots(2, |global, frame| {
            Value::eval_string(frame, JLRS_JL)?.into_jlrs_result()?;

            let droparray_fn = Value::new(frame, droparray as *mut c_void)?;
            Module::main(global)
                .submodule("Jlrs")?
                .global("droparray")?
                .set_nth_field(0, droparray_fn)?;

            Ok(())
        })
        .expect("Could not load Jlrs module");

        Ok(jl)
    }

    /// This method is similar to [`Julia::init`] except that it loads a custom system image. A
    /// custom image can be generated with the [`PackageCompiler`] package for Julia. The main
    /// advantage of using a custom image over the default one is that it allows you to avoid much
    /// of the compilation overhead often associated with Julia.
    ///
    /// Two arguments are required to call this method compared to [`Julia::init`];
    /// `julia_bindir` and `image_relative_path`. The first must be the absolute path to a
    /// directory that contains a compatible Julia binary (eg `${JULIA_DIR}/bin`), the second must
    /// be either an absolute or a relative path to a system image.
    ///
    /// This method will return an error if either of the two paths does not exist or if Julia
    /// has already been initialized. It is unsafe because this crate provides you with a way to
    /// execute arbitrary Julia code which can't be checked for correctness.
    ///
    /// [`PackageCompiler`]: https://julialang.github.io/PackageCompiler.jl/dev/
    pub unsafe fn init_with_image<P: AsRef<Path>, Q: AsRef<Path>>(
        julia_bindir: P,
        image_path: Q,
    ) -> JlrsResult<Self> {
        if INIT.swap(true, Ordering::SeqCst) {
            Err(JlrsError::AlreadyInitialized)?;
        }

        let julia_bindir_str = julia_bindir.as_ref().to_string_lossy().to_string();
        let image_path_str = image_path.as_ref().to_string_lossy().to_string();

        if !julia_bindir.as_ref().exists() {
            let io_err = IOError::new(ErrorKind::NotFound, julia_bindir_str);
            return Err(JlrsError::other(io_err))?;
        }

        if !image_path.as_ref().exists() {
            let io_err = IOError::new(ErrorKind::NotFound, image_path_str);
            return Err(JlrsError::other(io_err))?;
        }

        let bindir = CString::new(julia_bindir_str).unwrap();
        let im_rel_path = CString::new(image_path_str).unwrap();

        jl_init_with_image__threading(bindir.as_ptr(), im_rel_path.as_ptr());

        let mut jl = Julia {
            page: StackPage::default(),
        };

        jl.scope_with_slots(2, |global, frame| {
            Value::eval_string(frame, JLRS_JL)?.into_jlrs_result()?;

            let droparray_fn = Value::new(frame, droparray as *mut c_void)?;
            Module::main(global)
                .submodule("Jlrs")?
                .global("droparray")?
                .set_nth_field(0, droparray_fn)?;

            Ok(())
        })
        .expect("Could not load Jlrs module");

        Ok(jl)
    }

    /// Calls `include` in the `Main` module in Julia, which executes the file's contents in that
    /// module. This has the same effect as calling `include` in the Julia REPL.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use jlrs::prelude::*;
    /// # fn main() {
    /// # let mut julia = unsafe { Julia::init().unwrap() };
    /// julia.include("Path/To/MyJuliaCode.jl").unwrap();
    /// # }
    /// ```
    pub fn include<P: AsRef<Path>>(&mut self, path: P) -> JlrsResult<()> {
        if path.as_ref().exists() {
            return self.scope_with_slots(2, |global, frame| {
                let path_jl_str = Value::new(&mut *frame, path.as_ref().to_string_lossy())?;
                let include_func = Module::main(global).function("include")?;
                let res = include_func.call1(frame, path_jl_str)?;

                return match res {
                    Ok(_) => Ok(()),
                    Err(e) => Err(JlrsError::IncludeError(
                        path.as_ref().to_string_lossy().into(),
                        e.type_name().into(),
                    )
                    .into()),
                };
            });
        }

        Err(JlrsError::IncludeNotFound(path.as_ref().to_string_lossy().into()).into())
    }

    /// This method is a main entrypoint to interact with Julia. It takes a closure with two
    /// arguments, a `Global` and a mutable reference to a `GcFrame`, and can return arbitrary
    /// results.
    ///
    /// Example:
    ///
    /// ```
    /// # use jlrs::prelude::*;
    /// # use jlrs::util::JULIA;
    /// # fn main() {
    /// # JULIA.with(|j| {
    /// # let mut julia = j.borrow_mut();
    ///   julia.scope(|_global, frame| {
    ///       let _i = Value::new(&mut *frame, 1u64)?;
    ///       Ok(())
    ///   }).unwrap();
    /// # });
    /// # }
    /// ```
    pub fn scope<T, F>(&mut self, func: F) -> JlrsResult<T>
    where
        for<'base> F: FnOnce(Global<'base>, &mut GcFrame<'base, Sync>) -> JlrsResult<T>,
    {
        unsafe {
            let global = Global::new();
            let mut frame = GcFrame::new(self.page.as_mut(), 0, Sync);
            func(global, &mut frame)
        }
    }

    /// This method is a main entrypoint to interact with Julia. It takes a closure with two
    /// arguments, a `Global` and a mutable reference to a `GcFrame`, and can return arbitrary
    /// results. The frame will preallocate `slots` slots.
    ///
    /// Example:
    ///
    /// ```
    /// # use jlrs::prelude::*;
    /// # use jlrs::util::JULIA;
    /// # fn main() {
    /// # JULIA.with(|j| {
    /// # let mut julia = j.borrow_mut();
    ///   julia.scope_with_slots(1, |_global, frame| {
    ///       // Uses the preallocated slot
    ///       let _i = Value::new(&mut *frame, 1u64)?;
    ///       // Allocates a new slot, because only a single slot was preallocated
    ///       let _j = Value::new(&mut *frame, 1u64)?;
    ///       Ok(())
    ///   }).unwrap();
    /// # });
    /// # }
    /// ```
    pub fn scope_with_slots<T, F>(&mut self, slots: usize, func: F) -> JlrsResult<T>
    where
        for<'base> F: FnOnce(Global<'base>, &mut GcFrame<'base, Sync>) -> JlrsResult<T>,
    {
        unsafe {
            let global = Global::new();
            if slots + 2 > self.page.size() {
                self.page = StackPage::new(slots + 2);
            }
            let mut frame = GcFrame::new(self.page.as_mut(), slots, Sync);
            func(global, &mut frame)
        }
    }
}

impl Drop for Julia {
    fn drop(&mut self) {
        unsafe {
            jl_atexit_hook(0);
        }
    }
}

/// When you call Rust from Julia through `ccall`, Julia has already been initialized and trying to
/// initialize it again would cause a crash. In order to still be able to call Julia from Rust
/// and to borrow arrays (if you pass them as `Array` rather than `Ptr{Array}`), you'll need to
/// create a frame first. You can use this struct to do so. It must never be used outside
/// functions called through `ccall`, and only once for each `ccall`ed function.
///
/// If you only need to use a frame to borrow array data, you can use [`CCall::null_frame`].
/// Unlike [`Julia`], `CCall` postpones the allocation of the stack that is used for managing the
/// GC until a `GcFrame` is created. In the case of a null frame, this stack isn't allocated at
/// all.
pub struct CCall {
    page: Option<StackPage>,
}

impl CCall {
    /// Create a new `CCall`. This function must never be called outside a function called through
    /// `ccall` from Julia and must only be called once during that call. The stack is not
    /// allocated until a [`GcFrame`] is created.
    pub unsafe fn new() -> Self {
        CCall { page: None }
    }

    /// Wake the task associated with `handle`. The handle must be the `handle` field of a
    /// `Base.AsyncCondition` in Julia. This can be used to call a long-running Rust function from
    /// Julia with ccall in another thread and wait for it to complete in Julia without blocking,
    /// there's an example available in the repository: ccall_with_threads.
    pub unsafe fn uv_async_send(handle: *mut c_void) -> bool {
        uv_async_send(handle.cast()) == 0
    }

    /// Creates a [`GcFrame`], calls the given closure, and returns its result.
    pub fn scope<T, F>(&mut self, func: F) -> JlrsResult<T>
    where
        for<'base> F: FnOnce(Global<'base>, &mut GcFrame<'base, Sync>) -> JlrsResult<T>,
    {
        unsafe {
            let page = self.get_init_page();
            let global = Global::new();
            let mut frame = GcFrame::new(page.as_mut(), 0, Sync);
            func(global, &mut frame)
        }
    }

    /// Creates a [`GcFrame`] with `slots` slots, calls the given closure, and returns its result.
    pub fn scope_with_slots<T, F>(&mut self, slots: usize, func: F) -> JlrsResult<T>
    where
        for<'base> F: FnOnce(Global<'base>, &mut GcFrame<'base, Sync>) -> JlrsResult<T>,
    {
        unsafe {
            let page = self.get_init_page();
            let global = Global::new();
            if slots + 2 > page.size() {
                *page = StackPage::new(slots + 2);
            }
            let mut frame = GcFrame::new(page.as_mut(), slots, Sync);
            func(global, &mut frame)
        }
    }

    /// Create a [`NullFrame`] and call the given closure. A [`NullFrame`] cannot be nested and
    /// can only be used to (mutably) borrow array data. Unlike other scope-methods, no `Global`
    /// is provided to the closure.
    pub fn null_scope<'base, 'julia: 'base, T, F>(&'julia mut self, func: F) -> JlrsResult<T>
    where
        F: FnOnce(&mut NullFrame<'base>) -> JlrsResult<T>,
    {
        unsafe {
            let mut frame = NullFrame::new(self);
            func(&mut frame)
        }
    }

    #[inline(always)]
    fn get_init_page(&mut self) -> &mut StackPage {
        if self.page.is_none() {
            self.page = Some(StackPage::default());
        }

        self.page.as_mut().unwrap()
    }
}

unsafe extern "C" fn droparray(a: Array) {
    // The data of a moved array is allocated by Rust, this function is called by
    // a finalizer in order to ensure it's also freed by Rust.
    let arr_ref = &mut *a.ptr();

    if arr_ref.flags.how() != 2 {
        return;
    }

    let data_ptr = arr_ref.data.cast::<MaybeUninit<u8>>();
    arr_ref.data = null_mut();
    let n_els = arr_ref.elsize as usize * arr_ref.length;
    Vec::from_raw_parts(data_ptr, n_els, n_els);
}
