use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{Ordering::SeqCst, AtomicUsize};

/*struct NullAllocator;

unsafe impl GlobalAlloc for NullAllocator {
    unsafe fn alloc(&self, _lt: Layout) -> *mut u8 {
        panic!("you wish!");
        std::ptr::null_mut()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _lt: Layout) {
        panic!("never allocated");
    }
}
*/
static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

struct Counter;
unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null()
        {
            ALLOCATED.fetch_add(layout.size(), SeqCst);
        }

        return ret;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), SeqCst);
    }
}

#[global_allocator]
static A: Counter = Counter;

fn main() {
    println!("Allocated bytes before main: {}", ALLOCATED.load(SeqCst));
}
