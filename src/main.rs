use rand::prelude::*;
struct DataStruct {
    pub closure: Box<dyn FnMut() -> f32>,
}

fn main() {
    let mut sm = slotmap::DenseSlotMap::with_capacity(10000);
    let mut keys = Vec::new();
    let mut rng = rand::thread_rng();
    for _j in 0..20 {
        for _i in 0..rng.gen::<usize>() % 200 {
            let mut counter = 0.0;
            let mut vector = vec![10, 20];
            let data = DataStruct {
                closure: Box::new(move || {
                    counter += vector[0] as f32;
                    counter += 1.0;
                    vector[0] += counter as u32;
                    counter
                }),
            };
            keys.push(sm.insert(data));
        }

        for (_key, data) in sm.iter_mut() {
            print!("{}, ", (data.closure)());
        }

        for _k in 0..rng.gen::<usize>() % keys.len() {
            let key = keys.remove(rng.gen::<usize>() % (keys.len() - 1));
            sm.remove(key);
        }
    }
    sm.clear();
    println!("Hello, world!");
}
