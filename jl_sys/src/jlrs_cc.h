#include <uv.h>

#ifdef _MSC_VER
#include <windows.h>

template <typename T>
static inline T jl_atomic_load_relaxed(volatile T *obj)
{
    return jl_atomic_load_acquire(obj);
}
#endif

#include <julia.h>
#include <julia_gcext.h>

//! The Julia C API can throw exceptions when used incorrectly, whenever this happens the code
//! will try to jump to the nearest enclosing catch-block. If no enclosing catch-block exists the
//! program is aborted. Because the `JULIA_TRY` and `JULIA_CATCH` macros can't be expressed in
//! Rust without depending on undefined behaviour, this small C library provides
//! `jlrs_catch_wrapper` that is used to call Rust code inside a `JULIA_TRY`/`JULIA_CATCH` block.

#ifdef __cplusplus
extern "C"
{
#endif
#if !defined(JLRS_WINDOWS_LTS)
    typedef enum
    {
        JLRS_CATCH_OK = 0,
        JLRS_CATCH_ERR = 1,
        JLRS_CATCH_EXCECPTION = 2,
        JLRS_CATCH_PANIC = 3,
    } jlrs_catch_tag_t;

    typedef struct
    {
        jlrs_catch_tag_t tag;
        void *error;
    } jlrs_catch_t;

    typedef jlrs_catch_t (*jlrs_callback_caller_t)(void *, void*, void*);
    jlrs_catch_t jlrs_catch_wrapper(void *callback, jlrs_callback_caller_t caller, void *result, void *frame_slice);
#endif

    uint_t jlrs_array_data_owner_offset(uint16_t n_dims);
#if !defined(JLRS_WINDOWS_LTS)
    void jlrs_lock(jl_value_t *v);
    void jlrs_unlock(jl_value_t *v);
#endif

JL_DLLEXPORT void jl_enter_threaded_region(void);
JL_DLLEXPORT void jl_exit_threaded_region(void);
#ifdef __cplusplus
}
#endif