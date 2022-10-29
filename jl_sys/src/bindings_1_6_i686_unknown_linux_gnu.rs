/* generated from Julia version 1.6.7 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __uint64_t = ::std::os::raw::c_ulonglong;
pub type __sig_atomic_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [::std::os::raw::c_ulong; 32usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __atomic_wide_counter {
    pub __value64: ::std::os::raw::c_ulonglong,
    pub __value32: __atomic_wide_counter__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __atomic_wide_counter__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_slist {
    pub __next: *mut __pthread_internal_slist,
}
pub type __pthread_slist_t = __pthread_internal_slist;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: ::std::os::raw::c_int,
    pub __count: ::std::os::raw::c_uint,
    pub __owner: ::std::os::raw::c_int,
    pub __kind: ::std::os::raw::c_int,
    pub __nusers: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: __pthread_mutex_s__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_mutex_s__bindgen_ty_1 {
    pub __elision_data: __pthread_mutex_s__bindgen_ty_1__bindgen_ty_1,
    pub __list: __pthread_slist_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s__bindgen_ty_1__bindgen_ty_1 {
    pub __espins: ::std::os::raw::c_short,
    pub __eelision: ::std::os::raw::c_short,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [::std::os::raw::c_uint; 2usize],
    pub __g_size: [::std::os::raw::c_uint; 2usize],
    pub __g1_orig_size: ::std::os::raw::c_uint,
    pub __wrefs: ::std::os::raw::c_uint,
    pub __g_signals: [::std::os::raw::c_uint; 2usize],
}
pub type pthread_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::std::os::raw::c_char; 24usize],
    pub __align: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::std::os::raw::c_char; 48usize],
    pub __align: ::std::os::raw::c_longlong,
}
pub type sig_atomic_t = __sig_atomic_t;
pub type __jmp_buf = [::std::os::raw::c_int; 6usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: ::std::os::raw::c_int,
    pub __saved_mask: __sigset_t,
}
pub type uv_mutex_t = pthread_mutex_t;
pub type uv_cond_t = pthread_cond_t;
pub type uint_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct htable_t {
    pub size: usize,
    pub table: *mut *mut ::std::os::raw::c_void,
    pub _space: [*mut ::std::os::raw::c_void; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct arraylist_t {
    pub len: usize,
    pub max: usize,
    pub items: *mut *mut ::std::os::raw::c_void,
    pub _space: [*mut ::std::os::raw::c_void; 29usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct small_arraylist_t {
    pub len: u32,
    pub max: u32,
    pub items: *mut *mut ::std::os::raw::c_void,
    pub _space: [*mut ::std::os::raw::c_void; 6usize],
}
pub type sigjmp_buf = [__jmp_buf_tag; 1usize];
pub type jl_taggedvalue_t = _jl_taggedvalue_t;
pub type jl_tls_states_t = _jl_tls_states_t;
pub type jl_ptls_t = *mut jl_tls_states_t;
extern "C" {
    pub fn jl_get_ptls_states() -> jl_ptls_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_ucontext_t {
    pub uc_mcontext: sigjmp_buf,
}
pub type jl_thread_t = pthread_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_mutex_t {
    pub owner: jl_thread_t,
    pub count: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_gc_pool_t {
    pub freelist: *mut jl_taggedvalue_t,
    pub newpages: *mut jl_taggedvalue_t,
    pub osize: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_thread_gc_num_t {
    pub allocd: i64,
    pub freed: i64,
    pub malloc: u64,
    pub realloc: u64,
    pub poolalloc: u64,
    pub bigalloc: u64,
    pub freecall: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_thread_heap_t {
    pub weak_refs: arraylist_t,
    pub live_tasks: arraylist_t,
    pub mallocarrays: *mut _mallocarray_t,
    pub mafreelist: *mut _mallocarray_t,
    pub big_objects: *mut _bigval_t,
    pub rem_bindings: arraylist_t,
    pub _remset: [arraylist_t; 2usize],
    pub remset_nptr: ::std::os::raw::c_int,
    pub remset: *mut arraylist_t,
    pub last_remset: *mut arraylist_t,
    pub norm_pools: [jl_gc_pool_t; 43usize],
    pub free_stacks: [arraylist_t; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _jl_gc_mark_data {
    _unused: [u8; 0],
}
pub type jl_gc_mark_data_t = _jl_gc_mark_data;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_gc_mark_sp_t {
    pub pc: *mut *mut ::std::os::raw::c_void,
    pub data: *mut jl_gc_mark_data_t,
    pub pc_start: *mut *mut ::std::os::raw::c_void,
    pub pc_end: *mut *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_gc_mark_cache_t {
    pub perm_scanned_bytes: usize,
    pub scanned_bytes: usize,
    pub nbig_obj: usize,
    pub big_obj: [*mut ::std::os::raw::c_void; 1024usize],
    pub stack_lock: jl_mutex_t,
    pub pc_stack: *mut *mut ::std::os::raw::c_void,
    pub pc_stack_end: *mut *mut ::std::os::raw::c_void,
    pub data_stack: *mut jl_gc_mark_data_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jl_bt_element_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _jl_tls_states_t {
    pub pgcstack: *mut _jl_gcframe_t,
    pub world_age: usize,
    pub tid: i16,
    pub rngseed: u64,
    pub safepoint: *mut usize,
    pub sleep_check_state: i8,
    pub gc_state: i8,
    pub in_finalizer: i8,
    pub disable_gc: i8,
    pub heap: jl_thread_heap_t,
    pub gc_num: jl_thread_gc_num_t,
    pub sleep_lock: uv_mutex_t,
    pub wake_signal: uv_cond_t,
    pub defer_signal: sig_atomic_t,
    pub current_task: *mut _jl_task_t,
    pub next_task: *mut _jl_task_t,
    pub root_task: *mut _jl_task_t,
    pub timing_stack: *mut _jl_timing_block_t,
    pub stackbase: *mut ::std::os::raw::c_void,
    pub stacksize: usize,
    pub base_ctx: jl_ucontext_t,
    pub safe_restore: *mut sigjmp_buf,
    pub sig_exception: *mut _jl_value_t,
    pub bt_data: *mut _jl_bt_element_t,
    pub bt_size: usize,
    pub signal_request: sig_atomic_t,
    pub io_wait: sig_atomic_t,
    pub signal_stack: *mut ::std::os::raw::c_void,
    pub system_id: jl_thread_t,
    pub in_pure_callback: ::std::os::raw::c_int,
    pub finalizers_inhibited: ::std::os::raw::c_int,
    pub finalizers: arraylist_t,
    pub gc_cache: jl_gc_mark_cache_t,
    pub sweep_objs: arraylist_t,
    pub gc_mark_sp: jl_gc_mark_sp_t,
    pub previous_exception: *mut _jl_value_t,
    pub locks: small_arraylist_t,
}
extern "C" {
    pub fn jl_gc_safepoint();
}
pub type jl_value_t = _jl_value_t;
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct _jl_taggedvalue_bits {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: [u8; 3usize],
}
impl _jl_taggedvalue_bits {
    #[inline]
    pub fn gc(&self) -> usize {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_gc(&mut self, val: usize) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(gc: usize) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 2u8, {
            let gc: u32 = unsafe { ::std::mem::transmute(gc) };
            gc as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _jl_taggedvalue_t {
    pub __bindgen_anon_1: _jl_taggedvalue_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _jl_taggedvalue_t__bindgen_ty_1 {
    pub header: usize,
    pub next: *mut jl_taggedvalue_t,
    pub type_: *mut jl_value_t,
    pub bits: _jl_taggedvalue_bits,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jl_sym_t {
    pub left: *mut _jl_sym_t,
    pub right: *mut _jl_sym_t,
    pub hash: usize,
}
pub type jl_sym_t = _jl_sym_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_svec_t {
    pub length: usize,
}
#[repr(C)]
#[repr(align(2))]
#[derive(Debug, Copy, Clone)]
pub struct jl_array_flags_t {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
}
impl jl_array_flags_t {
    #[inline]
    pub fn how(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 2u8) as u16) }
    }
    #[inline]
    pub fn set_how(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn ndims(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 9u8) as u16) }
    }
    #[inline]
    pub fn set_ndims(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 9u8, val as u64)
        }
    }
    #[inline]
    pub fn pooled(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_pooled(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(11usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn ptrarray(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_ptrarray(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(12usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasptr(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_hasptr(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn isshared(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_isshared(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(14usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn isaligned(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(15usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_isaligned(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(15usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        how: u16,
        ndims: u16,
        pooled: u16,
        ptrarray: u16,
        hasptr: u16,
        isshared: u16,
        isaligned: u16,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 2u8, {
            let how: u16 = unsafe { ::std::mem::transmute(how) };
            how as u64
        });
        __bindgen_bitfield_unit.set(2usize, 9u8, {
            let ndims: u16 = unsafe { ::std::mem::transmute(ndims) };
            ndims as u64
        });
        __bindgen_bitfield_unit.set(11usize, 1u8, {
            let pooled: u16 = unsafe { ::std::mem::transmute(pooled) };
            pooled as u64
        });
        __bindgen_bitfield_unit.set(12usize, 1u8, {
            let ptrarray: u16 = unsafe { ::std::mem::transmute(ptrarray) };
            ptrarray as u64
        });
        __bindgen_bitfield_unit.set(13usize, 1u8, {
            let hasptr: u16 = unsafe { ::std::mem::transmute(hasptr) };
            hasptr as u64
        });
        __bindgen_bitfield_unit.set(14usize, 1u8, {
            let isshared: u16 = unsafe { ::std::mem::transmute(isshared) };
            isshared as u64
        });
        __bindgen_bitfield_unit.set(15usize, 1u8, {
            let isaligned: u16 = unsafe { ::std::mem::transmute(isaligned) };
            isaligned as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jl_array_t {
    pub data: *mut ::std::os::raw::c_void,
    pub length: usize,
    pub flags: jl_array_flags_t,
    pub elsize: u16,
    pub offset: u32,
    pub nrows: usize,
    pub __bindgen_anon_1: jl_array_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union jl_array_t__bindgen_ty_1 {
    pub maxsize: usize,
    pub ncols: usize,
}
pub type jl_tupletype_t = _jl_datatype_t;
pub type jl_typemap_t = jl_value_t;
pub type jl_call_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut jl_value_t,
        arg2: *mut *mut jl_value_t,
        arg3: u32,
        arg4: *mut _jl_code_instance_t,
    ) -> *mut jl_value_t,
>;
pub type jl_callptr_t = jl_call_t;
pub type jl_fptr_args_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut jl_value_t,
        arg2: *mut *mut jl_value_t,
        arg3: u32,
    ) -> *mut jl_value_t,
>;
pub type jl_fptr_sparam_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut jl_value_t,
        arg2: *mut *mut jl_value_t,
        arg3: u32,
        arg4: *mut jl_svec_t,
    ) -> *mut jl_value_t,
>;
#[repr(C)]
#[derive(Copy, Clone)]
pub union jl_generic_specptr_t {
    pub fptr: *mut ::std::os::raw::c_void,
    pub fptr1: jl_fptr_args_t,
    pub fptr3: jl_fptr_sparam_t,
}
pub type jl_method_instance_t = _jl_method_instance_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jl_method_t {
    pub name: *mut jl_sym_t,
    pub module: *mut _jl_module_t,
    pub file: *mut jl_sym_t,
    pub line: i32,
    pub primary_world: usize,
    pub deleted_world: usize,
    pub sig: *mut jl_value_t,
    pub specializations: *mut jl_svec_t,
    pub speckeyset: *mut jl_array_t,
    pub slot_syms: *mut jl_value_t,
    pub source: *mut jl_value_t,
    pub unspecialized: *mut _jl_method_instance_t,
    pub generator: *mut jl_value_t,
    pub roots: *mut jl_array_t,
    pub ccallable: *mut jl_svec_t,
    pub invokes: *mut jl_typemap_t,
    pub nargs: i32,
    pub called: i32,
    pub nospecialize: i32,
    pub nkw: i32,
    pub isva: u8,
    pub pure_: u8,
    pub writelock: jl_mutex_t,
}
pub type jl_method_t = _jl_method_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _jl_method_instance_t {
    pub def: _jl_method_instance_t__bindgen_ty_1,
    pub specTypes: *mut jl_value_t,
    pub sparam_vals: *mut jl_svec_t,
    pub uninferred: *mut jl_value_t,
    pub backedges: *mut jl_array_t,
    pub callbacks: *mut jl_array_t,
    pub cache: *mut _jl_code_instance_t,
    pub inInference: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _jl_method_instance_t__bindgen_ty_1 {
    pub value: *mut jl_value_t,
    pub module: *mut _jl_module_t,
    pub method: *mut jl_method_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _jl_code_instance_t {
    pub def: *mut jl_method_instance_t,
    pub next: *mut _jl_code_instance_t,
    pub min_world: usize,
    pub max_world: usize,
    pub rettype: *mut jl_value_t,
    pub rettype_const: *mut jl_value_t,
    pub inferred: *mut jl_value_t,
    pub isspecsig: u8,
    pub precompile: u8,
    pub invoke: jl_callptr_t,
    pub specptr: jl_generic_specptr_t,
}
pub type jl_code_instance_t = _jl_code_instance_t;
pub type jl_function_t = jl_value_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_tvar_t {
    pub name: *mut jl_sym_t,
    pub lb: *mut jl_value_t,
    pub ub: *mut jl_value_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_unionall_t {
    pub var: *mut jl_tvar_t,
    pub body: *mut jl_value_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_typename_t {
    pub name: *mut jl_sym_t,
    pub module: *mut _jl_module_t,
    pub names: *mut jl_svec_t,
    pub wrapper: *mut jl_value_t,
    pub cache: *mut jl_svec_t,
    pub linearcache: *mut jl_svec_t,
    pub hash: isize,
    pub mt: *mut _jl_methtable_t,
    pub partial: *mut jl_array_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_uniontype_t {
    pub a: *mut jl_value_t,
    pub b: *mut jl_value_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_fielddesc8_t {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub offset: u8,
}
impl jl_fielddesc8_t {
    #[inline]
    pub fn isptr(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_isptr(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn size(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 7u8) as u8) }
    }
    #[inline]
    pub fn set_size(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(isptr: u8, size: u8) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let isptr: u8 = unsafe { ::std::mem::transmute(isptr) };
            isptr as u64
        });
        __bindgen_bitfield_unit.set(1usize, 7u8, {
            let size: u8 = unsafe { ::std::mem::transmute(size) };
            size as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_fielddesc16_t {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
    pub offset: u16,
}
impl jl_fielddesc16_t {
    #[inline]
    pub fn isptr(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_isptr(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn size(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 15u8) as u16) }
    }
    #[inline]
    pub fn set_size(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 15u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(isptr: u16, size: u16) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let isptr: u16 = unsafe { ::std::mem::transmute(isptr) };
            isptr as u64
        });
        __bindgen_bitfield_unit.set(1usize, 15u8, {
            let size: u16 = unsafe { ::std::mem::transmute(size) };
            size as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_fielddesc32_t {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub offset: u32,
}
impl jl_fielddesc32_t {
    #[inline]
    pub fn isptr(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_isptr(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn size(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 31u8) as u32) }
    }
    #[inline]
    pub fn set_size(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 31u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(isptr: u32, size: u32) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let isptr: u32 = unsafe { ::std::mem::transmute(isptr) };
            isptr as u64
        });
        __bindgen_bitfield_unit.set(1usize, 31u8, {
            let size: u32 = unsafe { ::std::mem::transmute(size) };
            size as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_datatype_layout_t {
    pub nfields: u32,
    pub npointers: u32,
    pub first_ptr: i32,
    pub alignment: u16,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: u8,
}
impl jl_datatype_layout_t {
    #[inline]
    pub fn haspadding(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_haspadding(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn fielddesc_type(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 2u8) as u16) }
    }
    #[inline]
    pub fn set_fielddesc_type(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        haspadding: u16,
        fielddesc_type: u16,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let haspadding: u16 = unsafe { ::std::mem::transmute(haspadding) };
            haspadding as u64
        });
        __bindgen_bitfield_unit.set(1usize, 2u8, {
            let fielddesc_type: u16 = unsafe { ::std::mem::transmute(fielddesc_type) };
            fielddesc_type as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jl_datatype_t {
    pub name: *mut jl_typename_t,
    pub super_: *mut _jl_datatype_t,
    pub parameters: *mut jl_svec_t,
    pub types: *mut jl_svec_t,
    pub names: *mut jl_svec_t,
    pub instance: *mut jl_value_t,
    pub layout: *const jl_datatype_layout_t,
    pub size: i32,
    pub ninitialized: i32,
    pub hash: u32,
    pub abstract_: u8,
    pub mutabl: u8,
    pub hasfreetypevars: u8,
    pub isconcretetype: u8,
    pub isdispatchtuple: u8,
    pub isbitstype: u8,
    pub zeroinit: u8,
    pub isinlinealloc: u8,
    pub has_concrete_subtype: u8,
    pub cached_by_hash: u8,
}
pub type jl_datatype_t = _jl_datatype_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_weakref_t {
    pub value: *mut jl_value_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_uuid_t {
    pub hi: u64,
    pub lo: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jl_module_t {
    pub name: *mut jl_sym_t,
    pub parent: *mut _jl_module_t,
    pub bindings: htable_t,
    pub usings: arraylist_t,
    pub build_id: u64,
    pub uuid: jl_uuid_t,
    pub primary_world: usize,
    pub counter: u32,
    pub nospecialize: i32,
    pub optlevel: i8,
    pub compile: i8,
    pub infer: i8,
    pub istopmod: u8,
    pub lock: jl_mutex_t,
}
pub type jl_module_t = _jl_module_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _jl_typemap_entry_t {
    pub next: *mut _jl_typemap_entry_t,
    pub sig: *mut jl_tupletype_t,
    pub simplesig: *mut jl_tupletype_t,
    pub guardsigs: *mut jl_svec_t,
    pub min_world: usize,
    pub max_world: usize,
    pub func: _jl_typemap_entry_t__bindgen_ty_1,
    pub isleafsig: i8,
    pub issimplesig: i8,
    pub va: i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _jl_typemap_entry_t__bindgen_ty_1 {
    pub value: *mut jl_value_t,
    pub linfo: *mut jl_method_instance_t,
    pub method: *mut jl_method_t,
}
pub type jl_typemap_entry_t = _jl_typemap_entry_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jl_typemap_level_t {
    pub arg1: *mut jl_array_t,
    pub targ: *mut jl_array_t,
    pub name1: *mut jl_array_t,
    pub tname: *mut jl_array_t,
    pub linear: *mut jl_typemap_entry_t,
    pub any: *mut jl_typemap_t,
}
pub type jl_typemap_level_t = _jl_typemap_level_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jl_methtable_t {
    pub name: *mut jl_sym_t,
    pub defs: *mut jl_typemap_t,
    pub leafcache: *mut jl_array_t,
    pub cache: *mut jl_typemap_t,
    pub max_args: isize,
    pub kwsorter: *mut jl_value_t,
    pub module: *mut jl_module_t,
    pub backedges: *mut jl_array_t,
    pub writelock: jl_mutex_t,
    pub offs: u8,
    pub frozen: u8,
}
pub type jl_methtable_t = _jl_methtable_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_expr_t {
    pub head: *mut jl_sym_t,
    pub args: *mut jl_array_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_method_match_t {
    pub spec_types: *mut jl_tupletype_t,
    pub sparams: *mut jl_svec_t,
    pub method: *mut jl_method_t,
    pub fully_covers: u8,
}
extern "C" {
    pub static mut jl_typeofbottom_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_datatype_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_uniontype_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_unionall_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_tvar_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_any_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_type_type: *mut jl_unionall_t;
}
extern "C" {
    pub static mut jl_typename_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_type_typename: *mut jl_typename_t;
}
extern "C" {
    pub static mut jl_symbol_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_ssavalue_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_abstractslot_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_slotnumber_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_typedslot_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_argument_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_const_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_partial_struct_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_method_match_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_simplevector_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_tuple_typename: *mut jl_typename_t;
}
extern "C" {
    pub static mut jl_vecelement_typename: *mut jl_typename_t;
}
extern "C" {
    pub static mut jl_anytuple_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_emptytuple_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_anytuple_type_type: *mut jl_unionall_t;
}
extern "C" {
    pub static mut jl_vararg_type: *mut jl_unionall_t;
}
extern "C" {
    pub static mut jl_vararg_typename: *mut jl_typename_t;
}
extern "C" {
    pub static mut jl_function_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_builtin_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_bottom_type: *mut jl_value_t;
}
extern "C" {
    pub static mut jl_method_instance_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_code_instance_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_code_info_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_method_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_module_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_abstractarray_type: *mut jl_unionall_t;
}
extern "C" {
    pub static mut jl_densearray_type: *mut jl_unionall_t;
}
extern "C" {
    pub static mut jl_array_type: *mut jl_unionall_t;
}
extern "C" {
    pub static mut jl_array_typename: *mut jl_typename_t;
}
extern "C" {
    pub static mut jl_weakref_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_abstractstring_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_string_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_errorexception_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_argumenterror_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_loaderror_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_initerror_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_typeerror_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_methoderror_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_undefvarerror_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_lineinfonode_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_stackovf_exception: *mut jl_value_t;
}
extern "C" {
    pub static mut jl_memory_exception: *mut jl_value_t;
}
extern "C" {
    pub static mut jl_readonlymemory_exception: *mut jl_value_t;
}
extern "C" {
    pub static mut jl_diverror_exception: *mut jl_value_t;
}
extern "C" {
    pub static mut jl_undefref_exception: *mut jl_value_t;
}
extern "C" {
    pub static mut jl_interrupt_exception: *mut jl_value_t;
}
extern "C" {
    pub static mut jl_boundserror_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_an_empty_vec_any: *mut jl_value_t;
}
extern "C" {
    pub static mut jl_an_empty_string: *mut jl_value_t;
}
extern "C" {
    pub static mut jl_bool_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_char_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_int8_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_uint8_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_int16_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_uint16_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_int32_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_uint32_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_int64_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_uint64_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_float16_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_float32_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_float64_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_floatingpoint_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_number_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_nothing_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_signed_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_voidpointer_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_pointer_type: *mut jl_unionall_t;
}
extern "C" {
    pub static mut jl_llvmpointer_type: *mut jl_unionall_t;
}
extern "C" {
    pub static mut jl_ref_type: *mut jl_unionall_t;
}
extern "C" {
    pub static mut jl_pointer_typename: *mut jl_typename_t;
}
extern "C" {
    pub static mut jl_llvmpointer_typename: *mut jl_typename_t;
}
extern "C" {
    pub static mut jl_namedtuple_typename: *mut jl_typename_t;
}
extern "C" {
    pub static mut jl_namedtuple_type: *mut jl_unionall_t;
}
extern "C" {
    pub static mut jl_task_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_array_uint8_type: *mut jl_value_t;
}
extern "C" {
    pub static mut jl_array_any_type: *mut jl_value_t;
}
extern "C" {
    pub static mut jl_array_symbol_type: *mut jl_value_t;
}
extern "C" {
    pub static mut jl_array_int32_type: *mut jl_value_t;
}
extern "C" {
    pub static mut jl_expr_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_globalref_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_linenumbernode_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_gotonode_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_gotoifnot_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_returnnode_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_phinode_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_pinode_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_phicnode_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_upsilonnode_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_quotenode_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_newvarnode_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_intrinsic_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_methtable_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_typemap_level_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_typemap_entry_type: *mut jl_datatype_t;
}
extern "C" {
    pub static mut jl_emptysvec: *mut jl_svec_t;
}
extern "C" {
    pub static mut jl_emptytuple: *mut jl_value_t;
}
extern "C" {
    pub static mut jl_true: *mut jl_value_t;
}
extern "C" {
    pub static mut jl_false: *mut jl_value_t;
}
extern "C" {
    pub static mut jl_nothing: *mut jl_value_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jl_gcframe_t {
    pub nroots: usize,
    pub prev: *mut _jl_gcframe_t,
}
pub type jl_gcframe_t = _jl_gcframe_t;
extern "C" {
    pub fn jl_gc_enable(on: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jl_gc_is_enabled() -> ::std::os::raw::c_int;
}
pub const jl_gc_collection_t_JL_GC_AUTO: jl_gc_collection_t = 0;
pub const jl_gc_collection_t_JL_GC_FULL: jl_gc_collection_t = 1;
pub const jl_gc_collection_t_JL_GC_INCREMENTAL: jl_gc_collection_t = 2;
pub type jl_gc_collection_t = ::std::os::raw::c_uint;
extern "C" {
    pub fn jl_gc_collect(arg1: jl_gc_collection_t);
}
extern "C" {
    pub fn jl_gc_add_finalizer(v: *mut jl_value_t, f: *mut jl_function_t);
}
extern "C" {
    pub fn jl_gc_add_ptr_finalizer(
        ptls: jl_ptls_t,
        v: *mut jl_value_t,
        f: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn jl_gc_queue_root(root: *mut jl_value_t);
}
extern "C" {
    pub fn jl_array_typetagdata(a: *mut jl_array_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn jl_compute_fieldtypes(
        st: *mut jl_datatype_t,
        stack: *mut ::std::os::raw::c_void,
    ) -> *mut jl_svec_t;
}
extern "C" {
    pub fn jl_subtype(a: *mut jl_value_t, b: *mut jl_value_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jl_egal(a: *mut jl_value_t, b: *mut jl_value_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jl_object_id(v: *mut jl_value_t) -> usize;
}
extern "C" {
    pub fn jl_isa(a: *mut jl_value_t, t: *mut jl_value_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jl_type_union(ts: *mut *mut jl_value_t, n: usize) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_type_unionall(v: *mut jl_tvar_t, body: *mut jl_value_t) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_typename_str(v: *mut jl_value_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn jl_typeof_str(v: *mut jl_value_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn jl_new_typevar(
        name: *mut jl_sym_t,
        lb: *mut jl_value_t,
        ub: *mut jl_value_t,
    ) -> *mut jl_tvar_t;
}
extern "C" {
    pub fn jl_apply_type(
        tc: *mut jl_value_t,
        params: *mut *mut jl_value_t,
        n: usize,
    ) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_apply_tuple_type_v(p: *mut *mut jl_value_t, np: usize) -> *mut jl_tupletype_t;
}
extern "C" {
    pub fn jl_new_structv(
        type_: *mut jl_datatype_t,
        args: *mut *mut jl_value_t,
        na: u32,
    ) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_new_struct_uninit(type_: *mut jl_datatype_t) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_alloc_svec(n: usize) -> *mut jl_svec_t;
}
extern "C" {
    pub fn jl_alloc_svec_uninit(n: usize) -> *mut jl_svec_t;
}
extern "C" {
    pub fn jl_symbol(str_: *const ::std::os::raw::c_char) -> *mut jl_sym_t;
}
extern "C" {
    pub fn jl_symbol_n(str_: *const ::std::os::raw::c_char, len: usize) -> *mut jl_sym_t;
}
extern "C" {
    pub fn jl_get_kwsorter(ty: *mut jl_value_t) -> *mut jl_function_t;
}
extern "C" {
    pub fn jl_box_bool(x: i8) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_box_int8(x: i8) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_box_uint8(x: u8) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_box_int16(x: i16) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_box_uint16(x: u16) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_box_int32(x: i32) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_box_uint32(x: u32) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_box_char(x: u32) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_box_int64(x: i64) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_box_uint64(x: u64) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_box_float32(x: f32) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_box_float64(x: f64) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_box_voidpointer(x: *mut ::std::os::raw::c_void) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_unbox_int8(v: *mut jl_value_t) -> i8;
}
extern "C" {
    pub fn jl_unbox_uint8(v: *mut jl_value_t) -> u8;
}
extern "C" {
    pub fn jl_unbox_int16(v: *mut jl_value_t) -> i16;
}
extern "C" {
    pub fn jl_unbox_uint16(v: *mut jl_value_t) -> u16;
}
extern "C" {
    pub fn jl_unbox_int32(v: *mut jl_value_t) -> i32;
}
extern "C" {
    pub fn jl_unbox_uint32(v: *mut jl_value_t) -> u32;
}
extern "C" {
    pub fn jl_unbox_int64(v: *mut jl_value_t) -> i64;
}
extern "C" {
    pub fn jl_unbox_uint64(v: *mut jl_value_t) -> u64;
}
extern "C" {
    pub fn jl_unbox_float32(v: *mut jl_value_t) -> f32;
}
extern "C" {
    pub fn jl_unbox_float64(v: *mut jl_value_t) -> f64;
}
extern "C" {
    pub fn jl_unbox_voidpointer(v: *mut jl_value_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn jl_field_index(
        t: *mut jl_datatype_t,
        fld: *mut jl_sym_t,
        err: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jl_get_nth_field(v: *mut jl_value_t, i: usize) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_get_nth_field_noalloc(v: *mut jl_value_t, i: usize) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_set_nth_field(v: *mut jl_value_t, i: usize, rhs: *mut jl_value_t);
}
extern "C" {
    pub fn jl_islayout_inline(
        eltype: *mut jl_value_t,
        fsz: *mut usize,
        al: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jl_new_array(atype: *mut jl_value_t, dims: *mut jl_value_t) -> *mut jl_array_t;
}
extern "C" {
    pub fn jl_reshape_array(
        atype: *mut jl_value_t,
        data: *mut jl_array_t,
        dims: *mut jl_value_t,
    ) -> *mut jl_array_t;
}
extern "C" {
    pub fn jl_ptr_to_array_1d(
        atype: *mut jl_value_t,
        data: *mut ::std::os::raw::c_void,
        nel: usize,
        own_buffer: ::std::os::raw::c_int,
    ) -> *mut jl_array_t;
}
extern "C" {
    pub fn jl_ptr_to_array(
        atype: *mut jl_value_t,
        data: *mut ::std::os::raw::c_void,
        dims: *mut jl_value_t,
        own_buffer: ::std::os::raw::c_int,
    ) -> *mut jl_array_t;
}
extern "C" {
    pub fn jl_alloc_array_1d(atype: *mut jl_value_t, nr: usize) -> *mut jl_array_t;
}
extern "C" {
    pub fn jl_alloc_array_2d(atype: *mut jl_value_t, nr: usize, nc: usize) -> *mut jl_array_t;
}
extern "C" {
    pub fn jl_alloc_array_3d(
        atype: *mut jl_value_t,
        nr: usize,
        nc: usize,
        z: usize,
    ) -> *mut jl_array_t;
}
extern "C" {
    pub fn jl_pchar_to_array(str_: *const ::std::os::raw::c_char, len: usize) -> *mut jl_array_t;
}
extern "C" {
    pub fn jl_pchar_to_string(str_: *const ::std::os::raw::c_char, len: usize) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_arrayref(a: *mut jl_array_t, i: usize) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_arrayset(a: *mut jl_array_t, v: *mut jl_value_t, i: usize);
}
extern "C" {
    pub fn jl_array_grow_end(a: *mut jl_array_t, inc: usize);
}
extern "C" {
    pub fn jl_array_del_end(a: *mut jl_array_t, dec: usize);
}
extern "C" {
    pub fn jl_array_grow_beg(a: *mut jl_array_t, inc: usize);
}
extern "C" {
    pub fn jl_array_del_beg(a: *mut jl_array_t, dec: usize);
}
extern "C" {
    pub fn jl_apply_array_type(type_: *mut jl_value_t, dim: usize) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_array_eltype(a: *mut jl_value_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub static mut jl_main_module: *mut jl_module_t;
}
extern "C" {
    pub static mut jl_core_module: *mut jl_module_t;
}
extern "C" {
    pub static mut jl_base_module: *mut jl_module_t;
}
extern "C" {
    pub fn jl_get_global(m: *mut jl_module_t, var: *mut jl_sym_t) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_set_global(m: *mut jl_module_t, var: *mut jl_sym_t, val: *mut jl_value_t);
}
extern "C" {
    pub fn jl_set_const(m: *mut jl_module_t, var: *mut jl_sym_t, val: *mut jl_value_t);
}
extern "C" {
    pub fn jl_is_imported(m: *mut jl_module_t, s: *mut jl_sym_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jl_cpu_threads() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jl_getpagesize() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn jl_getallocationgranularity() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn jl_is_debugbuild() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jl_get_UNAME() -> *mut jl_sym_t;
}
extern "C" {
    pub fn jl_get_ARCH() -> *mut jl_sym_t;
}
extern "C" {
    pub fn jl_get_libllvm() -> *mut jl_value_t;
}
extern "C" {
    pub static mut jl_n_threads: ::std::os::raw::c_int;
}
extern "C" {
    pub fn jl_environ(i: ::std::os::raw::c_int) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_exception_occurred() -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_init__threading();
}
extern "C" {
    pub fn jl_init_with_image__threading(
        julia_bindir: *const ::std::os::raw::c_char,
        image_relative_path: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn jl_is_initialized() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jl_atexit_hook(status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn jl_eval_string(str_: *const ::std::os::raw::c_char) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_apply_generic(
        F: *mut jl_value_t,
        args: *mut *mut jl_value_t,
        nargs: u32,
    ) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_call(
        f: *mut jl_function_t,
        args: *mut *mut jl_value_t,
        nargs: i32,
    ) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_call0(f: *mut jl_function_t) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_call1(f: *mut jl_function_t, a: *mut jl_value_t) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_call2(
        f: *mut jl_function_t,
        a: *mut jl_value_t,
        b: *mut jl_value_t,
    ) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_call3(
        f: *mut jl_function_t,
        a: *mut jl_value_t,
        b: *mut jl_value_t,
        c: *mut jl_value_t,
    ) -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_yield();
}
pub type jl_timing_block_t = _jl_timing_block_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jl_excstack_t {
    _unused: [u8; 0],
}
pub type jl_excstack_t = _jl_excstack_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jl_handler_t {
    pub eh_ctx: sigjmp_buf,
    pub gcstack: *mut jl_gcframe_t,
    pub prev: *mut _jl_handler_t,
    pub gc_state: i8,
    pub locks_len: usize,
    pub defer_signal: sig_atomic_t,
    pub timing_stack: *mut jl_timing_block_t,
    pub world_age: usize,
}
pub type jl_handler_t = _jl_handler_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jl_task_t {
    pub next: *mut jl_value_t,
    pub queue: *mut jl_value_t,
    pub tls: *mut jl_value_t,
    pub donenotify: *mut jl_value_t,
    pub result: *mut jl_value_t,
    pub logstate: *mut jl_value_t,
    pub start: *mut jl_function_t,
    pub _state: u8,
    pub sticky: u8,
    pub _isexception: u8,
    pub tid: i16,
    pub prio: i16,
    pub excstack: *mut jl_excstack_t,
    pub eh: *mut jl_handler_t,
    pub ctx: jl_ucontext_t,
    pub stkbuf: *mut ::std::os::raw::c_void,
    pub bufsz: usize,
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub gcstack: *mut jl_gcframe_t,
}
impl _jl_task_t {
    #[inline]
    pub fn copy_stack(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 31u8) as u32) }
    }
    #[inline]
    pub fn set_copy_stack(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 31u8, val as u64)
        }
    }
    #[inline]
    pub fn started(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_started(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        copy_stack: ::std::os::raw::c_uint,
        started: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 31u8, {
            let copy_stack: u32 = unsafe { ::std::mem::transmute(copy_stack) };
            copy_stack as u64
        });
        __bindgen_bitfield_unit.set(31usize, 1u8, {
            let started: u32 = unsafe { ::std::mem::transmute(started) };
            started as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type jl_task_t = _jl_task_t;
extern "C" {
    pub fn jl_process_events() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jl_stdout_obj() -> *mut jl_value_t;
}
extern "C" {
    pub fn jl_stderr_obj() -> *mut jl_value_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jl_options_t {
    pub quiet: i8,
    pub banner: i8,
    pub julia_bindir: *const ::std::os::raw::c_char,
    pub julia_bin: *const ::std::os::raw::c_char,
    pub cmds: *mut *const ::std::os::raw::c_char,
    pub image_file: *const ::std::os::raw::c_char,
    pub cpu_target: *const ::std::os::raw::c_char,
    pub nthreads: i32,
    pub nprocs: i32,
    pub machine_file: *const ::std::os::raw::c_char,
    pub project: *const ::std::os::raw::c_char,
    pub isinteractive: i8,
    pub color: i8,
    pub historyfile: i8,
    pub startupfile: i8,
    pub compile_enabled: i8,
    pub code_coverage: i8,
    pub malloc_log: i8,
    pub opt_level: i8,
    pub debug_level: i8,
    pub check_bounds: i8,
    pub depwarn: i8,
    pub warn_overwrite: i8,
    pub can_inline: i8,
    pub polly: i8,
    pub trace_compile: *const ::std::os::raw::c_char,
    pub fast_math: i8,
    pub worker: i8,
    pub cookie: *const ::std::os::raw::c_char,
    pub handle_signals: i8,
    pub use_sysimage_native_code: i8,
    pub use_compiled_modules: i8,
    pub bindto: *const ::std::os::raw::c_char,
    pub outputbc: *const ::std::os::raw::c_char,
    pub outputunoptbc: *const ::std::os::raw::c_char,
    pub outputo: *const ::std::os::raw::c_char,
    pub outputasm: *const ::std::os::raw::c_char,
    pub outputji: *const ::std::os::raw::c_char,
    pub output_code_coverage: *const ::std::os::raw::c_char,
    pub incremental: i8,
    pub image_file_specified: i8,
    pub warn_scope: i8,
    pub image_codegen: i8,
    pub rr_detach: i8,
}
extern "C" {
    pub static mut jl_options: jl_options_t;
}
extern "C" {
    pub fn jl_ver_major() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jl_ver_minor() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jl_ver_patch() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jl_ver_is_release() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jl_ver_string() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn jl_git_branch() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn jl_git_commit() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn jl_get_current_task() -> *mut jl_value_t;
}
pub type jl_markfunc_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: jl_ptls_t, obj: *mut jl_value_t) -> usize>;
pub type jl_sweepfunc_t = ::std::option::Option<unsafe extern "C" fn(obj: *mut jl_value_t)>;
extern "C" {
    pub fn jl_new_foreign_type(
        name: *mut jl_sym_t,
        module: *mut jl_module_t,
        super_: *mut jl_datatype_t,
        markfunc: jl_markfunc_t,
        sweepfunc: jl_sweepfunc_t,
        haspointers: ::std::os::raw::c_int,
        large: ::std::os::raw::c_int,
    ) -> *mut jl_datatype_t;
}
extern "C" {
    pub fn jl_gc_alloc_typed(
        ptls: jl_ptls_t,
        sz: usize,
        ty: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn jl_gc_mark_queue_obj(ptls: jl_ptls_t, obj: *mut jl_value_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jl_gc_mark_queue_objarray(
        ptls: jl_ptls_t,
        parent: *mut jl_value_t,
        objs: *mut *mut jl_value_t,
        nobjs: usize,
    );
}
extern "C" {
    pub fn jl_gc_schedule_foreign_sweepfunc(ptls: jl_ptls_t, bj: *mut jl_value_t);
}
pub const jlrs_catch_tag_t_JLRS_CATCH_OK: jlrs_catch_tag_t = 0;
pub const jlrs_catch_tag_t_JLRS_CATCH_ERR: jlrs_catch_tag_t = 1;
pub const jlrs_catch_tag_t_JLRS_CATCH_EXCECPTION: jlrs_catch_tag_t = 2;
pub const jlrs_catch_tag_t_JLRS_CATCH_PANIC: jlrs_catch_tag_t = 3;
pub type jlrs_catch_tag_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jlrs_catch_t {
    pub tag: jlrs_catch_tag_t,
    pub error: *mut ::std::os::raw::c_void,
}
pub type jlrs_callback_caller_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_void,
        arg2: *mut ::std::os::raw::c_void,
        arg3: *mut ::std::os::raw::c_void,
    ) -> jlrs_catch_t,
>;
extern "C" {
    pub fn jlrs_catch_wrapper(
        callback: *mut ::std::os::raw::c_void,
        caller: jlrs_callback_caller_t,
        result: *mut ::std::os::raw::c_void,
        frame_slice: *mut ::std::os::raw::c_void,
    ) -> jlrs_catch_t;
}
extern "C" {
    pub fn jlrs_array_data_owner_offset(n_dims: u16) -> uint_t;
}
extern "C" {
    pub fn jlrs_lock(v: *mut jl_value_t);
}
extern "C" {
    pub fn jlrs_unlock(v: *mut jl_value_t);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mallocarray_t {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bigval_t {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jl_timing_block_t {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jl_value_t {
    pub _address: u8,
}