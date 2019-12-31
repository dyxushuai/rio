#![allow(non_camel_case_types)]

/// Generated by bindgen, then cleaned up
/// Target: linux 5.3.9_p3-debian-sources
///         include/uapi/linux/io_uring.h
use std::fmt;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_cqe {
    pub user_data: u64,
    pub res: i32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct io_uring_params {
    pub sq_entries: u32,
    pub cq_entries: u32,
    pub flags: u32,
    pub sq_thread_cpu: u32,
    pub sq_thread_idle: u32,
    pub resv: [u32; 5usize],
    pub sq_off: io_sqring_offsets,
    pub cq_off: io_cqring_offsets,
}

pub type __kernel_rwf_t = ::std::os::raw::c_int;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct io_uring_sqe {
    pub opcode: u8,
    pub flags: u8,
    pub ioprio: u16,
    pub fd: i32,
    pub off: u64,
    pub addr: u64,
    pub len: u32,
    pub __bindgen_anon_1: io_uring_sqe__bindgen_ty_1,
    pub user_data: u64,
    pub __bindgen_anon_2: io_uring_sqe__bindgen_ty_2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_1 {
    pub rw_flags: __kernel_rwf_t,
    pub fsync_flags: u32,
    pub poll_events: u16,
    pub sync_range_flags: u32,
    pub msg_flags: u32,
    _bindgen_union_align: u32,
}

impl Default for io_uring_sqe__bindgen_ty_1 {
    fn default() -> io_uring_sqe__bindgen_ty_1 {
        unsafe { std::mem::zeroed() }
    }
}

impl fmt::Debug for io_uring_sqe__bindgen_ty_1 {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "io_uring_sqe__bindgen_ty_1")
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_2 {
    pub buf_index: u16,
    pub __pad2: [u64; 3usize],
    _bindgen_union_align: [u64; 3usize],
}

impl fmt::Debug for io_uring_sqe__bindgen_ty_2 {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "io_uring_sqe__bindgen_ty_2")
    }
}

impl Default for io_uring_sqe__bindgen_ty_2 {
    fn default() -> io_uring_sqe__bindgen_ty_2 {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct io_sqring_offsets {
    pub head: u32,
    pub tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
    pub flags: u32,
    pub dropped: u32,
    pub array: u32,
    pub resv1: u32,
    pub resv2: u64,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct io_cqring_offsets {
    pub head: u32,
    pub tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
    pub overflow: u32,
    pub cqes: u32,
    pub resv: [u64; 2usize],
}
