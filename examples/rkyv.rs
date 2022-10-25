type Error = Box<dyn std::error::Error>;

use std::pin::Pin;

use rkyv::{Archive, Deserialize, Serialize};
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

    Ok(())
} 

fn main() {
    if let Err(e) = run() {
        eprintln!("{}: {}", "Error", e);
        std::process::exit(1);
    }
}
