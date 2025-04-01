use std::sync::Arc;

use joylight_backend::fixtures::{Selection, StrictSelection};

fn print_type<T>(_: &T) {
    println!("{:?}", std::any::type_name::<T>());
}

fn main() {
    let toast: Arc<StrictSelection> = Arc::new(StrictSelection {
        name: "toast".to_string(),
        fixtures: vec![],
    });

    println!("🍞 number: {:?}", Arc::strong_count(&toast));
    print_type(&toast);

    let toast2: Arc<dyn Selection> = toast as std::sync::Arc<dyn Selection>;
    // let toast2mum: Arc<RwLock<dyn Selection>> = toast.clone().into();
    // let toast2: Arc<RwLock<dyn Selection>> = ((toast as dyn Into<Arc<RwLock<dyn Selection>>>).into()).clone();

    println!("🍞 number: {:?}", Arc::strong_count(&toast2));
    print_type(&toast2);

    // let toast3: Arc<RwLock<StrictSelection>> = toast.clone();

    // println!("🍞 number: {:?}", Arc::strong_count(&toast3));

    // let toast4: Arc<RwLock<StrictSelection>> = toast2.clone();

    println!("test");
    // println!("toast: {:?}", toast.clone());
    // println!("toast2: {:?}", toast2.clone());
}
