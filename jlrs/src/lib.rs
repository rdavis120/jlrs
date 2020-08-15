//! The main goal behind `jlrs` is to provide a simple and safe interface to the Julia C API.
//! Currently this crate is only tested on Linux and Windows in combination with Julia 1.5.0, if
//! you try to use it with an earlier version of Julia it will fail to generate the bindings.
//!
//!
//! # Features
//!
//! An incomplete list of features that are currently supported by `jlrs`:
//!
//!  - Access arbitrary Julia modules and their contents.
//!  - Call arbitrary Julia functions.
//!  - Include and use your own Julia code.
//!  - Load a custom system image.
//!  - Create values that Julia can use, and convert them back to Rust, from Rust.
//!  - Access the type information and fields of values and check their properties.
//!  - Create and use n-dimensional arrays.
//!  - Support for mapping Julia structs to Rust structs, which can be generated with `JlrsReflect.jl`.
//!  - Structs that can be mapped to Rust include those with type parameters and bits unions.
//!
//!
//! # Generating the bindings
//!
//! This crate depends on `jl-sys` which contains the raw bindings to the Julia C API, these are
//! generated by `bindgen`. You can find the requirements for using `bindgen` in
//! [their User Guide](https://rust-lang.github.io/rust-bindgen/requirements.html).
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
//! The environment variable can be used to override the default. Similarly, in order to load
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
//! bring all the structs and traits you're likely to need into scope. Before you can use Julia it
//! must first be initialized. You do this by calling [`Julia::init`]. Note that this method can
//! only be called once, if you drop [`Julia`] you won't be able to create a new one and have to
//! restart the entire program. If you want to use a custom system image, you must call
//! [`Julia::init_with_image`] instead of [`Julia::init`].
//!
//! You can call [`Julia::include`] to include your own Julia code and either [`Julia::frame`] or
//! [`Julia::dynamic_frame`] to interact with Julia. If you want to have improved support for
//! backtraces `jlrs.jl` must be included. You can find this file in the root of this crate's github
//! repository. This is necessary because this functionality depends on some Julia code defined in
//! that file.
//!
//! The other two methods, [`Julia::frame`] and [`Julia::dynamic_frame`], take a closure that
//! provides you with a [`Global`], and either a [`StaticFrame`] or [`DynamicFrame`] respectively.
//! [`Global`] is a token that lets you access Julia modules their contents, and other global
//! values, while the frames are used to deal with local Julia data.
//!
//! Local data must be handled properly: Julia is a programming language with a garbage collector
//! that is unaware of any references to data outside of Julia. In order to make it aware of this
//! usage a stack must be maintained. You choose this stack's size when calling [`Julia::init`].
//! The elements of this stack are called stack frames; they contain a pointer to the previous
//! frame, the number of protected values, and that number of pointers to values. The two frame
//! types offered by `jlrs` take care of all the technical details, a [`DynamicFrame`] will grow
//! to the required size while a [`StaticFrame`] has a definite number of slots. These frames can
//! be nested (ie stacked) arbitrarily.
//!
//! In order to call a Julia function, you'll need two things: a function to call, and arguments
//! to call it with. You can acquire the function through the module that defines it with
//! [`Module::function`]; [`Module::base`] and [`Module::core`] provide access to Julia's `Base`
//! and `Core` module respectively, while everything you include through [`Julia::include`] is
//! made available relative to the `Main` module which you can access by calling [`Module::main`].
//!
//! Julia data is represented by a [`Value`]. Basic data types like numbers, booleans, and strings
//! can be created through [`Value::new`] and several methods exist to create an n-dimensional
//! array. Each value will be protected by a frame, and the two share a lifetime in order to
//! enforce that a value can only be used as long as its protecting frame hasn't been dropped.
//! Julia functions, their arguments and their results are all `Value`s too. All `Value`s can be
//! called as functions, whether this will succeed depends on the value actually being a function.
//! You can copy data from Julia to Rust by calling [`Value::cast`].
//!
//! As a simple example, let's create two values and add them:
//!
//! ```no_run
//! # use jlrs::prelude::*;
//! # fn main() {
//! let mut julia = unsafe { Julia::init(16).unwrap() };
//! julia.dynamic_frame(|global, frame| {
//!     // Create the two arguments
//!     let i = Value::new(frame, 2u64)?;
//!     let j = Value::new(frame, 1u32)?;
//!
//!     // We can find the addition-function in the base module
//!     let func = Module::base(global).function("+")?;
//!
//!     // Call the function and unbox the result
//!     let output = func.call2(frame, i, j)?.unwrap();
//!     output.cast::<u64>()
//! }).unwrap();
//! # }
//! ```
//!
//! You can also do this with a static frame:
//!
//! ```no_run
//! # use jlrs::prelude::*;
//! # fn main() {
//! let mut julia = unsafe { Julia::init(16).unwrap() };
//! // Three slots; two for the inputs and one for the output.
//! julia.frame(3, |global, frame| {
//!     // Create the two arguments, each value requires one slot
//!     let i = Value::new(frame, 2u64)?;
//!     let j = Value::new(frame, 1u32)?;
//!
//!     // We can find the addition-function in the base module
//!     let func = Module::base(global).function("+")?;
//!
//!     // Call the function and unbox the result.  
//!     let output = func.call2(frame, i, j)?.unwrap();
//!     output.cast::<u64>()
//! }).unwrap();
//! # }
//! ```
//!
//! This is only a small example, other things can be done with [`Value`] as well: their fields
//! can be accessed if the [`Value`] is some tuple or struct. They can contain more complex data;
//! if a function returns an array or a module it will still be returned as a [`Value`]. There
//! complex types are compatible with [`Value::cast`]. Additionally, you can create [`Output`]s in
//! a frame in order to protect a value from with a specific frame; this value will share that
//! frame's lifetime.
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
//! generate the correct Rust struct for types that don't include any unions or tuples with type
//! parameters. The reason for this restriction is that the layout of tuple and union fields can
//! be very different depending on these parameters in a way that can't be nicely expressed in
//! Rust.
//!
//!
//! # Lifetimes
//!
//! While reading the documentation for this crate, you will see that a lot of lifetimes are used.
//! Most of these lifetimes have a specific meaning:
//!
//! - `'base` is the lifetime of a frame created through [`Julia::frame`] or
//! [`Julia::dynamic_frame`]. This lifetime prevents you from using global Julia data outside of a
//! frame.
//!
//! - `'frame` is the lifetime of an arbitrary frame; in the base frame it will be the same as
//! `'base`. This lifetime prevents you from using Julia data after the frame that protects it
//! from garbage collection goes out of scope.
//!
//! - `'data` or `'borrow` is the lifetime of data that is borrowed. This lifetime prevents you
//! from mutably aliasing data and trying to use it after the borrowed data is dropped.
//!
//! - `'output` is the lifetime of the frame that created the output. This lifetime ensures that
//! when Julia data is protected by an older frame this data can be used until that frame goes out
//! of scope.
//!
//! [`prelude`]: prelude/index.html
//! [`Julia`]: struct.Julia.html
//! [`Julia::init`]: struct.Julia.html#method.init
//! [`Julia::init_with_image`]: struct.Julia.html#method.init_with_image
//! [`Julia::include`]: struct.Julia.html#method.include
//! [`Julia::frame`]: struct.Julia.html#method.frame
//! [`Julia::dynamic_frame`]: struct.Julia.html#method.dynamic_frame
//! [`Global`]: global/struct.Global.html
//! [`Output`]: frame/struct.Output.html
//! [`StaticFrame`]: frame/struct.StaticFrame.html
//! [`DynamicFrame`]: frame/struct.DynamicFrame.html
//! [`Frame`]: traits/trait.Frame.html
//! [`JuliaStruct`]: traits/trait.JuliaStruct.html
//! [`Module::function`]: value/module/struct.Module.html#method.function
//! [`Module::base`]: value/module/struct.Module.html#method.base
//! [`Module::core`]: value/module/struct.Module.html#method.core
//! [`Module::main`]: value/module/struct.Module.html#method.main
//! [`Value`]: value/struct.Value.html
//! [`Value::new`]: value/struct.Value.html#method.new
//! [`Value::cast`]: value/struct.Value.html#method.cast
//! [the instructions for compiling Julia on Windows using Cygwin and MinGW]: https://github.com/JuliaLang/julia/blob/v1.4.1/doc/build/windows.md#cygwin-to-mingw-cross-compiling

