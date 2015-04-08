#![stable(feature = "rust1", since = "1.0.0")]

#![feature(libc)]
#![feature(hash)]
#![feature(step_by)]

extern crate libc;

use std::ptr;
use std::mem;
use std::hash::{ hash, Hash, SipHasher };


struct Node<K: Hash + Eq + Clone, V: Eq + Clone> {
    key: K,
    value: V,
    next: *mut Node<K, V>,
}

const HASHTABLE_SIZE: usize = 1024;
struct HashMap<K: Hash + Eq + Clone, V: Eq + Clone> {
    ht: Vec<*mut Node<K, V>>,
}

impl<K: Hash + Eq + Clone, V: Eq + Clone> HashMap<K, V> {
    pub fn new() -> HashMap<K, V> {
        HashMap { ht: vec![ptr::null_mut(); HASHTABLE_SIZE] }
    }

    unsafe fn drop_node(node_ptr: *mut Node<K, V>) {
        if !(*node_ptr).next.is_null() {
            HashMap::drop_node((*node_ptr).next);
        }

        ptr::read(node_ptr as *mut Node<K, V>);
        libc::free(node_ptr as *mut libc::c_void);           
    }

    fn get_hash<T: Hash>(value: &T) -> usize {
        (hash::<T, SipHasher>(value) % HASHTABLE_SIZE as u64) as usize
    }

    pub fn insert(&mut self, key: &K, value: &V) -> bool {
        let hash = HashMap::<K, V>::get_hash(key);
        unsafe {
            let mut node_ptr = self.ht[hash];

            if !node_ptr.is_null() {
                while !(*node_ptr).next.is_null() {
                    if (*node_ptr).value == *value {
                        return false;
                    }
                    node_ptr = (*node_ptr).next;
                }
            }

            let new_node: *mut Node<K, V> =
                libc::malloc(mem::size_of::<Node<K, V>>() as libc::size_t) as *mut Node<K, V>;
            if new_node.is_null() {
                panic!("New node allocation fail");
            } else {
                (*new_node).key = key.clone();
                (*new_node).value = value.clone();
                (*new_node).next = ptr::null_mut();
            }

            if self.ht[hash].is_null() {
                self.ht[hash] = new_node;
            } else {
                (*node_ptr).next = new_node;
            }
            true
        }
    }

    pub fn find(&self, key: &K) -> Option<V> {
        let hash = HashMap::<K, V>::get_hash(key);
        unsafe {
            let mut node_ptr = self.ht[hash];
            while !node_ptr.is_null() {
                if (*node_ptr).key == *key {
                    return Some((*node_ptr).value.clone());
                }
                node_ptr = (*node_ptr).next;
            }
        }
        None
    }
}

impl<K: Hash + Eq + Clone, V: Eq + Clone> Drop for HashMap<K, V> {
    fn drop(&mut self) {
        for bucket in self.ht.iter() {
            if !bucket.is_null() {
                unsafe { HashMap::drop_node(*bucket) } 
            }
        }
    }
}



fn main() {
    println!("Here we go");
    
    println!("\ntest #1 - map insertion, search and mem leaks");
    {
        let mut igors_hash1 = HashMap::new();
        igors_hash1.insert(&42, &42);
        for test_key in 42..44 {
            let found = igors_hash1.find(&test_key);
            println!("Find key = {}({}), value = {}", test_key,
                     HashMap::<i32, i32>::get_hash(&test_key), found.unwrap_or(-1));
        }
    }

    println!("\ntest #2 - collision resolving and mem leaks");
    {
        let mut igors_hash2 = HashMap::new();
        for test_key in (0..10).step_by(2) {
            igors_hash2.insert(&test_key, &(test_key * 10));
            igors_hash2.insert(&(test_key + HASHTABLE_SIZE as i32), &(test_key * 100));
        }

        for test_key in 0..10 {
            let found = igors_hash2.find(&test_key);
            println!("Find key = {}({}), value = {}", test_key,
                     HashMap::<i32, i32>::get_hash(&test_key), found.unwrap_or(-1));
        }
        for test_key in 1024..1034 {
            let found = igors_hash2.find(&test_key);
            println!("Find key = {}({}), value = {}", test_key,
                     HashMap::<i32, i32>::get_hash(&test_key), found.unwrap_or(-1));
        }
    }

    println!("Hash of 42 = {}", HashMap::<i32, i32>::get_hash(&42));

    println!("\ntest #3 - string map");
    {
        let mut igors_hash3 = HashMap::new();
        let names = [ "Ihor".to_string(),
            "Kopeichyk".to_string(), "Gennadievich".to_string() ];
        let meanings = [ "Name".to_string(),
            "Last name".to_string(), "Second name".to_string() ];

        for item_idx in 0..names.len() {
            igors_hash3.insert(&names[item_idx], &meanings[item_idx]);
        }
        
        let found = igors_hash3.find(&"Ihor".to_string());
        println!("\"Ihor\" is {}", found.unwrap_or("[not found".to_string()));

        let found = igors_hash3.find(&"Vasiliy".to_string());
        println!("\"Vasiliy\" is {}", found.unwrap_or("[not found]".to_string()));
    }
}











