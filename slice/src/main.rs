#![allow(unused)]
#![feature(vec_into_raw_parts)]
use libc::c_void;
use std::boxed::Box;
use std::ffi::CString;
use std::os::raw::c_int;
use std::rc::Rc;
use std::rc::Weak;
use std::string::String;
use std::sync::Arc;
use std::vec::Vec;
use std::mem::MaybeUninit;


use libc::AF_APPLETALK;
extern "C" {
  fn matrix_mul(
    a: *mut c_int,
    b: *mut c_int,
    a_r: c_int,
    a_c: c_int,
    b_r: c_int,
    b_c: c_int,
  ) -> *mut c_int;
  fn c_alloc(size: c_int) -> *mut c_int;
  fn c_out_of_bound();
  fn out_of_bound(a: *mut c_int);
  fn c_free(a: *mut c_int);
  fn allocate_uninitialized_array(size: c_int) -> *mut c_int;
}

#[repr(C)]
// Akin to `&[u8]`, for C.
pub struct ByteSliceView {
    pub ptr: *const u8,
    pub len: usize,
}
#[repr(C)]
struct BarC {
    x: ByteSliceView,
}

struct Bar{
  pub x: [u8; 2],
}

#[no_mangle]
unsafe extern "C" fn bar_parse(input: *const u8, input_len: usize, bar_c: &mut BarC) {
    let input: &[u8] = unsafe { std::slice::from_raw_parts(input, input_len) };

    let bar: Bar = Bar {
        x: [input[0], input[1]],
    };

    *bar_c = BarC {
        x: ByteSliceView {
            ptr: bar.x.as_ptr(),
            len: bar.x.len(),
        },
    };
}



fn rust_heap_overflow(){
  let xs = vec![Box::new(0), Box::new(1), Box::new(2), Box::new(3)];
  let _y = unsafe { &(*xs.as_ptr().offset(5)) };
  println!("{:?}", _y);
}

fn rust_stack_overflow(){
  let xs = [1, 2, 3, 4];
  let _y = unsafe { *xs.as_ptr().offset(5)};
  println!("{:?}", _y);
}

fn main() {
  // leakage 
  // let a = [1, 2, 3, 4, 5, 6];
  // let b = [1, 2, 3, 4, 5, 6, 7, 8, 9];
  // let a_ptr = &a as *const i32;
  // let b_ptr = &b as *const i32;
  // let c = unsafe { matrix_mul(a_ptr as *mut i32, b_ptr as *mut i32, 2, 3, 3, 3) };
  // let c_vec = unsafe { Vec::from_raw_parts(c as *mut i32, 6, 6) };
  // // let c_ref = unsafe{slice::from_raw_parts(c, 2 * 3)};
  // // println!("{:?}", c_vec);
  // let w = unsafe{ c_alloc(10) };
  // unsafe {
  //   println!("{:?}", *c);
  // }



  // out of bound 
  let mut a = vec![1];
  unsafe{ 
    // *const i32 can be cast into *mut i32 easily
    out_of_bound(a.as_ptr() as *mut i32); 
    // c_free(a.as_mut_ptr());
  } 
  unsafe{c_out_of_bound();}
  

  // pure Rust's overflow
  // rust_heap_overflow();
  // rust_stack_overflow();


  // stack-use-after-return
  // let mut bar_c = MaybeUninit::<BarC>::uninit();
  // let input = [10, 11, 12];
  // unsafe {
  //     bar_parse(
  //         input.as_ptr(),
  //         input.len(),
  //         bar_c.as_mut_ptr().as_mut().unwrap(),
  //     );
  // }

  // let bar_c = unsafe { bar_c.assume_init() };
  // let x: &[u8] = unsafe{ std::slice::from_raw_parts(bar_c.x.ptr, 2)};
  // // assert_eq!(x[0], 10);
  // println!("{:?}", x);

  // rust uninitialized read
  // unsafe {
  //   let a = MaybeUninit::<[usize; 4]>::uninit();
  //   let a = a.assume_init();
  //   println!("{}", a[2]);
  // }

  // rust uninitialized read C
  // let size = 5;
  // let array_ptr = unsafe { allocate_uninitialized_array(size) };

  // let slice = unsafe{ std::slice::from_raw_parts(array_ptr, size as usize)};

  // println!("Uninitialized array content:");
  // for &num in slice.iter() {
  //     println!("{}", num);  
  // }
  // free(array_ptr as *mut c_void);


}

#[test]
fn bar() {
    // This mimicks how C/C++ code would call our function.
    let mut bar_c = MaybeUninit::<BarC>::uninit();
    let input = [10, 11, 12];
    unsafe {
        bar_parse(
            input.as_ptr(),
            input.len(),
            bar_c.as_mut_ptr().as_mut().unwrap(),
        );
    }

    let bar_c = unsafe { bar_c.assume_init_ref() };
    let x: &[u8] = unsafe{ std::slice::from_raw_parts(bar_c.x.ptr, 2)};
    assert_eq!(x, [10, 11].as_slice());
}