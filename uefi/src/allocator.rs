use core::alloc::{GlobalAlloc, Layout};

use crate::system_table_opt;

pub struct UefiPoolAllocator;

unsafe impl GlobalAlloc for UefiPoolAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let system_table = if let Some(x) = system_table_opt() {
            x
        } else {
            return core::ptr::null_mut();
        };
        let boot_services = system_table.boot_services();
        let alloc_result = boot_services.allocate_pool(layout.size());
        alloc_result.unwrap_or(core::ptr::null_mut())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let system_table = if let Some(x) = system_table_opt() {
            x
        } else {
            return;
        };
        let boot_services = system_table.boot_services();
        boot_services.free_pool(ptr).ok(); // ignore deallocation error
    }
}
