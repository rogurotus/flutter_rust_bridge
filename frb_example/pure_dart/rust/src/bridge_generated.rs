#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

use crate::api::*;
use flutter_rust_bridge::*;

// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_simple_adder(port: i64, a: i32, b: i32) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "simple_adder",
            port,
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_a = a.wire2api();
            let api_b = b.wire2api();
            move |task_callback| simple_adder(api_a, api_b)
        },
    );
}

#[no_mangle]
pub extern "C" fn wire_primitive_types(
    port: i64,
    my_i32: i32,
    my_i64: i64,
    my_f64: f64,
    my_bool: bool,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "primitive_types",
            port,
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_my_i32 = my_i32.wire2api();
            let api_my_i64 = my_i64.wire2api();
            let api_my_f64 = my_f64.wire2api();
            let api_my_bool = my_bool.wire2api();
            move |task_callback| primitive_types(api_my_i32, api_my_i64, api_my_f64, api_my_bool)
        },
    );
}

#[no_mangle]
pub extern "C" fn wire_handle_string(port: i64, s: *mut wire_uint_8_list) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "handle_string",
            port,
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_s = s.wire2api();
            move |task_callback| handle_string(api_s)
        },
    );
}

#[no_mangle]
pub extern "C" fn wire_handle_vec_u8(port: i64, v: *mut wire_uint_8_list) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "handle_vec_u8",
            port,
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_v = v.wire2api();
            move |task_callback| handle_vec_u8(api_v)
        },
    );
}

#[no_mangle]
pub extern "C" fn wire_handle_vec_of_primitive(port: i64, n: i32) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "handle_vec_of_primitive",
            port,
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_n = n.wire2api();
            move |task_callback| handle_vec_of_primitive(api_n)
        },
    );
}

#[no_mangle]
pub extern "C" fn wire_handle_zero_copy_vec_of_primitive(port: i64, n: i32) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "handle_zero_copy_vec_of_primitive",
            port,
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_n = n.wire2api();
            move |task_callback| handle_zero_copy_vec_of_primitive(api_n)
        },
    );
}

#[no_mangle]
pub extern "C" fn wire_handle_struct(port: i64, arg: *mut wire_MySize, boxed: *mut wire_MySize) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "handle_struct",
            port,
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_arg = arg.wire2api();
            let api_boxed = boxed.wire2api();
            move |task_callback| handle_struct(api_arg, api_boxed)
        },
    );
}

#[no_mangle]
pub extern "C" fn wire_handle_newtype(port: i64, arg: *mut wire_NewTypeInt) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "handle_newtype",
            port,
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_arg = arg.wire2api();
            move |task_callback| handle_newtype(api_arg)
        },
    );
}

#[no_mangle]
pub extern "C" fn wire_handle_list_of_struct(port: i64, l: *mut wire_list_my_size) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "handle_list_of_struct",
            port,
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_l = l.wire2api();
            move |task_callback| handle_list_of_struct(api_l)
        },
    );
}

#[no_mangle]
pub extern "C" fn wire_handle_complex_struct(port: i64, s: *mut wire_MyTreeNode) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "handle_complex_struct",
            port,
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_s = s.wire2api();
            move |task_callback| handle_complex_struct(api_s)
        },
    );
}

#[no_mangle]
pub extern "C" fn wire_handle_sync_return(
    port: i64,
    mode: *mut wire_uint_8_list,
) -> *mut wire_uint_8_list {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "handle_sync_return",
            port,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_mode = mode.wire2api();
            let ret = handle_sync_return(api_mode);
            TODO
        },
    );
}

#[no_mangle]
pub extern "C" fn wire_handle_stream(port: i64, arg: *mut wire_uint_8_list) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "handle_stream",
            port,
            mode: FfiCallMode::Stream,
        },
        move || {
            let api_arg = arg.wire2api();
            move |task_callback| handle_stream(task_callback.stream_sink(), api_arg)
        },
    );
}

