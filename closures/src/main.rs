use closures::{self, ShirtColor};
use std::thread;

fn main() {
    {
        let store = closures::Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref1, giveaway1
        );

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref2, giveaway2
        );
    }
    
    {
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;

        closures::generate_workout(simulated_user_specified_value, simulated_random_number);
    }

    {
        // fn add_one_v1(x: u32) -> u32 { x + 1 }
        // let add_one_v2 = |x: u32| -> u32 { x + 1 };
        // let _add_one_v3 = |x| { x + 1 }; //OPTIMIZE: consider giving this closure parameter an explicit type: `: /* Type */`
        let add_one_v4 = |x| x + 1;

        let n = add_one_v4(5);
        println!("\nResult of Add One version 4: {n}");
    }

    {
        let mut list = vec![1, 2, 3];
        println!("\nBefore defining closure: {:?}", list);

        let mut borrows_mutable = || list.push(7);
        // println!("Trying to use mutable variable: {:?}", list); //UGLY: cannot borrow `list` as immutable because it is also borrowed as mutable

        borrows_mutable();
        println!("After calling closure: {:?}", list);

        thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();
    }
}
