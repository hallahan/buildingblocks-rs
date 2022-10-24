use std::{fs::{File, OpenOptions}, mem::size_of};
use fast_hilbert::{h2xy};

type Error = Box<dyn std::error::Error>;

use flatbuffers::FlatBufferBuilder;
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
    let mut builder = FlatBufferBuilder::new();
    let mut point_table_builder = PointTableBuilder::new(&mut builder);
    // push h value on slotoff: 4
    // field_locs [{off: 8, id:4}] 64bits, slot offset 4
    point_table_builder.add_h(h);
    // field_locs [{off: 8, id:4}, {off: 12, id:6}] 32bits, slot offset 6
    point_table_builder.add_x(x);
    // field_locs [{off: 8, id:4}, {off: 12, id:6}, {off: 16, id:4}] 32bits, slot offset 6
    point_table_builder.add_y(y);
    // point_table_builder.finish calls 
    let point_table = point_table_builder.finish();
    // builder.finish allocates 24 more bytes to owned_buf, to a total of 64, with head being 24
    builder.finish(point_table, None);
    // this is just a slice where head is the start
    let data = builder.finished_data();
    // total                - 40 bytes
    // vtable_start_pos     - 16 bytes (usize)
    // vtable               -  8 bytes
    // data_len             -  4 bytes 
    // data                 - 16 bytes
    //
    // vtable_start_pos = size_of_u32 + size_prefix_offset + size_file_identifier
    // the 00, 00 is just padding
    // 20 bytes in is where data_len is. That is the negative offset for the vtable.
    // [vtable_start_position   __  __  00, 00, vt_len, objlen, hoffset,xoffset,yoffset,data_len__  __, y_  __  __  __, x_  __  __  __, h_  __  __  __  __  __  __  __]
    // [14, 00, 00, 00, 00, 00, 00, 00, 00, 00, 0A, 00, 14, 00, 0C, 00, 08, 00, 04, 00, 0A, 00, 00, 00, 03, 00, 00, 00, 03, 00, 00, 00, 0A, 00, 00, 00, 00, 00, 00, 00]
    // [20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 20, 0, 12, 0, 8, 0, 4, 0, 10, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0]
    println!("point_table {:02X?}", data);
    println!("point_table {:?}", data);



    let mut b = FlatBufferBuilder::new();

    // max num_items
    b.start_vector::<Point>(3);

    let (x, y) = h2xy::<u32>(10);
    let p0 = Point::new(h, x, y);

    let (x, y) = h2xy::<u32>(11);
    let p1 = Point::new(h, x, y);

    let (x, y) = h2xy::<u32>(12);
    let p2 = Point::new(h, x, y);

    b.push(p0);
    b.push(p1);
    b.push(p2);

    // num items actually written
    let point_vector = b.end_vector::<Point>(3);

    // at this point, all of the points are in memory

    let mut feat_b = FeatureBuilder::new(&mut b);
    feat_b.add_points(point_vector);
    let feature = feat_b.finish();


    b.finish(feature, None);
    let d = b.finished_data();
    println!("point_vector {:02X?}", d);
    println!("point_vector {:?}", d);


    Ok(())
}


fn main() {

    if let Err(e) = run() {
        eprintln!("{}: {}", "Error", e);
        std::process::exit(1);
    }
    
}

// table at this point is 16 bytes
// finish will write the vtable
// push 0xF0F0_F0F0

// 


// Comment in write_vtable
// Layout of the data this function will create when a new vtable is
// needed.
// --------------------------------------------------------------------
// vtable starts here
// | x, x -- vtable len (bytes) [u16]
// | x, x -- object inline len (bytes) [u16]
// | x, x -- zero, or num bytes from start of object to field #0   [u16]
// | ...
// | x, x -- zero, or num bytes from start of object to field #n-1 [u16]
// vtable ends here
// table starts here
// | x, x, x, x -- offset (negative direction) to the vtable [i32]
// |               aka "vtableoffset"
// | -- table inline data begins here, we don't touch it --
// table ends here -- aka "table_start"
// --------------------------------------------------------------------
//
// Layout of the data this function will create when we re-use an
// existing vtable.
//
// We always serialize this particular vtable, then compare it to the
// other vtables we know about to see if there is a duplicate. If there
// is, then we erase the serialized vtable we just made.
// We serialize it first so that we are able to do byte-by-byte
// comparisons with already-serialized vtables. This 1) saves
// bookkeeping space (we only keep revlocs to existing vtables), 2)
// allows us to convert to little-endian once, then do
// fast memcmp comparisons, and 3) by ensuring we are comparing real
// serialized vtables, we can be more assured that we are doing the
// comparisons correctly.
//
// --------------------------------------------------------------------
// table starts here
// | x, x, x, x -- offset (negative direction) to an existing vtable [i32]
// |               aka "vtableoffset"
// | -- table inline data begins here, we don't touch it --
// table starts here: aka "table_start"
// --------------------------------------------------------------------
