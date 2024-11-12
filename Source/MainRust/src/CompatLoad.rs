use crate::{main, stop};
use std::os::raw::c_void;
use std::ptr::{null_mut};
use clroxide::clr::Clr;
use clroxide::primitives::{_Type, wrap_i64_in_variant, wrap_method_arguments, wrap_unknown_ptr_in_variant};
use derivative::Derivative;

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


#[no_mangle]
pub unsafe extern "C" fn RunMod(ptr: i64)
{
    let mut clr = Clr::context_only(None).unwrap();
    let context = clr.get_context().unwrap();
    let app_domain = context.app_domain;
    let mscorlib =  (*app_domain).load_library("mscorlib").unwrap();

    let int_ptr = (*mscorlib).get_type("IntPtr").unwrap();
    let gc_handle = (*mscorlib).get_type("System.Runtime.InteropServices.GCHandle").unwrap();

    let int_ptr_call = (*int_ptr).get_method("IntPtr").unwrap();
    let gc_handle_from_int_ptr = (*gc_handle).get_method("FromIntPtr").unwrap();

    let _int_ptr = (*int_ptr_call).invoke(wrap_method_arguments(vec![wrap_i64_in_variant(ptr)]).unwrap(), None).unwrap();

    let mut temp_interface = (*gc_handle_from_int_ptr).invoke(wrap_method_arguments(vec![_int_ptr]).unwrap(), None).unwrap();
    MOD_INTERFACE = &mut temp_interface as *mut _ as *mut _Type;
    let property_info = (*MOD_INTERFACE).get_property("name").unwrap() ;
    MOD_LOGGER = &mut (*property_info).get_value(Some(temp_interface)).unwrap() as *mut _ as *mut _Type;

    main();
}

#[no_mangle]
pub extern "C" fn StopMod()
{

    stop();
}