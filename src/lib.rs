#[macro_use]
extern crate napi_derive;
#[macro_use]
extern crate lazy_static;
use napi::{Env, JsNull, JsObject, NapiRaw, NapiValue, Result};
use std::collections::HashMap;
use std::fs::metadata;
use std::mem::{self, transmute};

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
fn set_string(mem_id: String, js_string: String) {
  unsafe {
    let shmem = if metadata(&mem_id).is_ok() {
      let old_data = ShmemConf::new().size(0).flink(&mem_id).open().unwrap();
      println!("{}", old_data.len());
      std::ptr::write_bytes(old_data.as_ptr(), 0, old_data.len());
      old_data
    } else {
      ShmemConf::new().size(4096).flink(&mem_id).create().unwrap()
    };
    let ptr = shmem.as_ptr();
    std::ptr::copy(js_string.as_ptr(), ptr, js_string.len());
    if SHMEM.is_some() {
      let global_shmem = SHMEM.as_mut().unwrap();
      if global_shmem.get(&mem_id).is_none() {
        global_shmem.insert(mem_id, shmem);
      }
    }
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
