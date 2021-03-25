#[test]
fn get_user_count() {
    let user_count = no_mans_sky::get_user_count().unwrap();
    assert!(user_count > 0);
}
