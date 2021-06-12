use jlrs::util::JULIA;
use jlrs::{
    memory::gc::{Gc, GcCollection},
    prelude::*,
    wrappers::ptr::array::dimensions::Dims,
};

macro_rules! impl_test {
    ($name:ident, $name_mut:ident, $name_slice:ident, $name_slice_mut:ident, $value_type:ty) => {
        #[test]
        fn $name() {
            JULIA.with(|j| {
                let mut jlrs = j.borrow_mut();

                jlrs.scope_with_slots(5, |global, frame| unsafe {
                    let data: Vec<$value_type> = (1..=24).map(|x| x as $value_type).collect();

                    let array = Array::from_vec(&mut *frame, data, (2, 3, 4))?;
                    let d = array
                        .cast::<Array>()?
                        .inline_data::<$value_type, _>(&mut *frame)?;

                    let mut out = 1 as $value_type;
                    for third in &[0, 1, 2, 3] {
                        for second in &[0, 1, 2] {
                            for first in &[0, 1] {
                                assert_eq!(d[[*first, *second, *third]], out);
                                out += 1 as $value_type;
                            }
                        }
                    }

                    let gi = Module::base(global)
                        .function_ref("getindex")?
                        .wrapper_unchecked();
                    let one = Value::new(&mut *frame, 1usize)?;
                    let two = Value::new(&mut *frame, 2usize)?;
                    let three = Value::new(&mut *frame, 3usize)?;
                    let four = Value::new(&mut *frame, 4usize)?;

                    out = 1 as $value_type;
                    for third in &[one, two, three, four] {
                        for second in &[one, two, three] {
                            for first in &[one, two] {
                                frame.scope_with_slots(1, |frame| {
                                    let v = gi
                                        .call(&mut *frame, &mut [array, *first, *second, *third])?
                                        .unwrap();
                                    assert_eq!(v.unbox::<$value_type>()?, out);
                                    out += 1 as $value_type;
                                    Ok(())
                                })?;
                            }
                        }
                    }

                    Ok(())
                })
                .unwrap();

                unsafe {
                    jlrs.gc_collect(GcCollection::Full);
                }
            });
        }

        #[test]
        fn $name_mut() {
            JULIA.with(|j| {
                let mut jlrs = j.borrow_mut();

                jlrs.scope_with_slots(5, |global, frame| unsafe {
                    let data: Vec<$value_type> = (1..=24).map(|x| x as $value_type).collect();

                    let array = Array::from_vec(&mut *frame, data, (2, 3, 4))?;
                    let mut d = array
                        .cast::<Array>()?
                        .inline_data_mut::<$value_type, _>(&mut *frame)?;

                    for third in &[0, 1, 2, 3] {
                        for second in &[0, 1, 2] {
                            for first in &[0, 1] {
                                d[(*first, *second, *third)] += 1 as $value_type;
                            }
                        }
                    }
                    let gi = Module::base(global)
                        .function_ref("getindex")?
                        .wrapper_unchecked();
                    let one = Value::new(&mut *frame, 1usize)?;
                    let two = Value::new(&mut *frame, 2usize)?;
                    let three = Value::new(&mut *frame, 3usize)?;
                    let four = Value::new(&mut *frame, 4usize)?;

                    let mut out = 2 as $value_type;
                    for third in &[one, two, three, four] {
                        for second in &[one, two, three] {
                            for first in &[one, two] {
                                frame.scope_with_slots(1, |frame| {
                                    let v = gi
                                        .call(&mut *frame, &mut [array, *first, *second, *third])?
                                        .unwrap();
                                    assert_eq!(v.unbox::<$value_type>()?, out);
                                    out += 1 as $value_type;
                                    Ok(())
                                })?;
                            }
                        }
                    }

                    Ok(())
                })
                .unwrap();
            });
        }

        #[test]
        fn $name_slice() {
            JULIA.with(|j| {
                let mut jlrs = j.borrow_mut();

                jlrs.scope_with_slots(1, |_, frame| {
                    let data: Vec<$value_type> = (1..=24).map(|x| x as $value_type).collect();

                    let array = Array::from_vec(&mut *frame, data.clone(), (2, 3, 4))?;
                    let d = array
                        .cast::<Array>()?
                        .inline_data::<$value_type, _>(&mut *frame)?;

                    for (a, b) in data.iter().zip(d.as_slice()) {
                        assert_eq!(a, b)
                    }

                    Ok(())
                })
                .unwrap();
            });
        }

        #[test]
        fn $name_slice_mut() {
            JULIA.with(|j| {
                let mut jlrs = j.borrow_mut();

                jlrs.scope_with_slots(1, |_, frame| {
                    let data: Vec<$value_type> = (1..=24).map(|x| x as $value_type).collect();

                    let array = Array::from_vec(&mut *frame, data.clone(), (2, 3, 4))?;
                    let mut d = array
                        .cast::<Array>()?
                        .inline_data_mut::<$value_type, _>(&mut *frame)?;

                    for (a, b) in data.iter().zip(d.as_mut_slice()) {
                        assert_eq!(a, b)
                    }

                    Ok(())
                })
                .unwrap();
            });
        }
    };
}

