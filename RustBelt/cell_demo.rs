use std::{cell::Cell, ops::IndexMut};

#[test]
// Cell: !Sync なので、シングルスレッドで sequential に実行されると考えていい。
fn demo_cell_1() {
    // define IMMUTABLE cells.
    let cell1 = &Cell::new(10);
    let cell2 = &Cell::new(20);

    // 10, 20
    cell1.set(1);
    // 1, 20
    cell1.swap(&cell2);
    // 20, 1
    let cell1_cpy = cell1;
    // do nothing
    dbg!(cell1_cpy.replace(2));
    // 2, 1 (print 20)
    dbg!(cell2.take());
    // 2, 0 (print 1)
    println!("cell1 = {:?}, cell2 = {:?}", cell1.get(), cell2.get());
}

#[test]
// pub fn get_mut(&mut self) -> &mut T
fn demo_cell_get_mut() {
    let data = [1, 3, 2];
    let cell = &mut Cell::new(data);

    let mut_pointer = cell.get_mut();
    mut_pointer.sort();

    println! {"{:?}", cell}
    // cannot use mut_pointetr anymore.
    // println! {"{:?}", mut_pointer}
}

#[test]
// pub const fn as_ptr(&self) -> *mut T
fn demo_cell_as_ptr() {
    let mut data = vec![10, 20];
    let cell = Cell::from_mut(&mut data);

    unsafe {
        let first_p = cell.as_ptr();
        let second_p = (*cell.as_ptr()).index_mut(1);

        (*first_p).truncate(1);
        (*first_p).shrink_to_fit();
        *second_p += 1; // pointer

        // println!("{}", { second_p });
    }

    let mut contents = cell.replace(vec![]);
    println!(
        "{:?} (capacity = {}, length = {})",
        contents,
        contents.capacity(),
        contents.len()
    );
    unsafe {
        contents.set_len(10); // unsafe command to
    }
    println!(
        "{:?} (capacity = {}, length = {})",
        contents,
        contents.capacity(),
        contents.len()
    );
}
