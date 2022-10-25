fn main() {
    println!("Hello, world!");
    println!("{}",fair_dice_roll());

//     struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

struct Number {
	odd: bool,
	value: i32,
}

    let mut n = Number {
    	odd: true,
    	value: 7,
    };

    n.value = 19;
    n.odd = true;
}

fn fair_dice_roll() -> i32{
	4
}
