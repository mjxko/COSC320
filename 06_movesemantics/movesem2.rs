fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    let vec0 = vec![22, 44, 66];
    let vec1 = fill_vec(vec0);

    println!("Updated vector: {:?}", vec1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);       // our original vector stays unchanged
        assert_eq!(vec1, [22, 44, 66, 88]);  // our updated vector
    }
}
