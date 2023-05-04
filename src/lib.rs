#[macro_use]
extern crate napi_derive;

use std::fs::metadata;
use std::{collections::HashMap, mem, mem::transmute};

use napi::JsString;
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
use arrayref::array_ref;

#[napi]
fn get_object(env: Env, mem_id: String) -> JsObject {
  let shmem = ShmemConf::new().flink(&mem_id).open().unwrap();
  let js_object_layout = std::alloc::Layout::new::<JsObject>();
  unsafe {
    let js_object_ptr = std::alloc::alloc(js_object_layout) as *mut JsObject;
    let ptr = shmem.as_ptr();
    let js_object_bytes = std::slice::from_raw_parts(ptr, js_object_layout.size());
    std::ptr::copy_nonoverlapping(
      js_object_bytes.as_ptr(),
      js_object_ptr as *mut u8,
      js_object_layout.size(),
    );
    let js_object = std::ptr::read(js_object_ptr);
    std::alloc::dealloc(js_object_ptr as *mut u8, js_object_layout);
    js_object
  }
}
use napi::{sys::napi_value, Env, JsObject, Result};

#[napi]
fn set_object(env: Env, mem_id: String, js_object: JsObject) {
  unsafe {
    let shmem = if metadata(&mem_id).is_ok() {
      let old_data = ShmemConf::new().size(0).flink(&mem_id).open().unwrap();
      std::ptr::write_bytes(old_data.as_ptr(), 0, old_data.len());
      old_data
    } else {
      ShmemConf::new().size(4096).flink(&mem_id).create().unwrap()
    };
    let property_names: JsObject = js_object.get_property_names().unwrap();
    let mut property_names_array: Vec<JsString> = Vec::new();
    for i in 0..property_names.get_array_length().unwrap() {
        let property_name: JsString = property_names.get_element(i).unwrap();
        println!("{:?}", property_name);
        let bar = property_name.into_utf8().unwrap();
        property_names_array.push(property_name);
    }
  }
}