#[no_mangle]
pub extern "C" fn wire_return_err(port: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "return_err",
            port,
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| return_err(),
    );
}

#[no_mangle]
pub extern "C" fn wire_return_panic(port: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "return_panic",
            port,
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| return_panic(),
    );
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_return(port: i64, left: f64, right: f64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "handle_optional_return",
            port,
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_left = left.wire2api();
            let api_right = right.wire2api();
            move |task_callback| handle_optional_return(api_left, api_right)
        },
    );
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_struct(port: i64, document: *mut wire_uint_8_list) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "handle_optional_struct",
            port,
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_document = document.wire2api();
            move |task_callback| handle_optional_struct(api_document)
        },
    );
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_increment(port: i64, opt: *mut wire_ExoticOptionals) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "handle_optional_increment",
            port,
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_opt = opt.wire2api();
            move |task_callback| handle_optional_increment(api_opt)
        },
    );
}

#[no_mangle]
pub extern "C" fn wire_handle_increment_boxed_optional(port: i64, opt: *mut f64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "handle_increment_boxed_optional",
            port,
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_opt = opt.wire2api();
            move |task_callback| handle_increment_boxed_optional(api_opt)
        },
    );
}

#[no_mangle]
pub extern "C" fn wire_handle_option_box_arguments(
    port: i64,
    i8box: *mut i8,
    u8box: *mut u8,
    i32box: *mut i32,
    i64box: *mut i64,
    f64box: *mut f64,
    boolbox: *mut bool,
    structbox: *mut wire_ExoticOptionals,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "handle_option_box_arguments",
            port,
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_i8box = i8box.wire2api();
            let api_u8box = u8box.wire2api();
            let api_i32box = i32box.wire2api();
            let api_i64box = i64box.wire2api();
            let api_f64box = f64box.wire2api();
            let api_boolbox = boolbox.wire2api();
            let api_structbox = structbox.wire2api();
            move |task_callback| {
                handle_option_box_arguments(
                    api_i8box,
                    api_u8box,
                    api_i32box,
                    api_i64box,
                    api_f64box,
                    api_boolbox,
                    api_structbox,
                )
            }
        },
    );
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_Attribute {
    key: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ExoticOptionals {
    int32: *mut i32,
    int64: *mut i64,
    float64: *mut f64,
    boolean: *mut bool,
    zerocopy: *mut wire_uint_8_list,
    int8list: *mut wire_int_8_list,
    uint8list: *mut wire_uint_8_list,
    int32list: *mut wire_int_32_list,
    int64list: *mut wire_int_64_list,
    float32list: *mut wire_float_32_list,
    float64list: *mut wire_float_64_list,
    attributes: *mut wire_list_attribute,
    attributes_nullable: *mut wire_list_opt_box_autoadd_attribute,
    nullable_attributes: *mut wire_list_opt_box_autoadd_attribute,
    newtypeint: *mut wire_NewTypeInt,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_float_32_list {
    ptr: *mut f32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_float_64_list {
    ptr: *mut f64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_int_32_list {
    ptr: *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_int_64_list {
    ptr: *mut i64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_int_8_list {
    ptr: *mut i8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_attribute {
    ptr: *mut wire_Attribute,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_my_size {
    ptr: *mut wire_MySize,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_my_tree_node {
    ptr: *mut wire_MyTreeNode,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_opt_box_autoadd_attribute {
    ptr: *mut *mut wire_Attribute,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MySize {
    width: i32,
    height: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MyTreeNode {
    value_i32: i32,
    value_vec_u8: *mut wire_uint_8_list,
    children: *mut wire_list_my_tree_node,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_NewTypeInt {
    field0: i64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_attribute() -> *mut wire_Attribute {
    support::new_leak_box_ptr(wire_Attribute::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_bool(value: bool) -> *mut bool {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_exotic_optionals() -> *mut wire_ExoticOptionals {
    support::new_leak_box_ptr(wire_ExoticOptionals::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_f64(value: f64) -> *mut f64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i32(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i64(value: i64) -> *mut i64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_size() -> *mut wire_MySize {
    support::new_leak_box_ptr(wire_MySize::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_tree_node() -> *mut wire_MyTreeNode {
    support::new_leak_box_ptr(wire_MyTreeNode::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_new_type_int() -> *mut wire_NewTypeInt {
    support::new_leak_box_ptr(wire_NewTypeInt::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_bool(value: bool) -> *mut bool {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_exotic_optionals() -> *mut wire_ExoticOptionals {
    support::new_leak_box_ptr(wire_ExoticOptionals::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_f64(value: f64) -> *mut f64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_i32(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_i64(value: i64) -> *mut i64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_i8(value: i8) -> *mut i8 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_my_size() -> *mut wire_MySize {
    support::new_leak_box_ptr(wire_MySize::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_u8(value: u8) -> *mut u8 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_float_32_list(len: i32) -> *mut wire_float_32_list {
    let ans = wire_float_32_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_float_64_list(len: i32) -> *mut wire_float_64_list {
    let ans = wire_float_64_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_int_32_list(len: i32) -> *mut wire_int_32_list {
    let ans = wire_int_32_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_int_64_list(len: i32) -> *mut wire_int_64_list {
    let ans = wire_int_64_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_int_8_list(len: i32) -> *mut wire_int_8_list {
    let ans = wire_int_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_attribute(len: i32) -> *mut wire_list_attribute {
    let wrap = wire_list_attribute {
        ptr: support::new_leak_vec_ptr(<wire_Attribute>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_my_size(len: i32) -> *mut wire_list_my_size {
    let wrap = wire_list_my_size {
        ptr: support::new_leak_vec_ptr(<wire_MySize>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_my_tree_node(len: i32) -> *mut wire_list_my_tree_node {
    let wrap = wire_list_my_tree_node {
        ptr: support::new_leak_vec_ptr(<wire_MyTreeNode>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_opt_box_autoadd_attribute(
    len: i32,
) -> *mut wire_list_opt_box_autoadd_attribute {
    let wrap = wire_list_opt_box_autoadd_attribute {
        ptr: support::new_leak_vec_ptr(<*mut wire_Attribute>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        if self.is_null() {
            None
        } else {
            Some(self.wire2api())
        }
    }
}

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<ZeroCopyBuffer<Vec<u8>>> for *mut wire_uint_8_list {
    fn wire2api(self) -> ZeroCopyBuffer<Vec<u8>> {
        ZeroCopyBuffer(self.wire2api())
    }
}

impl Wire2Api<Attribute> for wire_Attribute {
    fn wire2api(self) -> Attribute {
        Attribute {
            key: self.key.wire2api(),
            value: self.value.wire2api(),
        }
    }
}

impl Wire2Api<bool> for bool {
    fn wire2api(self) -> bool {
        self
    }
}

impl Wire2Api<Attribute> for *mut wire_Attribute {
    fn wire2api(self) -> Attribute {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<bool> for *mut bool {
    fn wire2api(self) -> bool {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<ExoticOptionals> for *mut wire_ExoticOptionals {
    fn wire2api(self) -> ExoticOptionals {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<f64> for *mut f64 {
    fn wire2api(self) -> f64 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<i32> for *mut i32 {
    fn wire2api(self) -> i32 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<i64> for *mut i64 {
    fn wire2api(self) -> i64 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<MySize> for *mut wire_MySize {
    fn wire2api(self) -> MySize {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<MyTreeNode> for *mut wire_MyTreeNode {
    fn wire2api(self) -> MyTreeNode {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<NewTypeInt> for *mut wire_NewTypeInt {
    fn wire2api(self) -> NewTypeInt {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<Box<bool>> for *mut bool {
    fn wire2api(self) -> Box<bool> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<Box<ExoticOptionals>> for *mut wire_ExoticOptionals {
    fn wire2api(self) -> Box<ExoticOptionals> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<Box<f64>> for *mut f64 {
    fn wire2api(self) -> Box<f64> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<Box<i32>> for *mut i32 {
    fn wire2api(self) -> Box<i32> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<Box<i64>> for *mut i64 {
    fn wire2api(self) -> Box<i64> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<Box<i8>> for *mut i8 {
    fn wire2api(self) -> Box<i8> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<Box<MySize>> for *mut wire_MySize {
    fn wire2api(self) -> Box<MySize> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<Box<u8>> for *mut u8 {
    fn wire2api(self) -> Box<u8> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<ExoticOptionals> for wire_ExoticOptionals {
    fn wire2api(self) -> ExoticOptionals {
        ExoticOptionals {
            int32: self.int32.wire2api(),
            int64: self.int64.wire2api(),
            float64: self.float64.wire2api(),
            boolean: self.boolean.wire2api(),
            zerocopy: self.zerocopy.wire2api(),
            int8list: self.int8list.wire2api(),
            uint8list: self.uint8list.wire2api(),
            int32list: self.int32list.wire2api(),
            int64list: self.int64list.wire2api(),
            float32list: self.float32list.wire2api(),
            float64list: self.float64list.wire2api(),
            attributes: self.attributes.wire2api(),
            attributes_nullable: self.attributes_nullable.wire2api(),
            nullable_attributes: self.nullable_attributes.wire2api(),
            newtypeint: self.newtypeint.wire2api(),
        }
    }
}

impl Wire2Api<f32> for f32 {
    fn wire2api(self) -> f32 {
        self
    }
}

impl Wire2Api<f64> for f64 {
    fn wire2api(self) -> f64 {
        self
    }
}

impl Wire2Api<Vec<f32>> for *mut wire_float_32_list {
    fn wire2api(self) -> Vec<f32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

impl Wire2Api<Vec<f64>> for *mut wire_float_64_list {
    fn wire2api(self) -> Vec<f64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}

impl Wire2Api<i64> for i64 {
    fn wire2api(self) -> i64 {
        self
    }
}

impl Wire2Api<i8> for i8 {
    fn wire2api(self) -> i8 {
        self
    }
}

impl Wire2Api<Vec<i32>> for *mut wire_int_32_list {
    fn wire2api(self) -> Vec<i32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

impl Wire2Api<Vec<i64>> for *mut wire_int_64_list {
    fn wire2api(self) -> Vec<i64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

impl Wire2Api<Vec<i8>> for *mut wire_int_8_list {
    fn wire2api(self) -> Vec<i8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

impl Wire2Api<Vec<Attribute>> for *mut wire_list_attribute {
    fn wire2api(self) -> Vec<Attribute> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<Vec<MySize>> for *mut wire_list_my_size {
    fn wire2api(self) -> Vec<MySize> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<Vec<MyTreeNode>> for *mut wire_list_my_tree_node {
    fn wire2api(self) -> Vec<MyTreeNode> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<Vec<Option<Attribute>>> for *mut wire_list_opt_box_autoadd_attribute {
    fn wire2api(self) -> Vec<Option<Attribute>> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<MySize> for wire_MySize {
    fn wire2api(self) -> MySize {
        MySize {
            width: self.width.wire2api(),
            height: self.height.wire2api(),
        }
    }
}

impl Wire2Api<MyTreeNode> for wire_MyTreeNode {
    fn wire2api(self) -> MyTreeNode {
        MyTreeNode {
            value_i32: self.value_i32.wire2api(),
            value_vec_u8: self.value_vec_u8.wire2api(),
            children: self.children.wire2api(),
        }
    }
}

impl Wire2Api<NewTypeInt> for wire_NewTypeInt {
    fn wire2api(self) -> NewTypeInt {
        NewTypeInt(self.field0.wire2api())
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_Attribute {
    fn new_with_null_ptr() -> Self {
        Self {
            key: std::ptr::null_mut(),
            value: std::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_ExoticOptionals {
    fn new_with_null_ptr() -> Self {
        Self {
            int32: std::ptr::null_mut(),
            int64: std::ptr::null_mut(),
            float64: std::ptr::null_mut(),
            boolean: std::ptr::null_mut(),
            zerocopy: std::ptr::null_mut(),
            int8list: std::ptr::null_mut(),
            uint8list: std::ptr::null_mut(),
            int32list: std::ptr::null_mut(),
            int64list: std::ptr::null_mut(),
            float32list: std::ptr::null_mut(),
            float64list: std::ptr::null_mut(),
            attributes: std::ptr::null_mut(),
            attributes_nullable: std::ptr::null_mut(),
            nullable_attributes: std::ptr::null_mut(),
            newtypeint: std::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_MySize {
    fn new_with_null_ptr() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_MyTreeNode {
    fn new_with_null_ptr() -> Self {
        Self {
            value_i32: Default::default(),
            value_vec_u8: std::ptr::null_mut(),
            children: std::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_NewTypeInt {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}

// Section: impl IntoDart

impl support::IntoDart for Attribute {
    fn into_dart(self) -> support::DartCObject {
        vec![self.key.into_dart(), self.value.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Attribute {}

impl support::IntoDart for Element {
    fn into_dart(self) -> support::DartCObject {
        vec![
            self.tag.into_dart(),
            self.text.into_dart(),
            self.attributes.into_dart(),
            self.children.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Element {}

impl support::IntoDart for ExoticOptionals {
    fn into_dart(self) -> support::DartCObject {
        vec![
            self.int32.into_dart(),
            self.int64.into_dart(),
            self.float64.into_dart(),
            self.boolean.into_dart(),
            self.zerocopy.into_dart(),
            self.int8list.into_dart(),
            self.uint8list.into_dart(),
            self.int32list.into_dart(),
            self.int64list.into_dart(),
            self.float32list.into_dart(),
            self.float64list.into_dart(),
            self.attributes.into_dart(),
            self.attributes_nullable.into_dart(),
            self.nullable_attributes.into_dart(),
            self.newtypeint.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for ExoticOptionals {}

impl support::IntoDart for MySize {
    fn into_dart(self) -> support::DartCObject {
        vec![self.width.into_dart(), self.height.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for MySize {}

impl support::IntoDart for MyTreeNode {
    fn into_dart(self) -> support::DartCObject {
        vec![
            self.value_i32.into_dart(),
            self.value_vec_u8.into_dart(),
            self.children.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for MyTreeNode {}

impl support::IntoDart for NewTypeInt {
    fn into_dart(self) -> support::DartCObject {
        vec![self.0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for NewTypeInt {}

impl support::IntoDart for VecOfPrimitivePack {
    fn into_dart(self) -> support::DartCObject {
        vec![
            self.int8list.into_dart(),
            self.uint8list.into_dart(),
            self.int16list.into_dart(),
            self.uint16list.into_dart(),
            self.uint32list.into_dart(),
            self.int32list.into_dart(),
            self.uint64list.into_dart(),
            self.int64list.into_dart(),
            self.float32list.into_dart(),
            self.float64list.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for VecOfPrimitivePack {}

impl support::IntoDart for ZeroCopyVecOfPrimitivePack {
    fn into_dart(self) -> support::DartCObject {
        vec![
            self.int8list.into_dart(),
            self.uint8list.into_dart(),
            self.int16list.into_dart(),
            self.uint16list.into_dart(),
            self.uint32list.into_dart(),
            self.int32list.into_dart(),
            self.uint64list.into_dart(),
            self.int64list.into_dart(),
            self.float32list.into_dart(),
            self.float64list.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for ZeroCopyVecOfPrimitivePack {}

// Section: executor
support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}
