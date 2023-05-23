use rand::prelude::*;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    let mut heap:[isize; 100000] = [0; 100000];  
    let heap_size:isize = heap.len() as isize;
    for i in 0..heap_size {
        heap[i as usize] = random::<isize>();
    }
    let mut heap_ordered:isize = 0;
    while heap_ordered<20 {
        let mut biggest = heap[0];
        let mut biggest_index = 0;
        for i in 0..heap_size-heap_ordered{
            let i:usize = i as usize;
            if heap[i] > biggest {
                biggest = heap[i];
                biggest_index = i;
            }
        }

        heap[biggest_index] = heap[0];
        let last:usize =(heap_size-(heap_ordered+1)) as usize;
        heap[0] = heap[last];
        heap[last] = biggest;
        heap_ordered = heap_ordered + 1;
        
        
        
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
