/*
    Counts the number of times the maximum changes (from left to right)
*/
fn count_tops(values: [i32;5]) -> i32{
    let mut n_tops = 0;
    let mut curr_max = 0;

    for v in values{
        if v > curr_max {
            curr_max = v;
            n_tops += 1;
        }
    }

    return n_tops;
}

fn main(){
    let values = [1, 3, 5, 2, 7];

    let tops = count_tops(values);

    for e in values{
        println!("Values: {}", e);
    }
    println!("Number of tops: {}", tops)
}