impl_test!(
    array_data_3d_u8,
    array_data_3d_u8_mut,
    array_data_3d_u8_slice,
    array_data_3d_u8_mut_slice,
    u8
);
impl_test!(
    array_data_3d_u16,
    array_data_3d_u16_mut,
    array_data_3d_u16_slice,
    array_data_3d_u16_mut_slice,
    u16
);
impl_test!(
    array_data_3d_u32,
    array_data_3d_u32_mut,
    array_data_3d_u32_slice,
    array_data_3d_u32_mut_slice,
    u32
);
impl_test!(
    array_data_3d_u64,
    array_data_3d_u64_mut,
    array_data_3d_u64_slice,
    array_data_3d_u64_mut_slice,
    u64
);
impl_test!(
    array_data_3d_i8,
    array_data_3d_i8_mut,
    array_data_3d_i8_slice,
    array_data_3d_i8_mut_slice,
    i8
);
impl_test!(
    array_data_3d_i16,
    array_data_3d_i16_mut,
    array_data_3d_i16_slice,
    array_data_3d_i16_mut_slice,
    i16
);
impl_test!(
    array_data_3d_i32,
    array_data_3d_i32_mut,
    array_data_3d_i32_slice,
    array_data_3d_i32_mut_slice,
    i32
);
impl_test!(
    array_data_3d_i64,
    array_data_3d_i64_mut,
    array_data_3d_i64_slice,
    array_data_3d_i64_mut_slice,
    i64
);
impl_test!(
    array_data_3d_f32,
    array_data_3d_f32_mut,
    array_data_3d_f32_slice,
    array_data_3d_f32_mut_slice,
    f32
);
impl_test!(
    array_data_3d_f64,
    array_data_3d_f64_mut,
    array_data_3d_f64_slice,
    array_data_3d_f64_mut_slice,
    f64
);

#[test]
fn borrow_nested() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.scope_with_slots(1, |global, frame| unsafe {
            let data: Vec<u8> = (1..=24).map(|x| x as u8).collect();

            let array = Array::from_vec(&mut *frame, data, (2, 3, 4))?;

            frame.scope_with_slots(4, |frame| {
                let d = {
                    array
                        .cast_unchecked::<Array>()
                        .inline_data::<u8, _>(&mut *frame)?
                };

                let mut out = 1 as u8;
                for third in &[0, 1, 2, 3] {
                    for second in &[0, 1, 2] {
                        for first in &[0, 1] {
                            assert_eq!(d[(*first, *second, *third)], out);
                            out += 1 as u8;
                        }
                    }
                }

                let gi = Module::base(global)
                    .function_ref("getindex")?
                    .wrapper_unchecked();
                let one = Value::new(&mut *frame, 1usize)?;
                let two = Value::new(&mut *frame, 2usize)?;
                let three = Value::new(&mut *frame, 3usize)?;
                let four = Value::new(&mut *frame, 4usize)?;

                out = 1 as u8;
                for third in &[one, two, three, four] {
                    for second in &[one, two, three] {
                        for first in &[one, two] {
                            frame.scope_with_slots(1, |frame| {
                                let v = gi
                                    .call(&mut *frame, &mut [array, *first, *second, *third])?
                                    .unwrap();
                                assert_eq!(v.unbox::<u8>()?, out);
                                out += 1 as u8;
                                Ok(())
                            })?;
                        }
                    }
                }

                Ok(())
            })
        })
        .unwrap();
    });
}

