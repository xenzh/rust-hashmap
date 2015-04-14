#![stable(feature = "rust1", since = "1.0.0")]

extern crate libc;

use std::ptr;
use std::mem;
use std::hash::{ hash, Hash, SipHasher };

// =======================================================================================

struct Node<K: Hash + Eq + Clone, V: Eq + Clone> {
    key: K,
    value: V,
    next: *mut Node<K, V>,
}


const HASHTABLE_SIZE: usize = 1024;
pub struct HashMap<K: Hash + Eq + Clone, V: Eq + Clone> {
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

// =======================================================================================

struct HashMapIterator<K: Hash + Eq + Clone, V: Eq + Clone> {
    pub noopk: K,
    pub noopv: V,
}

// impl<K: Hash + Eq + Clone, V: Eq + Clone> Iterator for HashMapIterator<K, V> {
//     pub fn next(&mut self) ->Option<<Self as Iterator>::Item> {
//         None
//     }
// }
