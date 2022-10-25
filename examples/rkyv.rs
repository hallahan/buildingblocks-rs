type Error = Box<dyn std::error::Error>;

use std::{pin::Pin, fs::OpenOptions, io::Write, collections::BTreeMap};

use memmap2::MmapMut;
use rkyv::{AlignedBytes, Archive, Deserialize, Serialize, collections::ArchivedBTreeMap, ser::serializers::{AllocSerializer, BufferSerializer}};
// bytecheck can be used to validate your data if you want
// use bytecheck::CheckBytes;

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
// This will generate a PartialEq impl between our unarchived and archived types
#[archive(compare(PartialEq))]
// To use the safe API, you have to derive CheckBytes for the archived type
#[archive_attr(derive(Debug))]
struct Test {
    int: u8,
    string: String,
    option: Option<Vec<i32>>,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug))]
struct Point {
    h: u64,
    x: u32,
    y: u32
}

// type MyVec<'a> = Vec<(&'a u16, &'a u16)>;

// impl ExactSizeIterator for MyVec<'a> {
//     fn len(&self) -> usize {
//         self.len()
//     }
// }


fn run() -> Result<(), Error> {
    
    let value = Test {
        int: 42,
        string: "hello world".to_string(),
        option: Some(vec![1, 2, 3, 4]),
    };

    // Serializing is as easy as a single function call
    let bytes = rkyv::to_bytes::<_, 256>(&value).unwrap();

    // Or you can use the unsafe API for maximum performance
    let archived = unsafe { rkyv::archived_root::<Test>(&bytes[..]) };
    assert_eq!(archived, &value);

    println!("archived {:?}", archived);

    // Lets see if we can mutate...
    let mut aligned_vec = rkyv::to_bytes::<_, 256>(&value)?;
    let bytes_mut = aligned_vec.as_mut_slice();
    let pin = Pin::<&mut [u8]>::new(bytes_mut);
    let archived2 = unsafe { rkyv::archived_root_mut::<Test>(pin) };
    unsafe {
        let a = archived2.get_unchecked_mut();
        a.int = 255;
        println!("mutate {:?}", a);
    }

    // Mutate a memmap buffer
    let file = OpenOptions::new().read(true).write(true).create(true).open("data/rkyv_points")?;
    file.set_len(1600_u64)?;
    let mut mmap = unsafe { MmapMut::map_mut(&file)? };
    let slc: &mut [u8] = &mut mmap[..];
    let p = Pin::<&mut [u8]>::new(slc);
    let a = unsafe { rkyv::archived_root_mut::<Point>(p) };
    unsafe {
        let ap = a.get_unchecked_mut();
        ap.h = 11;
        ap.x = 12;
        ap.y = 13;

        println!("rkyv_points {:?}", ap);
        println!("rkyv_points hex {:02X?}", ap);
    }



    // Make a BTree
    type MyMap = BTreeMap<String, usize>;
    const FILE_NAME: &'static str = "data/btree.rkyv";

    // build BTreeMap
    let mut map = MyMap::new();

    for i in 0..100 {
        map.insert(format!("{i}"), i);
    }

    // serialize to disk
    let mut output = OpenOptions::new().create(true).write(true).open(FILE_NAME).unwrap();
    output.write_all(&rkyv::to_bytes::<_, 1024>(&map).unwrap()).unwrap();
    drop(output);

    // map into memory
    let input = OpenOptions::new().read(true).write(true).open(FILE_NAME).unwrap();
    let mut mmap = unsafe { memmap2::MmapMut::map_mut(&input).unwrap() };

    // check bytes (optional)
    rkyv::check_archived_root::<MyMap>(&*mmap).unwrap();

    // cast map back out
    let pinned_bytes = unsafe { Pin::new_unchecked(mmap.as_mut()) };
    let rkyv_map = unsafe { rkyv::archived_root_mut::<MyMap>(pinned_bytes) };

    // verify
    for (k, v) in rkyv_map.iter() {
        println!("k {:?} v {:?}", k, v);
        assert_eq!(k.parse::<usize>().unwrap(), *v as usize);
    }




    // let vec: Vec<(&u16, &u16)> = vec![(&3210, &1), (&3100, &2), (&1000, &3), (&700, &4), (&55, &5), (&22, &2)];

    // let input = OpenOptions::new().read(true).write(true).open("btree_ex").unwrap();
    // let mut mmap = unsafe { memmap2::MmapMut::map_mut(&input).unwrap() };
    // let u8_arr = mmap.as_mut();
    
    
    // let mut ser = AllocSerializer::<1024>::default();
    // unsafe {
    //      let _resolver = ArchivedBTreeMap::serialize_from_reverse_iter(vec.into_iter(), &mut ser)?;
    //      let aligned_vec = ser.into_serializer().into_inner();
    // };
    

    

    Ok(())
} 

fn main() {
    if let Err(e) = run() {
        eprintln!("{}: {}", "Error", e);
        std::process::exit(1);
    }
}
