// Parallel iteration
// Finished in 3 secs.
// sorted_nodes.par_iter_mut().enumerate().for_each(|(sorted_i, sorted_node)| {
//     let i = hilbert_node_pairs[sorted_i].i() as usize;
//     let node = &nodes[i];
//     sorted_node.fill_from(node);
// });

// Parallel iteration via zip
// Finished in 2 secs.
// sorted_nodes.par_iter_mut().zip(hilbert_node_pairs.par_iter_mut()).for_each(|(sorted_node, hilbert_node_pair)| {
//     let i = hilbert_node_pair.i() as usize;
//     let node = &nodes[i];
//     sorted_node.fill_from(node);
// });

// Serial iteration
// Finished in 12 secs.
// let mut sorted_i: usize = 0;
// for pair in hilbert_node_pairs {
//     let i = pair.i() as usize;
//     let node = &nodes[i];
//     sorted_nodes[sorted_i].fill_from(node);
//     sorted_i += 1;
// }

// Append to new file
// Very slow.
// let mut f = OpenOptions::new()
//     .read(true)
//     .write(true)
//     .create(true)
//     .open(dir.join("sorted_nodes_appended"))?;

// for p in hilbert_node_pairs {
//     let i = p.i() as usize;
//     let n: &Node = &nodes[i];
//     let b: &[u8] = unsafe {
//         from_raw_parts((n as *const Node) as *const u8, size_of::<Node>())
//     };
//     f.write_all(b)?;
// }