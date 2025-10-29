fn main() {
    let list1 = vec![34, 50, 25, 100, 65];
    let list2 = vec![1.45, 2.34, 0.67, 3.89, 2.22];

    let l1 = largest(&list1);
    let l2 = largest(&list2);

    println!("largest in list1 is {}", l1);
    println!("largest in list2 is {}", l2);
}

fn largest<T>(list: &[T])-> &T
where
    T: PartialOrd,
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
