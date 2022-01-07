use map::Map;

#[test]
fn add() {
    let mut map = Map::default();
    let mut room_id = map.create_raw(0, (0f32, 0f32), vec![]);
    let mut rng = rand::thread_rng();

    let add_res = map.add(room_id, 1, &mut rng, 1);
    assert!(add_res.is_ok(), "second room creation must always succeed");
    room_id = add_res.unwrap();

    // TODO: this has a small chance of failure.., I should not be dependent on rng for tests.
    assert!(
        map.add(room_id, 1, &mut rng, 5).is_ok(),
        "third room creation is very likely to succeed"
    );

    let mut has_failed = false;
    for _ in 0..100 {
        let add_res = map.add(room_id, 1, &mut rng, 1);
        if add_res.is_err() {
            has_failed = true;
            break;
        }
        room_id = add_res.unwrap();
    }
    assert!(
        has_failed,
        "Adding a lot of rooms has a very high chance of failure"
    );
}
