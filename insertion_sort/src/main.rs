// 挿入ソート
fn insertion_sort<T>(origine_vector: &mut Vec<T>)
where
    T: PartialOrd + Copy + std::fmt::Debug,
{
    for i in 1..origine_vector.len() {
        let target = origine_vector[i];
        let mut tmp = i;
        
        // 隣の数字と比較
        while tmp > 0 && origine_vector[tmp - 1] > target {
            origine_vector[tmp] = origine_vector[tmp - 1];
            tmp -= 1;
        }
        // 数字の入れ替え
        origine_vector[tmp] = target;
        println!("i({})   : {:?}", i, origine_vector);
    }
}

fn main() {
    let mut origine_vector = vec![5, 2, 4, 6, 1, 3];
    println!("initial: {:?}", origine_vector);
    insertion_sort(&mut origine_vector);
    println!("sorted : {:?}", origine_vector);
}
