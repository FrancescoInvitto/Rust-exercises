fn main(){
    let values_iterator = [1, 3, 5, 2, 7].iter();
    let n_tops = values_iterator.fold((0, 0), |(maxVal, count), &x| if x > maxVal { (x, count + 1) } else { (maxVal, count) });
    
    println!("Number of tops: {}", n_tops.1)
}



