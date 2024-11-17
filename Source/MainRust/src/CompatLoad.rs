use crate::{main, stop};
use std::fmt::Debug;
use std::result;
use std::io::{Write};
use std::os::raw::c_void;
use std::ptr::{null_mut};
use clroxide::clr::Clr;
use clroxide::primitives::{_Type, wrap_i64_in_variant, wrap_method_arguments, wrap_unknown_ptr_in_variant};
use derivative::Derivative;
use std::fs::File;

static mut MOD_INTERFACE : *mut _Type = null_mut();
static mut MOD_LOGGER : *mut _Type = null_mut();

#[derive(Derivative, Clone, Copy)]
#[derivative(Debug, Default)]
#[repr(C)]
pub struct Config ;

pub unsafe fn ReadConfig<>(mut conf : *mut Config) -> Config {
    let read = (*MOD_INTERFACE).get_method("ReadConfig").unwrap();
    return *(&mut (*read).invoke(wrap_method_arguments(vec![wrap_unknown_ptr_in_variant(conf as *mut _ as  *mut c_void)]).unwrap(), None).unwrap() as *mut _ as *mut Config);
}

pub unsafe fn debugging<T>(mut file: &File, input : result::Result<T, String>) -> T {
    let _ = file.write(b"Testing\n");
    if input.is_ok() {
        return input.unwrap();
    }
    let _ = file.write( input.as_ref().unwrap_err_unchecked().as_bytes());

    return input.unwrap();
}


#[no_mangle]
pub unsafe extern "C" fn RunMod(ptr: i64)
{
    let mut file = File::create("LOG.txt").unwrap();

    let mut clr = debugging(&file ,Clr::context_only(None));
    let context = debugging(&file ,clr.get_context());
    let app_domain = context.app_domain;
    let mscorlib =  debugging(&file ,(*app_domain).load_library("mscorlib"));

    let int_ptr = debugging(&file ,(*mscorlib).get_type("System.IntPtr"));
    let gc_handle = debugging(&file ,(*mscorlib).get_type("System.Runtime.InteropServices.GCHandle"));
    let int_ptr_call = debugging(&file ,(*int_ptr).get_constructor_with_signature("Void .ctor(Int64)"));
    let gc_handle_from_int_ptr = debugging(&file ,(*gc_handle).get_method_with_signature("System.Runtime.InteropServices.GCHandle FromIntPtr(IntPtr)"));
    let _int_ptr = debugging(&file ,(*int_ptr_call).invoke(wrap_method_arguments(vec![wrap_i64_in_variant(ptr)]).unwrap()));
    let gc_handle_instance = debugging(&file,(*mscorlib).create_instance("System.Runtime.InteropServices.GCHandle"));
    
    for x in (*gc_handle).get_methods().unwrap() {
        let _ = file.write(((*x).to_string().unwrap() + "\n").as_bytes());
    }
    
    let mut temp_interface = debugging(&file ,(*gc_handle_from_int_ptr).invoke(wrap_method_arguments(vec![_int_ptr]).unwrap(), Some(gc_handle_instance.clone())));

    /*
    MOD_INTERFACE = &mut temp_interface as *mut _ as *mut _Type;
    let property_info = debugging(&file ,(*MOD_INTERFACE).get_property("name"));
    MOD_LOGGER = &mut debugging(&file ,(*property_info).get_value(Some(temp_interface))) as *mut _ as *mut _Type;

    // Call our main code.
    main();
    */
}

#[no_mangle]
pub extern "C" fn StopMod()
{
    stop();
}