pub mod error;
pub mod frame;
pub mod global;
#[doc(hidden)]
pub mod jl_sys_export;
pub mod prelude;
mod stack;
pub mod traits;
#[doc(hidden)]
pub mod util;
pub mod value;

use error::{JlrsError, JlrsResult};
use frame::{DynamicFrame, StaticFrame};
use global::Global;
use jl_sys::{jl_atexit_hook, jl_init, jl_init_with_image__threading};
use stack::{Dynamic, RawStack, StackView, Static};
use std::io::{Error as IOError, ErrorKind};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use value::module::Module;
use value::Value;

static INIT: AtomicBool = AtomicBool::new(false);

/// This struct can be created only once during the lifetime of your program. You must create it
/// with [`Julia::init`] or [`Julia::init_with_image`] before you can do anything related to
/// Julia. While this struct exists, Julia is active; dropping it causes the shutdown code to be
/// called.
///
/// [`Julia::init`]: struct.Julia.html#method.init
/// [`Julia::init_with_image`]: struct.Julia.html#method.init_with_image
pub struct Julia {
    stack: RawStack,
}

impl Julia {
    /// Initializes Julia, this function can only be called once. If you call it a second time it
    /// will return an error. If this struct is dropped, you will need to restart your program to
    /// be able to call Julia code again.
    ///
    /// You have to choose a stack size when calling this function. This will be the total number
    /// of slots that will be available for the GC stack. One of these slots will always be in
    /// use. Each frame needs two slots of overhead, plus one for every value created with that
    /// frame. A [`StaticFrame`] preallocates its slots, while a [`DynamicFrame`] grows to the
    /// required size. If calling a method requires one or more slots, this amount is explicitly
    /// documented.
    ///
    /// This function is unsafe because this crate provides you with a way to execute arbitrary
    /// Julia code which can't be checked for correctness.
    ///
    /// [`StaticFrame`]: frame/struct.StaticFrame.html
    /// [`DynamicFrame`]: frame/struct.DynamicFrame.html
    pub unsafe fn init(stack_size: usize) -> JlrsResult<Self> {
        if INIT.swap(true, Ordering::SeqCst) {
            return Err(JlrsError::AlreadyInitialized.into());
        }

        jl_init();

        Ok(Julia {
            stack: RawStack::new(stack_size),
        })
    }

