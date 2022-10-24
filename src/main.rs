use std::{fs::{File, OpenOptions}, mem::size_of};
use fast_hilbert::{h2xy};

type Error = Box<dyn std::error::Error>;

use memmap2::{Mmap, MmapMut};

#[allow(dead_code, unused_imports)]
#[path = "./example_generated.rs"]
pub mod example;
use example::*;

// const GB1: u64 = 1_073_741_824_u64;

fn run() -> Result<(), Error> {

    // Make a point in memory.
    let h = 10_u64;
    let (x, y) = h2xy::<u32>(h);
    let point = Point::new(h, x, y);
    let point_len = point.0.len() as u64;
    println!("point len {}", point_len);
    println!("size_of::<Point>() {}", size_of::<Point>());
    println!("point {:?}", point);
    println!("point as bytes {:02X?}", point.0);


    // Make a memmap buffer to fit 100 points.
    let num_points = 100_u64;
    let file = OpenOptions::new().read(true).write(true).create(true).open("data/fb_points")?;
    file.set_len(num_points * size_of::<Point>() as u64)?;
    let mut mmap = unsafe { MmapMut::map_mut(&file)? };


    // Set the 13th item to the point
    let slc = &mut mmap[..];
    unsafe {
        let points = ::core::slice::from_raw_parts_mut( slc.as_ptr() as *mut Point, num_points as usize);
        println!("points len {}", points.len());
        points[12] = point;
    };

    // fill up the entire buffer
    let slc = &mut mmap[..];
    unsafe {
        let points = ::core::slice::from_raw_parts_mut( slc.as_ptr() as *mut Point, num_points as usize);
        for i in 0..num_points {
            let (x, y) = h2xy::<u32>(i);
            let p = Point::new(i, x, y);
            points[i as usize] = p;
        }
    };

    // Make a simple point table

    Ok(())
}


fn main() {

    if let Err(e) = run() {
        eprintln!("{}: {}", "Error", e);
        std::process::exit(1);
    }
    
}
