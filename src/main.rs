#![stable(feature = "rust1", since = "1.0.0")]

#![feature(libc)]
#![feature(hash)]
#![feature(step_by)]

mod igors_hashmap;


// =======================================================================================


fn main() {
    println!("Here we go");
    
    println!("\ntest #1 - map insertion, search and mem leaks");
    {
        let mut igors_hash1 = igors_hashmap::HashMap::new();
        igors_hash1.insert(&42, &42);
        for test_key in 42..44 {
            let found = igors_hash1.find(&test_key);
            println!("Find key = {}, value = {}", test_key, found.unwrap_or(-1));
        }
    }

    println!("\ntest #2 - collision resolving and mem leaks");
    {
        let mut igors_hash2 = igors_hashmap::HashMap::new();
        for test_key in (0..10).step_by(2) {
            igors_hash2.insert(&test_key, &(test_key * 10));
            igors_hash2.insert(&(test_key + 1024 as i32), &(test_key * 100));
        }

        for test_key in 0..10 {
            let found = igors_hash2.find(&test_key);
            println!("Find key = {}, value = {}", test_key, found.unwrap_or(-1));
        }
        for test_key in 1024..1034 {
            let found = igors_hash2.find(&test_key);
            println!("Find key = {}, value = {}", test_key, found.unwrap_or(-1));
        }
    }

    println!("\ntest #3 - string map");
    {
        let mut igors_hash3: igors_hashmap::HashMap<String, String> =
            igors_hashmap::HashMap::new();

        // causes crashes on windows 7
        // igors_hash3.insert(&"Ihor".to_string(), &"Name".to_string());
        // igors_hash3.insert(&"Kopeichyk".to_string(), &"Last name".to_string());
        // igors_hash3.insert(&"Gennadievich".to_string(), &"Second name".to_string());
        
        // let found = igors_hash3.find(&"Ihor".to_string());
        // println!("\"Ihor\" is {}", found.unwrap_or("[not found".to_string()));

        // let found = igors_hash3.find(&"Vasiliy".to_string());
        // println!("\"Vasiliy\" is {}", found.unwrap_or("[not found]".to_string()));
    }
}
