use std::alloc::{alloc, Layout, dealloc};
use std::time::Instant;
use std::fmt::Debug;
use std::ptr;
use std::sync::atomic::{fence, Ordering};

struct UnalignedBuffer<T> {
    ptr: *mut u8,
    layout: Layout,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> UnalignedBuffer<T> {
    fn new(elements: usize, offset: usize) -> Self {
        let size = std::mem::size_of::<T>() * elements + offset;
        let layout = Layout::from_size_align(size, 1)
            .expect("Invalid layout");
        let ptr = unsafe { alloc(layout) };
        Self {
            ptr,
            layout,
            _phantom: std::marker::PhantomData,
        }
    }

    fn get_ptr(&self, offset: usize) -> *mut T {
        unsafe {
            self.ptr.add(offset) as *mut T
        }
    }
}

impl<T> Drop for UnalignedBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr, self.layout);
        }
    }
}

fn run_test<T>(type_name: &str)
where
    T: Copy + Debug + 
       std::ops::Add<Output = T> + 
       std::ops::AddAssign +
       From<i32> + std::cmp::PartialEq,
{
    // Increased workload
    const N: usize = 10_000_000;
    const REPEAT: usize = 50;
    
    println!("\nProcessing {} ({} bytes)", type_name, std::mem::size_of::<T>());
    
    for offset in 0..std::mem::size_of::<T>() {
        let mut sum_time = 0f64;
        print!("offset {}: ", offset);
        
        let buffer = UnalignedBuffer::<T>::new(N, offset);
        let mut results = Vec::new();
        
        for _ in 0..REPEAT {
            let start = Instant::now();
            
            unsafe {
                let data = buffer.get_ptr(offset);
                
                // Write phase with memory fence
                for i in 0..N {
                    ptr::write(data.add(i), T::from(i as i32 % 100));
                    if i % 1000 == 0 { fence(Ordering::SeqCst); }
                }
                
                // Read phase with accumulation
                let mut sum = T::from(0);
                for i in 0..N {
                    sum += ptr::read(data.add(i));
                    if i % 1000 == 0 { fence(Ordering::SeqCst); }
                }
                
                results.push(sum);
            }
            
            // Use nanoseconds for more precision
            let elapsed = start.elapsed().as_nanos();
            sum_time += elapsed as f64;
        }
        
        // Convert back to milliseconds for display
        println!(" avg: {:.3}ms", (sum_time / REPEAT as f64) / 1_000_000.0);
        
        // Print a checksum to prevent optimization
        let checksum: T = results.iter().copied().fold(T::from(0), |acc, x| acc + x);
        print!(".");
        if checksum == T::from(0) { print!("!"); }
    }
}

fn main() {
    println!("Testing true unaligned memory access...");
    run_test::<i32>("i32");
    run_test::<i64>("i64");
    run_test::<i128>("i128");
}