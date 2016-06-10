extern crate crossbeam;

fn main() {
    let mut array = [1, 2, 3, 4, 5];

    crossbeam::scope(|scope| {
        for i in 0..5 {
            scope.spawn(move || {
                array[i] += 1;
            });
        }
    });

    println!("{:?}", array);
}
