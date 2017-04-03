extern crate rand;

use rand::{Rng, SeedableRng, StdRng};
use std::collections::HashMap;
use std::io::Read;
use std::fs::File;

const BASE: u32 = 26-6+9;

fn int_to_sym(seed: u32) -> char {
    match seed {
        0 => '1',
        1 => '2',
        2 => '3',
        3 => '4',
        4 => '5',
        5 => '6',
        6 => '7',
        7 => '8',
        8 => '9',
        9 => 'B',
        10=> 'C',
        11=> 'D',
        12=> 'F',
        13=> 'G',
        14=> 'H',
        15=> 'J',
        16=> 'K',
        17=> 'L',
        18=> 'M',
        19=> 'N',
        20=> 'P',
        21=> 'Q',
        22=> 'R',
        23=> 'S',
        24=> 'T',
        25=> 'V',
        26=> 'W',
        27=> 'X',
        28=> 'Z',
        _ => panic!("Invalid sym code!")
    }
}

fn int_to_string(seed: u32) -> String {
    let mut s = seed;
    let mut _str = Vec::new();

    for _ in 0..4 {
        let c = int_to_sym(s%BASE);
        _str.push(c);
        s = s/BASE;
    }
    _str.into_iter().collect()
}


fn main() {
    let no_words = ["P1SS", "SH1T", "FVCK", "SH17", "BVTT", "D1CK", "CVNT", "CL1T", "FVGG", "FVGS", "DVMN", "D1K3", "K1K3", "CVCK", "KVCK"];
    let mut f = File::open("all.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let mut no_words_map = HashMap::new();

    for word in no_words.iter() {
        no_words_map.insert(word.to_string(), word.to_string());
    }
    for word in buffer.split('\n') {
        no_words_map.insert(word.to_string(), word.to_string());
    }

    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut population = Vec::new();
    for i in 0..BASE.pow(4) {
        //println!("{}", i);
        if !no_words_map.contains_key::<str>( &int_to_string(i).as_str() ) {
            population.push(i);
        } else {
            //println!("Rejected: {}", int_to_string(i));
        }
    }
    rng.shuffle(population.as_mut_slice());
    for p in population {
        println!("{}", int_to_string(p) );
    }
}
