use libzfs::Libzfs;
#[test]
fn zpool_list_test() {
    // this test assumes a pool called "test" exists

    let mut zfs = Libzfs::new();
    let pool = zfs.pool_by_name("test");
    let pool = pool.unwrap();
    let _state = pool.state();
    dbg!(pool);
}
