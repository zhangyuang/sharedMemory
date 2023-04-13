#[macro_use]
extern crate napi_derive;
#[macro_use]
extern crate lazy_static;
use napi::sys::napi_value__;
use napi::{Env, JsNull, JsObject, NapiRaw, NapiValue, Result};
use std::collections::HashMap;
use std::mem::{self, transmute};
use std::sync::{Arc, Mutex};

use shared_memory::{Shmem, ShmemConf};

// #[napi]
// fn get_func<T: Fn(i32) -> Result<()>>(callback: T) {
//   unsafe {
//     let callback_ptr = SHMEM.as_ref().unwrap();
//     let callback_ptr = **callback_ptr;
//     let callback_ptr: &dyn Fn(i32) -> Result<()> = unsafe { &*(callback_ptr as *const T) };
//     callback_ptr(1);
//   }
// }

// #[napi]
// fn set_func<T: Fn(i32) -> Result<()>>(callback: T) {
//   unsafe {
//     let callback_ptr = &callback as *const T as *const u8;
//     let foo = Box::new(callback_ptr);
//     SHMEM = Some(foo);
//   }
// }

// #[napi]
// fn get_string(env: Env) -> String {
//   unsafe {
//     let test = SHMEMSTRING.take().unwrap();
//     let test = (*test).clone();
//     return test
//   }
// }

static mut SHMEM: Option<HashMap<String, Shmem>> = None;
#[napi]
fn get_string(mem_id: String) -> Result<String> {
  let shmem = ShmemConf::new().flink(&mem_id).open().unwrap();
  unsafe {
    let slice = std::slice::from_raw_parts(shmem.as_ptr(), 4096);
    let str = std::str::from_utf8_unchecked(slice).to_string();
    Ok(str)
  }
}

#[napi]
fn set_string(mem_id: String, callback: String) {
  unsafe {
    let shmem = ShmemConf::new().size(4096).flink(&mem_id).create().unwrap();
    let ptr = shmem.as_ptr();
    let len = std::cmp::min(callback.len(), shmem.len());
    std::ptr::copy(callback.as_ptr(), ptr, len);
    let global_shmem = SHMEM.as_mut().unwrap();
    global_shmem.insert(mem_id, shmem);
  }
}
#[napi]
fn init() {
  unsafe { SHMEM = Some(HashMap::new()) }
}
#[napi]
fn clear(mem_id: String) {
  unsafe {
    let global_shmem = SHMEM.as_mut().unwrap();
    global_shmem.remove(&mem_id);
  }
}

// static mut SHMEMOBJECT: Option<JsObject> = None;

// #[napi]
// fn get_object(env: Env) {
//   unsafe {
//     let object = SHMEMOBJECT.take().unwrap();
//     let ptr = &object;
//     println!("{:p}", ptr);
//     let bar = object.get::<&str, String>("foo").unwrap();
//     println!("{:?}", bar);
//   }
// }

// #[napi]
// fn set_object(env: Env, object: JsObject) {
//   unsafe {
//     let ptr = &object;
//     println!("{:p}", ptr);
//     SHMEMOBJECT = Some(object);
//   }
// }