    /// This function is similar to [`Julia::init`] except that it loads a custom system image. A
    /// custom image can be generated with the [`PackageCompiler`] package for Julia. The main
    /// advantage of using a custom image over the default one is that it allows you to avoid much
    /// of the compilation overhead often associated with Julia.
    ///
    /// Two additional arguments are required to call this function compared to [`Julia::init`];
    /// `julia_bindir` and `image_relative_path`. The first must be the absolute path to a
    /// directory that contains a compatible Julia binary (eg `${JULIA_DIR}/bin`), the second must
    /// be either an absolute or a relative path to a system image.
    ///
    /// This function will return an error if either of the two paths does not exist or if Julia
    /// has already been initialized.
    ///
    /// [`Julia::init`]: struct.Julia.html#init
    /// [`PackageCompiler`]: https://julialang.github.io/PackageCompiler.jl/dev/
    pub unsafe fn init_with_image<P: AsRef<Path>>(
        stack_size: usize,
        julia_bindir: P,
        image_path: P,
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

        let bindir = std::ffi::CString::new(julia_bindir_str).unwrap();
        let im_rel_path = std::ffi::CString::new(image_path_str).unwrap();

        jl_init_with_image__threading(bindir.as_ptr(), im_rel_path.as_ptr());

        Ok(Julia {
            stack: RawStack::new(stack_size),
        })
    }

