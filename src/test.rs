use super::igors_hashmap;


#[test]
pub fn simple_int_pair_insertion() {
    let mut igors_hash1 = igors_hashmap::HashMap::new();
    igors_hash1.insert(&42, &42);

    let found = igors_hash1.find(&42);
    assert_eq!(found.is_some(), true);
    assert_eq!(found.unwrap(), 42);

    let not_found = igors_hash1.find(&43);
    assert_eq!(not_found.is_some(), false);
}

#[test]
pub fn simple_int_pair_removal() {
    let mut igors_hash1 = igors_hashmap::HashMap::new();
    igors_hash1.insert(&42, &42);

    let found = igors_hash1.find(&42);
    assert_eq!(found.is_some(), true);
    assert_eq!(found.unwrap(), 42);

    igors_hash1.remove(&42);
    let found = igors_hash1.find(&42);
    assert_eq!(found.is_some(), false);
}

#[test]
pub fn int_pair_collision_handling() {
    let mut igors_hash2 = igors_hashmap::HashMap::new();
    for test_key in (0..10).step_by(2) {
        igors_hash2.insert(&test_key, &(test_key * 10));
        igors_hash2.insert(&(test_key + 1024 as i32), &(test_key * 100));
    }

    for test_key in 0..10 {
        let found = igors_hash2.find(&test_key);
        if test_key % 2 == 0 {
            assert_eq!(found.is_some(), true);
            assert_eq!(found.unwrap(), test_key * 10);
        } else {
            assert_eq!(found.is_some(), false);
        }
    }

    for test_key in 1024..1034 {
        let found = igors_hash2.find(&test_key);
        if test_key % 2 == 0 {
            assert_eq!(found.is_some(), true);
            assert_eq!(found.unwrap(), (test_key - 1024) * 100);
        } else {
            assert_eq!(found.is_some(), false);
        }
    }
}

#[test]
pub fn simple_string_pair_insertion() {
    let mut igors_hash3: igors_hashmap::HashMap<String, String> =
        igors_hashmap::HashMap::new();

    // causes crashes on windows 7
    igors_hash3.insert(&"Ihor".to_string(), &"Name".to_string());
    igors_hash3.insert(&"Kopeichyk".to_string(), &"Last name".to_string());
    igors_hash3.insert(&"Gennadievich".to_string(), &"Second name".to_string());
    
    let found = igors_hash3.find(&"Ihor".to_string());
    assert_eq!(found.is_some(), true);
    assert_eq!(found.unwrap(), "Name".to_string());

    let found = igors_hash3.find(&"Vasiliy".to_string());
    assert_eq!(found.is_some(), false);
}