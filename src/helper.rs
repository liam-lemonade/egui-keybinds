use std::collections::HashSet;

// returns values in `one` that are not in set `two`
pub fn vec_intersection<T: Clone + std::hash::Hash + Eq>(one: &Vec<T>, two: &Vec<T>) -> Vec<T> {
    let set_one: HashSet<T> = one.iter().cloned().collect();
    let set_two: HashSet<T> = two.iter().cloned().collect();

    (&set_one - &set_two).iter().cloned().collect()
}