#[test]
fn access_borrowed_array_dimensions() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.scope_with_slots(1, |_, frame| {
            let arr_val = Array::new::<f32, _, _, _>(&mut *frame, (1, 2))?;
            let arr = arr_val.cast::<Array>()?;

            let data = arr.inline_data::<f32, _>(&mut *frame)?;
            assert_eq!(data.dimensions().into_dimensions().as_slice(), &[1, 2]);

            Ok(())
        })
        .unwrap();
    })
}

#[test]
fn access_mutable_borrowed_array_dimensions() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.scope_with_slots(1, |_, frame| {
            let arr_val = Array::new::<f32, _, _, _>(&mut *frame, (1, 2))?;
            let arr = arr_val.cast::<Array>()?;

            let data = arr.inline_data_mut::<f32, _>(&mut *frame)?;
            assert_eq!(data.dimensions().into_dimensions().as_slice(), &[1, 2]);

            Ok(())
        })
        .unwrap();
    })
}

#[test]
fn unrestricted_array_borrow() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.scope_with_slots(2, |_, frame| {
            unsafe {
                let arr_val = Array::new::<f32, _, _, _>(&mut *frame, (1, 2))?;
                let arr_val2 = Array::new::<f32, _, _, _>(&mut *frame, (1, 2))?;
                let arr = arr_val.cast::<Array>()?;
                let arr2 = arr_val2.cast::<Array>()?;

                let data = arr.unrestricted_inline_data_mut::<f32, _>(&*frame)?;
                let data2 = arr2.unrestricted_inline_data_mut::<f32, _>(&*frame)?;
                assert_eq!(
                    data.dimensions().into_dimensions().as_slice(),
                    data2.dimensions().into_dimensions().as_slice()
                );
            }

            Ok(())
        })
        .unwrap();
    })
}

#[test]
fn unrestricted_typed_array_borrow() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.scope_with_slots(2, |_, frame| {
            unsafe {
                let arr_val = Array::new::<f32, _, _, _>(&mut *frame, (1, 2))?;
                let arr_val2 = Array::new::<f32, _, _, _>(&mut *frame, (1, 2))?;
                let arr = arr_val.cast::<TypedArray<f32>>()?;
                let arr2 = arr_val2.cast::<TypedArray<f32>>()?;

                let data = arr.unrestricted_inline_data_mut(&*frame)?;
                let data2 = arr2.unrestricted_inline_data_mut(&*frame)?;
                assert_eq!(
                    data.dimensions().into_dimensions().as_slice(),
                    data2.dimensions().into_dimensions().as_slice()
                );
            }

            Ok(())
        })
        .unwrap();
    })
}

#[test]
fn value_data() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.scope_with_slots(2, |global, frame| {
            unsafe {
                let arr = Module::main(global)
                    .submodule_ref("JlrsTests")?
                    .wrapper_unchecked()
                    .function_ref("vecofmodules")?
                    .wrapper_unchecked()
                    .call0(&mut *frame)?
                    .unwrap()
                    .cast::<Array>()?;
                let data = arr.value_data(&mut *frame)?;

                assert!(data[0].wrapper_unchecked().is::<Module>());
            }
            Ok(())
        })
        .unwrap();
    })
}

#[test]
fn value_data_mut() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.scope_with_slots(3, |global, frame| {
            unsafe {
                let submod = Module::main(global)
                    .submodule_ref("JlrsTests")?
                    .wrapper_unchecked();
                let arr = submod
                    .function_ref("vecofmodules")?
                    .wrapper_unchecked()
                    .call0(&mut *frame)?
                    .unwrap()
                    .cast::<Array>()?;
                let mut data = arr.value_data_mut(&mut *frame)?;
                data.set(0, submod.as_value().as_ref())?;

                let getindex = Module::base(global)
                    .function_ref("getindex")?
                    .wrapper_unchecked();
                let idx = Value::new(&mut *frame, 1usize)?;
                let entry = getindex
                    .call2(&mut *frame, arr.as_value(), idx)?
                    .unwrap()
                    .cast::<Module>()?;

                assert_eq!(entry.name().hash(), submod.name().hash());
            }
            Ok(())
        })
        .unwrap();
    })
}

