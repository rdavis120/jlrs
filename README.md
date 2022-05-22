# jlrs

[![Rust Docs](https://docs.rs/jlrs/badge.svg)](https://docs.rs/jlrs)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

jlrs is a crate that provides access to most of the Julia C API, it can be used to embed Julia
in Rust applications and to use functionality from the Julia C API when writing `ccall`able
functions in Rust. Currently this crate is only tested on Linux and Windows in combination
with Julia 1.6 and 1.8 and is not compatible with other versions of Julia.

The documentation assumes you're already familiar with the Julia programming language.

An incomplete list of features that are currently supported by jlrs:

 - Access arbitrary Julia modules and their contents.
 - Call Julia functions, including functions that take keyword arguments.
 - Exceptions can be handled or converted to their error message, optionally with color.
 - Include and call your own Julia code.
 - Use a custom system image.
 - Create values that Julia can use, and convert them back to Rust, from Rust.
 - Access the type information and fields of values. The contents of inline and bits-union
   fields can be accessed directly.
 - Create and use n-dimensional arrays. The `jlrs-ndarray` feature can be enabled for
   integration with ndarray.
 - Support for mapping Julia structs to Rust structs that can be generated by JlrsReflect.jl.
 - Structs that can be mapped to Rust include those with type parameters and bits unions.
 - An async runtime is available which can be used from multiple threads and supports
   scheduling Julia `Task`s and `await`ing the result without blocking the runtime thread.

NB: Active development happens on the `dev` branch, the `master` branch points to the most recently released version.

## Prerequisites

Julia must be installed before jlrs can be used. Only version 1.6 and 1.8 are supported. Using version 1.6 requires enabling the `lts` feature.

##### Linux

The recommended way to install Julia is to download the binaries from the official website,
which is distributed in an archive containing a directory called `julia-x.y.z`. This directory
contains several other directories, including a `bin` directory containing the `julia`
executable.

During compilation, the paths to the header and library are normally detected automatically by
executing the command `which julia`. The path to `julia.h` must be
`$(which julia)/../include/julia/julia.h` and the path to the library
`$(which julia)/../lib/libjulia.so`. If you want to override this default behaviour the
`JULIA_DIR` environment variable must be set to the path to the appropriate `julia.x-y-z`
directory; in this case `$JULIA_DIR/include/julia/julia.h` and
`$JULIA_DIR/lib/libjulia.so` are used instead.

In order to be able to load `libjulia.so` this file must be on the library search path. If
this is not the case you must add `/path/to/julia-x.y.z/lib` to the `LD_LIBRARY_PATH`
environment variable. When the `uv` feature is enabled, `/path/to/julia-x.y.z/lib/julia` must
also be added to `LD_LIBRARY_PATH`. The latter path should not be added to the default path
because this can break tools currently installed on your system.

##### Windows

Julia can be installed using juliaup, or with the installer or portable installation
downloaded from the official website. In the first case, Julia has been likely placed in
`%USERPROFILE%\.julia\juliaup\julia-x.y.z+0~x64`, while installing or extracting allows
you to pick the destination. After installation or extraction a folder called `Julia-x.y.z`
exists, which contains several folders including a `bin` folder containing `julia.exe`. The
path to the `bin` folder must be added to the `Path` environment variable.

Julia is automatically detected by executing the command `where julia`. If this returns
multiple locations the first one is used. The default can be overridden by setting the
`JULIA_DIR` environment variable. This doesn't work correctly with juliaup, in this case
the environment variable must be set.

Note that while both Julia 1.6 and 1.8 are supported on Windows, several methods are currently
unavailable when the LTS version is used.

If you use the MSVC target, you must create two or three lib files using `lib.exe`. The def
files required for this can be found in the 
[`def` folder](https://github.com/Taaitaaiger/jlrs/tree/master/jl_sys/def) in the jl-sys crate. 
To create the lib files, copy the three files from either the `lts` or `stable` folder to the 
`bin` folder where Julia is installed. Afterwards, open a Developer Command Prompt for VS19 and
execute the following commands:

```cmd
cd C:\Path\To\Julia-x.y.z\bin
lib /def:libjulia.def /out:libjulia.lib /machine:x64
lib /def:libopenlibm.def /out:libopenlibm.lib /machine:x64
lib /def:libuv-2.def /out:libuv-2.lib /machine:x64
```

The final command only needs to be executed if the `uv` feature has been enabled. If you use
the GNU target these lib files must not exist.

## Features

Most functionality of jlrs is only available if the proper features are enabled. These
features generally belong to one of two categories: runtimes and utilities.

A runtime lets you call Julia from Rust, you must enable one of them if you want to embed
Julia in a Rust application. The following features enable a runtime:

- `sync-rt`
  Enables the sync runtime, `Julia`. The sync runtime provides single-threaded, blocking
  access to the Julia C API.

- `async-rt`
  Enables the async runtime, `AsyncJulia`. The async runtime runs on a separate thread and
  can be used from multiple threads. While access to the C API is single-threaded, the async
  runtime can run multiple tasks in parallel by making use of Julia's task system and Rust's
  async/await syntax. To use this feature you must provide a backing runtime.

- `tokio-rt` and `async-std-rt`
  These features provide a backing runtime for the async runtime. The first uses tokio, the
  second async-std. The `async-rt` feature is automatically enabled when one of these features
  is enabled.

If you're writing a library, either one that will be called from Julia or one that will be
used by a Rust application that embeds Julia, no runtime is required.

In addition to these runtimes, the following utility features are available:

- `prelude`
  Provides a prelude module. This feature is enabled by default.

- `lts`
  Use the current LTS version of Julia (1.6) instead of the current stable version (1.8).

- `async`
  Enable the features of the async runtime which don't depend on the backing runtime. This
  can be used in libraries which provide implementations of tasks that the async runtime can
  handle.

- `jlrs-derive`
  This features should be used in combination with the JlrsReflect.jl package. This package
  generates Rust bindings for Julia structs, these bindings use the custom derive macros to
  enable the safe conversion of data from Julia to Rust, and from Rust to Julia in some cases.

- `jlrs-ndarray`
  Access the contents of a Julia array as an `ArrayView` or `ArrayViewMut` from ndarray.

- `f16`
  Adds support for working with Julia's `Float16` type from Rust using half's `f16` type.

- `ccall`
  Julia's `ccall` interface can be used to call functions written in Rust from Julia. No
  runtime can be used in this case because Julia has already been initialized, when this
  feature is enabled the `CCall` struct is available which offers the same functionality as
  the sync runtime without initializing Julia.

- `uv`
  This feature enables the method `CCall::uv_async_send`, which can be used to wake a Julia
  `AsyncCondition` from Rust. The `ccall` feature is automically enabled when this feature
  is used.

- `pyplot`
  This feature lets you plot data using the Pyplot package and Gtk 3 from Rust.


## Using this crate

If you want to embed Julia and call it from Rust, you must enable a runtime feature:

`jlrs = {version = "0.14", features = ["sync-rt"]}`

`jlrs = {version = "0.14", features = ["tokio-rt"]}`

`jlrs = {version = "0.14", features = ["async-std-rt"]}`

When Julia is embedded in an application, it must be initialized before it can be used. The
following snippet initializes the sync runtime:

```rust
use jlrs::prelude::*;

      // Initializing Julia is unsafe because this can load arbitrary
      // Julia code, and because it can race with other crates unrelated
      // to jlrs. It returns an error if Julia has already been
      // initialized.
      let _julia = unsafe { RuntimeBuilder::new().start().unwrap() };
```

To use the async runtime you must upgrade the `RuntimeBuilder` to an
`AsyncRuntimeBuilder` by providing a backing runtime and channel. Implementations for tokio
and async-std are available if these features have been enabled. For example, an async runtime
backed by tokio and an unbounded channel can be initialized as follows if the `tokio-rt`
feature is enabled:

```rust
use jlrs::prelude::*;

      // Initializing Julia is unsafe for the same reasons as the sync runtime.
      let (_julia, _task_handle) = unsafe {
          RuntimeBuilder::new()
              .async_runtime::<Tokio, UnboundedChannel<_>>()
              .start()
              .unwrap()
      };
```

The async runtime can also be started asynchronously:

```rust
use jlrs::prelude::*;

#[tokio::main]
async fn main() {
      // Initializing Julia is unsafe for the same reasons as the sync runtime.
      let (_julia, _task_handle) = unsafe {
          RuntimeBuilder::new()
              .async_runtime::<Tokio, UnboundedChannel<_>>()
              .start_async()
              .await
              .unwrap()
      };
}
```

If you're calling Rust from Julia everything has already been initialized. If the `ccall`
feature is enabled `CCall` is available which provides the same functionality as the sync
runtime.

### Calling Julia from Rust

This section will focus on some topics that are common between the sync and async runtimes.

After initialization you have an instance of `Julia` or `AsyncJulia`, both provide a
method called `include` that lets you include files with custom Julia code. In order to
create Julia data and call Julia functions, a scope must be created.

When the sync runtime is used this can be done by calling the methods `Julia::scope` and
`Julia::scope_with_capacity`. These methods take a closure with two arguments, a `Global`
and a mutable reference to a `GcFrame` (frame). The first is an access token for global
Julia data, the second is used to root non-global data. While non-global data is rooted, it
won't be freed by Julia's garbage collector (GC). The frame is created when
`Julia::scope(_with_capacity)` is called and dropped when it returns, so any data rooted in
the frame associated with a scope won't be freed by the GC until leaving that scope.

Because `AsyncJulia` is a handle to the async runtime which runs on another thread it's not
possible to directly create a scope. Rather, the async runtime deals with tasks. The simplest
of these is a blocking task, which can be executed by calling
`AsyncJulia::(try_)blocking_task(_with_capacity)`. These methods accept any closure
`Julia::scope` can handle with the additional requirement that it must be `Send` and `Sync`.
It's called a blocking task because the runtime is blocked while executing this task. The
other kinds of tasks that the async runtime can handle will be introduced later.

Inside the closure provided to `Julia::scope` or `AsyncJulia::blocking_task` it's possible to
interact with Julia. Global Julia data can be accessed through its module system, the methods
`Module::main`, `Module::base`, and `Module::core` can be used to access the `Main`,
`Base`, and `Core` modules respectively. The contents of these modules can then be accessed by
calling `Module::function` which returns a `Function`, `Module::global` which returns a
`Value`, and `Module::submodule` which returns another `Module`.

`Value`, `Module`, and `Function` are all examples of pointer wrapper types. Pointer wrapper
types wrap a pointer to some data owned by the GC. Other important examples of pointer
wrapper types are `Array`, `JuliaString` and `DataType`. A `Value` wraps arbitrary Julia
data, all other pointer wrapper types can always be converted to a `Value`. All pointer
wrapper types wrap instances of mutable types defined by the Julia C API.

In addition to pointer wrapper types there are inline wrapper types, these types provide
matching layouts for Julia data. Examples are primitive types like `Float32` and `UInt`, the
inline wrapper types associated with these types are their counterparts, `f32` and `usize`.

`Value` provides several methods to allocate new Julia data. The simplest one is
`Value::eval_string`, which evaluates the contents of the string passed to it and returns
the result as a `Value`. For example, you can evaluate `2` to convert it to  `Value`. In
practice, this method should rarely be used. It can be used to evaluate simple function calls
like `sqrt(2)`, but it must be parsed, compiled, and can't take any arguments. Its most
important use-case is importing installed and standard library packages by evaluating an
`import` or `using` statement.

A more interesting method, `Value::new`, can be used with data of any type that implements
`IntoJulia`. This trait is implemented by primitive types like `i8` and `char`. Any type
that implements `IntoJulia` also implements `Unbox` which is used to extract the contents
of a `Value`. Because `sqrt(2)` returns a `Float64`, it can be unboxed as an `f64`. Pointer
wrapper types don't implement `IntoJulia` or `Unbox`, if they can be created from Rust
they provide methods to do so.

It's possible to call anything that implements `Call` as a Julia function. In addition to
`Function`, this trait is implemented by `Value` itself. Functions can be called with any
number of positional arguments and can be provided with keyword arguments. Keywords must be
provided as a `NamedTuple`, which can be created with the `named_tuple` macro.

Evaluating raw code and calling Julia functions is always unsafe. Nothing prevents you from
calling a function like `nasaldemons() = unsafe_load(Ptr{Float64}(0x05391A445))`. Similarly,
mutating Julia data is unsafe because nothing prevents you from mutating data that shouldn't
be mutated, e.g. the contents of the `Core` module. A full overview of the rules that you
should keep in mind can be found in the `safety` module.

As a simple example, let's convert two numbers to Julia values and add them:

```rust
use jlrs::prelude::*;

// Initializing Julia is unsafe because it can race with another crate that does
// the same.
let mut julia = unsafe { RuntimeBuilder::new().start().unwrap() };

let res = julia.scope(|global, frame| {
    // Create the two arguments. The first argument, something that implements
    // PartialScope, is taken by value and mutable references don't
    // implement Copy, so it's necessary to mutably reborrow the frame.
    let i = Value::new(&mut *frame, 2u64)?;
    let j = Value::new(&mut *frame, 1u32)?;

    // The `+` function can be found in the base module.
    let func = Module::base(global).function(&mut *frame, "+")?;

    // Call the function and unbox the result as a `u64`. The result of the function
    // call is a nested `Result`; the outer error doesn't contain to any Julia
    // data, while the inner error contains the exception if one is thrown. Here the
    // exception is converted to the outer error type by calling `into_jlrs_result`, this new
    // error contains the error message Julia would have shown.
    unsafe { func.call2(&mut *frame, i, j)? }
        .into_jlrs_result()?
        .unbox::<u64>()
}).unwrap();

assert_eq!(res, 3);
```

#### Async and persistent tasks

In addition to blocking tasks, the async runtime lets you execute async tasks which implement
the `AsyncTask` trait, and persistent tasks which implement `PersistentTask`. Both of
these traits are async traits.

An async task is similar to a blocking task, except that you must implement the async `run`
method instead of providing a closure. This method takes a `Global` and a mutable reference to
an `AsyncGcFrame`. This new frame type not only provides access to the same features as
`GcFrame`, it can also be used to call async methods provided by the `CallAsync` trait.
These methods schedule a function call as a new Julia `Task` and can be `await`ed until
this task has completed. The async runtime can switch to another task while the result is
pending, allowing multiple tasks to progress.

The previous example can be rewritten as an async task:

```rust
use jlrs::prelude::*;

struct AdditionTask {
    a: u64,
    b: u32,
}

// Only the runtime thread can call the Julia C API, so the async
// trait methods of `AsyncTask` must not return a future that
// implements `Send` or `Sync`.
#[async_trait(?Send)]
impl AsyncTask for AdditionTask {
    // The type of the result of this task if it succeeds.
    type Output = u64;

    // This async method replaces the closure from the previous examples,
    // an `AsyncGcFrame` can be used the same way as a `GcFrame` but also
    // can be used in combination with methods from the `CallAsync` trait.
    async fn run<'base>(
        &mut self,
        global: Global<'base>,
        frame: &mut AsyncGcFrame<'base>,
    ) -> JlrsResult<Self::Output> {
        let a = Value::new(&mut *frame, self.a)?;
        let b = Value::new(&mut *frame, self.b)?;

        let func = Module::base(global).function(&mut *frame, "+")?;

        // CallAsync::call_async schedules the function call on another
        // thread and returns a Future that resolves when the scheduled
        // function has returned or thrown an error.
        unsafe { func.call_async(&mut *frame, &mut [a, b]) }
            .await?
            .into_jlrs_result()?
            .unbox::<u64>()
    }
}
```

While blocking and async tasks run once and return their result, a persistent task returns a
handle. This handle can be shared across threads and used to call its `run` method. In
addition to a global and an async frame, this method can use the state and input data provided
by the caller.

As an example, let's accumulate some number of values in a Julia array and return the sum of
its contents:

```rust
use jlrs::prelude::*;

struct AccumulatorTask {
    n_values: usize
}

struct AccumulatorTaskState {
    array: TypedArray<'static, 'static, usize>,
    offset: usize
}

// Only the runtime thread can call the Julia C API, so the async trait
// methods of `PersistentTask` must not return a future that implements
// `Send` or `Sync`.
#[async_trait(?Send)]
impl PersistentTask for AccumulatorTask {
    // The type of the result of the task if it succeeds.
    type Output = usize;
    // The type of the task's internal state.
    type State = AccumulatorTaskState;
    // The type of the additional data that the task must be called with.
    type Input = usize;

    // This method is called before the task can be called. Note that the
    // lifetime of the frame is `'static`: the frame is not dropped until
    // the task has completed, so the task's internal state can contain
    // Julia data rooted in this frame.
    async fn init<'inner>(
        &'inner mut self,
        _global: Global<'static>,
        frame: &'inner mut AsyncGcFrame<'static>,
    ) -> JlrsResult<Self::State> {
        // A `Vec` can be moved from Rust to Julia if the element type
        // implements `IntoJulia`.
        let data = vec![0usize; self.n_values];
        let array = Array::from_vec(&mut *frame, data, self.n_values)?
            .try_as_typed::<usize>()?;

        Ok(AccumulatorTaskState {
            array,
            offset: 0
        })
    }

    // Whenever the task is called through its handle this method
    // is called. Unlike `init`, the frame that this method can use
    // is dropped after `run` returns.
    async fn run<'inner, 'frame>(
        &'inner mut self,
        global: Global<'frame>,
        frame: &'inner mut AsyncGcFrame<'frame>,
        state: &'inner mut Self::State,
        input: Self::Input,
    ) -> JlrsResult<Self::Output> {
        {
            // Array data can be directly accessed from Rust.
            // TypedArray::inline_data_mut can be used if the type
            // of the elements is concrete and immutable.
            // This is safe because this is the only active reference to
            // the array.
            let mut data = unsafe { state.array.inline_data_mut(frame)? };
            data[state.offset] = input;

            state.offset += 1;
            if (state.offset == self.n_values) {
                state.offset = 0;
            }
        }

        // Return the sum of the contents of `state.array`.
        unsafe {
            Module::base(global)
                .function(&mut *frame, "sum")?
                .call1(&mut *frame, state.array.as_value())?
                .into_jlrs_result()?
                .unbox::<usize>()
        }
    }
}
```

### Calling Rust from Julia

Julia's `ccall` interface can be used to call `extern "C"` functions defined in Rust, for most
use-cases you shouldn't need jlrs. There are two major ways to use `ccall`, with a pointer to
the function or a `(:function, "library")` pair.

A function can be cast to a void pointer and converted to a `Value`:

```rust
use jlrs::prelude::*;

// This function will be provided to Julia as a pointer, so its name can be mangled.
unsafe extern "C" fn call_me(arg: bool) -> isize {
    if arg {
        1
    } else {
        -1
    }
}

let mut julia = unsafe { RuntimeBuilder::new().start().unwrap() };
julia.scope(|global, frame| unsafe {
    // Cast the function to a void pointer
    let call_me_val = Value::new(&mut *frame, call_me as *mut std::ffi::c_void)?;

    // Value::eval_string can be used to create new functions.
    let func = Value::eval_string(
        &mut *frame,
        "myfunc(callme::Ptr{Cvoid})::Int = ccall(callme, Int, (Bool,), true)"
    )?.into_jlrs_result()?;

    // Call the function and unbox the result.
    let output = func.call1(&mut *frame, call_me_val)?
        .into_jlrs_result()?
        .unbox::<isize>()?;

    assert_eq!(output, 1);

    Ok(())
}).unwrap();
```

You can also use functions defined in `dylib` and `cdylib` libraries. In order to create such
a library you need to add

```toml
[lib]
crate-type = ["dylib"]
```

or

```toml
[lib]
crate-type = ["cdylib"]
```

respectively to your crate's `Cargo.toml`. Use a `dylib` if you want to use the crate in other
Rust crates, but if it's only intended to be called through `ccall` a `cdylib` is the better
choice. On Linux, such a crate will be compiled to `lib<crate_name>.so`.

The functions you want to use with `ccall` must be both `extern "C"` functions to ensure the C
ABI is used, and annotated with `#[no_mangle]` to prevent name mangling. Julia can find
libraries in directories that are either on the default library search path or included by
setting the `LD_LIBRARY_PATH` environment variable on Linux. If the compiled library is not
directly visible to Julia, you can open it with `Libdl.dlopen` and acquire function pointers
with `Libdl.dlsym`. These pointers can be called the same way as the pointer in the previous
example.

If the library is visible to Julia you can access it using the library name. If `call_me` is
defined in a crate called `foo`, the following should work:

```julia
ccall((:call_me, "libfoo"), Int, (Bool,), false)
```

One important aspect of calling Rust from other languages in general is that panicking across
an FFI boundary is undefined behaviour. If you're not sure your code will never panic, wrap it
with `std::panic::catch_unwind`.

Most features provided by jlrs including accessing modules, calling functions, and borrowing
array data require a `Global` or a frame. You can access these by creating an instance of
`CCall` first. Another method provided by `CCall` is `CCall::uv_async_send`, this method
can be used to wake an `Base.AsyncCondition`. In particular, it can be used to write a
`ccall`able function that does its actual work on another thread, returns early and then
`wait`ing on the async condition from Julia. The advantage of this is that the long-running
function won't block Julia. In this case you will need to use `GC.@preserve` to ensure Julia
is aware that the use of this data is still in use after the `ccall` has returned.


## Testing

The restriction that Julia can be initialized once must be taken into account when running
tests that use `jlrs`. The recommended approach is to create a thread-local static `RefCell`:

```rust
use jlrs::prelude::*;
use std::cell::RefCell;
thread_local! {
    pub static JULIA: RefCell<Julia> = {
        let julia = RefCell::new(unsafe { RuntimeBuilder::new().start().unwrap() });

        /* include everything you need to use */

        julia
    };
}
```

A similar approach works for the async runtimes:

```rust
use jlrs::prelude::*;
use std::{cell::RefCell, time::Duration};
thread_local! {
    pub static JULIA: RefCell<AsyncJulia<Tokio>> = {
        let julia = RefCell::new(unsafe {
            RuntimeBuilder::new()
                .async_runtime::<Tokio, UnboundedChannel<_>>()
                .start()
                .unwrap()
                .0
        });

        /* include everything you need to use */

        julia
    };
}
```

Tests that use these constructs can only use one thread for testing, so you must use
`cargo test -- --test-threads=1`, otherwise the code above will panic when a test tries to
initialize Julia a second time from another thread.

If you want to run all of jlrs's tests, all these requirements must be taken into account:
cargo test --features sync-rt,jlrs-ndarray,f16,uv,jlrs-derive,tokio-rt,async-std-rt -- --test-threads=1`


## Custom types

In order to map a struct in Rust to one in Julia you can derive `ValidLayout`, `Unbox`,
and `Typecheck`. If the struct in Julia has no type parameters and is a bits type you can
also derive `IntoJulia`.

You normally shouldn't need to implement these structs or traits manually. The JlrsReflect
package can generate correct Rust struct and automatically derive the supported traits for
types that have no atomic fields, nor any tuple or union fields with type parameters. The
reason for this restriction is that the layout of such fields can be very different in a way
that can't be easily represented.

These custom types can also be used when you call Rust from Julia with `ccall`.
