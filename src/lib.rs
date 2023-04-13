#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
#[macro_use]
extern crate lazy_static;

use napi::{CallContext, JsFunction, JsNull, JsObject, Result, JsUnknown};
use shared_memory::{Shmem, ShmemConf, ShmemError};
use std::sync::{Mutex, Arc};

struct Box(pub Option<Shmem>);
unsafe impl Sync for Box {}

// lazy_static! {
//   static ref SHARED_MEMORY: *const u8 = 0 as *const u8;

// }

static mut GLOBAL_PTR: *const &dyn Fn(String)->i32 = 0 as *const &dyn Fn(String)->i32;

// // static SHARED_MEMORY: Mutex<u8> = Mutex::new(0);

// #[napi]
// fn set_func<T: Fn(i32) -> Result<()>>(callback: T) {
//   let shmem = SHARED_MEMORY.lock().unwrap();
//   let shmem = *shmem as *mut u8;
//   let shmem_ptr = shmem as *mut &dyn Fn(i32) -> Result<()>;
//   unsafe { std::ptr::write(shmem_ptr, &callback) };
//   // unsafe {
//   //     let foo = *shmem_ptr;
//   //     let bar = foo(1); 
//   // }

// }

// #[napi]
// fn create_share_memory () {
//    let shmem = ShmemConf::new()
//     .size(4096)
//     .flink("/Users/yuuang/Desktop/github/share-memory/share")
//     .create()
//     .unwrap();
//   let ptr = shmem.as_ptr();
//   // 获取指针的大小
//   let ptr_size = std::mem::size_of_val(&ptr);

//   // 将指针地址存储在变量中
//   let mut ptr_addr: [u8; 8] = [0; 8];
//   let ptr_addr_ptr = &mut ptr_addr as *mut [u8; 8] as *mut u8;
//   unsafe {
//       std::ptr::copy_nonoverlapping(&ptr as *const _ as *const u8, ptr_addr_ptr, ptr_size);
//   }

// println!("{:?}", ptr_addr);
//   unsafe {
//     let ptr = *ptr;
//     let mut share_memory = SHARED_MEMORY.lock().unwrap();
//     // *share_memory = ptr as *const u8
//   }
// }