#[test]
fn unrestricted_value_data_mut() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.scope_with_slots(6, |global, frame| {
            unsafe {
                let submod = Module::main(global)
                    .submodule_ref("JlrsTests")?
                    .wrapper_unchecked();
                let arr1 = submod
                    .function_ref("vecofmodules")?
                    .wrapper_unchecked()
                    .call0(&mut *frame)?
                    .unwrap()
                    .cast::<Array>()?;

                let arr2 = submod
                    .function_ref("anothervecofmodules")?
                    .wrapper_unchecked()
                    .call0(&mut *frame)?
                    .unwrap()
                    .cast::<Array>()?;

                let mut data1 = arr1.unrestricted_value_data_mut(&*frame)?;
                let mut data2 = arr2.unrestricted_value_data_mut(&*frame)?;
                data1.set(0, submod.as_value().as_ref())?;
                data2.set(1, submod.as_value().as_ref())?;

                let getindex = Module::base(global)
                    .function_ref("getindex")?
                    .wrapper_unchecked();
                let idx1 = Value::new(&mut *frame, 1usize)?;
                let idx2 = Value::new(&mut *frame, 2usize)?;
                let entry1 = getindex
                    .call2(&mut *frame, arr1.as_value(), idx1)?
                    .unwrap()
                    .cast::<Module>()?;
                let entry2 = getindex
                    .call2(&mut *frame, arr2.as_value(), idx2)?
                    .unwrap()
                    .cast::<Module>()?;

                assert_eq!(entry1.name().hash(), entry2.name().hash());
            }
            Ok(())
        })
        .unwrap();
    })
}

#[test]
fn typed_array_value_data() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.scope_with_slots(2, |global, frame| {
            unsafe {
                let arr = Module::main(global)
                    .submodule_ref("JlrsTests")?
                    .wrapper_unchecked()
                    .function_ref("vecofmodules")?
                    .wrapper_unchecked()
                    .call0(&mut *frame)?
                    .unwrap()
                    .cast::<TypedArray<Module>>()?;
                let data = arr.value_data(&mut *frame)?;

                assert!(data[0].wrapper_unchecked().is::<Module>());
            }
            Ok(())
        })
        .unwrap();
    })
}

#[test]
fn typed_array_value_data_mut() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.scope_with_slots(3, |global, frame| {
            unsafe {
                let submod = Module::main(global)
                    .submodule_ref("JlrsTests")?
                    .wrapper_unchecked();
                let arr = submod
                    .function_ref("vecofmodules")?
                    .wrapper_unchecked()
                    .call0(&mut *frame)?
                    .unwrap()
                    .cast::<TypedArray<Module>>()?;
                let mut data = arr.value_data_mut(&mut *frame)?;
                data.set(0, submod.as_value().as_ref())?;

                let getindex = Module::base(global)
                    .function_ref("getindex")?
                    .wrapper_unchecked();
                let idx = Value::new(&mut *frame, 1usize)?;
                let entry = getindex
                    .call2(&mut *frame, arr.as_value(), idx)?
                    .unwrap()
                    .cast::<Module>()?;

                assert_eq!(entry.name().hash(), submod.name().hash());
            }
            Ok(())
        })
        .unwrap();
    })
}

#[test]
fn typed_array_unrestricted_value_data_mut() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.scope_with_slots(6, |global, frame| {
            unsafe {
                let submod = Module::main(global)
                    .submodule_ref("JlrsTests")?
                    .wrapper_unchecked();
                let arr1 = submod
                    .function_ref("vecofmodules")?
                    .wrapper_unchecked()
                    .call0(&mut *frame)?
                    .unwrap()
                    .cast::<TypedArray<Module>>()?;

                let arr2 = submod
                    .function_ref("anothervecofmodules")?
                    .wrapper_unchecked()
                    .call0(&mut *frame)?
                    .unwrap()
                    .cast::<TypedArray<Module>>()?;

                let mut data1 = arr1.unrestricted_value_data_mut(&*frame)?;
                let mut data2 = arr2.unrestricted_value_data_mut(&*frame)?;
                data1.set(0, submod.as_value().as_ref())?;
                data2.set(1, submod.as_value().as_ref())?;

                let getindex = Module::base(global)
                    .function_ref("getindex")?
                    .wrapper_unchecked();
                let idx1 = Value::new(&mut *frame, 1usize)?;
                let idx2 = Value::new(&mut *frame, 2usize)?;
                let entry1 = getindex
                    .call2(&mut *frame, arr1.as_value(), idx1)?
                    .unwrap()
                    .cast::<Module>()?;
                let entry2 = getindex
                    .call2(&mut *frame, arr2.as_value(), idx2)?
                    .unwrap()
                    .cast::<Module>()?;

                assert_eq!(entry1.name().hash(), entry2.name().hash());
            }
            Ok(())
        })
        .unwrap();
    })
}