    /// Change the stack size to `stack_size`.
    pub fn set_stack_size(&mut self, stack_size: usize) {
        unsafe { self.stack = RawStack::new(stack_size) }
    }

    /// Returns the current stack size.
    pub fn stack_size(&self) -> usize {
        self.stack.size()
    }

    /// Calls `include` in the `Main` module in Julia, which executes the file's contents in that
    /// module. This has the same effect as calling `include` in the Julia REPL.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use jlrs::prelude::*;
    /// # fn main() {
    /// # let mut julia = unsafe { Julia::init(16).unwrap() };
    /// julia.include("MyJuliaCode.jl").unwrap();
    /// # }
    /// ```
    pub fn include<P: AsRef<Path>>(&mut self, path: P) -> JlrsResult<()> {
        if path.as_ref().exists() {
            return self.frame(3, |global, frame| {
                let path_jl_str = Value::new(frame, path.as_ref().to_string_lossy())?;
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

    /// Create a [`StaticFrame`] that can hold `capacity` values, and call the given closure.
    /// Returns the result of this closure, or an error if the new frame can't be created because
    /// there's not enough space on the GC stack. The number of required slots on the stack is
    /// `capacity + 2`.
    ///
    /// Every output and value you create inside the closure using the [`StaticFrame`], either
    /// directly or through calling a [`Value`], will reduce the available capacity of the
    /// [`StaticFrame`] by 1.
    ///
    /// Example:
    ///
    /// ```
    /// # use jlrs::prelude::*;
    /// # use jlrs::util::JULIA;
    /// # fn main() {
    /// # JULIA.with(|j| {
    /// # let mut julia = j.borrow_mut();
    ///   julia.frame(1, |_global, frame| {
    ///       let i = Value::new(frame, 1u64)?;
    ///       Ok(())
    ///   }).unwrap();
    /// # });
    /// # }
    /// ```
    ///
    /// [`StaticFrame`]: ../frame/struct.StaticFrame.html
    /// [`Value`]: ../value/struct.Value.html
    pub fn frame<'base, 'julia: 'base, T, F>(
        &'julia mut self,
        capacity: usize,
        func: F,
    ) -> JlrsResult<T>
    where
        F: FnOnce(Global<'base>, &mut StaticFrame<'base>) -> JlrsResult<T>,
    {
        unsafe {
            let d = self.stack.as_mut();
            let global = Global::new();
            let mut view = StackView::<Static>::new(d);
            let frame_idx = view.new_frame(capacity)?;
            let mut frame = StaticFrame::with_capacity(frame_idx, capacity, view);
            func(global, &mut frame)
        }
    }

    /// Create a [`DynamicFrame`] and call the given closure. Returns the result of this closure,
    /// or an error if the new frame can't be created because the stack is too small. The number
    /// of required slots on the stack is 2.
    ///
    /// Every output and value you create inside the closure using the [`DynamicFrame`], either
    /// directly or through calling a [`Value`], will occupy a single slot on the GC stack.
    ///
    /// Example:
    ///
    /// ```
    /// # use jlrs::prelude::*;
    /// # use jlrs::util::JULIA;
    /// # fn main() {
    /// # JULIA.with(|j| {
    /// # let mut julia = j.borrow_mut();
    /// julia.dynamic_frame(|_global, frame| {
    ///     let j = Value::new(frame, 1u64)?;
    ///     Ok(())
    /// }).unwrap();
    /// # });
    /// # }
    /// ```
    ///
    /// [`DynamicFrame`]: ../frame/struct.DynamicFrame.html
    /// [`Value`]: ../value/struct.Value.html
    pub fn dynamic_frame<'base, 'julia: 'base, T, F>(&'julia mut self, func: F) -> JlrsResult<T>
    where
        F: FnOnce(Global<'base>, &mut DynamicFrame<'base>) -> JlrsResult<T>,
    {
        unsafe {
            let d = self.stack.as_mut();
            let global = Global::new();
            let mut view = StackView::<Dynamic>::new(d);
            let frame_idx = view.new_frame()?;
            let mut frame = DynamicFrame::new(frame_idx, view);
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
