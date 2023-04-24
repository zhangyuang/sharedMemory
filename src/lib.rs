#[macro_use]
extern crate napi_derive;


use std::collections::HashMap;
use std::fs::metadata;

use shared_memory::{Shmem, ShmemConf};

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


#[napi]
fn get_object(env: Env, mem_id: String) -> JsObject {
  let shmem = ShmemConf::new().flink(&mem_id).open().unwrap();
  unsafe {
   let ptr = shmem.as_ptr();
   let foo = JsObject::from_raw_unchecked(env.raw(), ptr as *mut napi_value__);
   foo
  }
}
use napi::{JsObject, Result, Env, NapiRaw, NapiValue, sys};
use sys::{napi_value__};

#[napi]
fn set_object(env:Env,mem_id: String, js_object: JsObject) {
  unsafe {
    let shmem = if metadata(&mem_id).is_ok() {
      let old_data = ShmemConf::new().size(0).flink(&mem_id).open().unwrap();
      std::ptr::write_bytes(old_data.as_ptr(), 0, old_data.len());
      old_data
    } else {
      ShmemConf::new().size(4096).flink(&mem_id).create().unwrap()
    };
    let foo = js_object.raw() as *mut u8;
    std::ptr::copy(foo, shmem.as_ptr(), 1000);
    if SHMEM.is_some() {
      let global_shmem = SHMEM.as_mut().unwrap();
      if global_shmem.get(&mem_id).is_none() {
        global_shmem.insert(mem_id, shmem);
      }
    }
  }
}
