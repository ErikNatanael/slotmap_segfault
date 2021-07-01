use rand::prelude::*;

fn main() {
    let mut sm = slotmap::HopSlotMap::with_capacity(10000);
    let mut keys = Vec::new();
    let mut rng = rand::thread_rng();
    // Repeat simulate actual usage by going back and forth between inserting and removing
    for _j in 0..20 {
        // Insert a random number of new values
        for _i in 0..(rng.gen::<usize>() % 20 + 1) {
            // A boxed value causes a segfault or an "abort"
            let data = Box::new(4);
            // A non boxed value causes an infinite busy loop
            // let data = 4;
            keys.push(sm.insert(data));
        }

        // Randomly remove certain elements, probably out of order
        for _k in 0..rng.gen::<usize>() % keys.len() {
            let key = keys.remove(rng.gen::<usize>() % (keys.len() - 1));
            sm.remove(key);
        }
    }
    // This is where the segfault/abort/busy loop happens. Also happens with clear().
    println!("Number of elements in map: {}", sm.len());
    for (i, (key, value)) in sm.drain().enumerate() {
        // When a crash happens, the number of key value pairs is not the same as the len
        println!("value num {}", i);
    }
    println!("Hello, world!");
}
