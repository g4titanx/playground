//! Data Alignment Performance Testing
//! 
//! This program measures the performance impact of memory alignment on different
//! integer types. It performs read and write operations on vectors with different
//! memory alignments and measures the execution time.
//! 
//! usage:
//! ```bash
//! cargo run --release
//! ```
use std::fmt::Debug;
use std::time::Instant;


/// Runs alignment performance tests for a given numeric type.
/// 
/// # Type Parameters
/// 
/// * `T` - The numeric type to test. Must implement necessary traits for:
///   - Copying (`Copy`)
///   - Debug printing (`Debug`)
///   - Basic arithmetic (`Mul`, `Add`, `AddAssign`)
///   - Conversion from i32 (`From<i32>`)
/// 
/// # Arguments
/// 
/// * `type_name` - Name of the type being tested, used for output labeling
/// 
/// # Test Methodology
/// 
/// 1. For each possible alignment offset (0 to size_of::<T>):
///    - Creates a vector with the specified offset
///    - Performs REPEAT iterations of:
///      a. Writing N sequential numbers
///      b. Reading and performing arithmetic operations
///    - Measures and reports average execution time
/// 
/// # Memory Layout
/// 
/// ```text
/// [padding bytes (offset)] [actual data (N elements)]
/// ```
/// 
/// The padding affects the alignment of the actual data section.
fn run_test<T>(_type_name: &str)
where
    T: Copy
        + Debug
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::AddAssign
        + From<i32>,
{
    const N: usize = 10_000_000;   // Number of elements to process
    const REPEAT: usize = 20;       // Number of test iterations

    println!("Processing word of size {}", std::mem::size_of::<T>());

    for offset in 0..std::mem::size_of::<T>() {
        println!("offset = {offset}");

        let mut sum_time = 0f64;
        println!("ignore this: ");

        // Create vec with extra space for offset
        let mut base_vec = Vec::with_capacity(N + offset + 1);

        // Fill with dummy data to ensure proper offset
        base_vec.resize(offset, T::from(0));

        // Now our actual data will start at the offset
        base_vec.reserve(N);

        // Run multiple iterations to get stable timing
        for _ in 0..REPEAT {
            let start = Instant::now();

            // Reset to padding only
            base_vec.truncate(offset);

            // Write phase - convert index to i32 before converting to T
            for i in 0..N {
                base_vec.push(T::from(i as i32));
            }

            // Read and compute phase
            // Uses a multiplicative hash combined with addition to ensure
            // both operations are tested and results aren't optimized away
            let mut val = T::from(1);
            for i in offset..N + offset {
                val += base_vec[i] * val + T::from(33);
            }

            let elapsed = start.elapsed().as_millis();
            sum_time += elapsed as f64;
            print!("{val:?}");
        }

        println!();
        println!(
            " average time for offset {} is {:.1}",
            offset % std::mem::size_of::<T>(),
            sum_time / REPEAT as f64
        );
    }
    println!();
}

fn main() {
    println!("Running alignment tests...\n");
    run_test::<i32>("i32");
    run_test::<i64>("i64");
    run_test::<i128>("i128");
}
