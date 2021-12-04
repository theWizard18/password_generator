use std::env;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("help")) {
        println!("input a whole number as the first argument to the command for the password length");
        println!("other valid arguments:");
        println!("    no-up : no upper case letters");
        println!("    no-low : no lower case letters");
        println!("    no-num : no numbers");
        println!("    no-spc : no special characters ('&', '@', '!', etc.)");
    } else {
        let chars =  process_args(
            args.contains(&String::from("no-up")),
            args.contains(&String::from("no-low")),
            args.contains(&String::from("no-num")),
            args.contains(&String::from("no-spc"))
        );

        let length = match args.len() {
            1 => 16,
            _ => match args[1].parse::<u8>() {
                Ok(n) => n,
                _ => 16,
            },
        };
        password_gen(length, chars)
    }
}

fn process_args(no_up :bool, no_low :bool, no_num :bool, no_spc :bool) -> Vec<char> {
    let upcase = vec!['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let lowcase = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let numbers = vec!['0','1','2','3','4','5','6','7','8','9'];
    let special = vec!['!','@','#','$','%','&','*','?','~'];
    match (no_up, no_low, no_num, no_spc) {
        // as cat_vecs accepts 2 arguments, it was necessary to pass itself when more vectors were needed.
        (true, true, true, false) => special,
        (true, true, false, true) => numbers,
        (true, true, false, false) => cat_vecs(numbers, special),
        (true, false, true, true) => lowcase,
        (true, false, true, false) => cat_vecs(lowcase, special),
        (true, false, false, true) => cat_vecs(lowcase, numbers),
        (true, false, false, false) => cat_vecs(lowcase, cat_vecs(numbers, special)),
        (false, true, true, true) => upcase,
        (false, true, true, false) => cat_vecs(upcase, special),
        (false, true, false, true) => cat_vecs(upcase, numbers),
        (false, true, false, false) => cat_vecs(upcase, cat_vecs(numbers, special)),
        (false, false, true, true) => cat_vecs(upcase, lowcase),
        (false, false, true, false) => cat_vecs(upcase, cat_vecs(lowcase, special)),
        (false, false, false, true) => cat_vecs(upcase, cat_vecs(lowcase, numbers)),
        _ => cat_vecs(upcase, cat_vecs(lowcase, cat_vecs(numbers, special))),
    }
}

fn cat_vecs(mut vec_a :Vec<char>, mut vec_b :Vec<char>) -> Vec<char> {
    let mut vec_c = vec![];
    vec_c.append(&mut vec_a);
    vec_c.append(&mut vec_b);

    vec_c
}

fn password_gen( len: u8, chars: Vec<char>) {
    let mut password = String::new();
    let mut chr;
    for _i in 0..len {
        chr = chars.choose(&mut thread_rng()).unwrap();
        password.push(*chr);
    };
    println!("{}", password);
}

