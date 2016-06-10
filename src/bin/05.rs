extern crate rayon;

use rayon::prelude::*;

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    v.par_iter_mut()
     .for_each(|i| *i += 1);

    println!("{:?}", v);
}
