mod utils;

fn main() {
    let nums = [1, 2, 3, 4, 5];
    let target = 3;

    let idx: Option<usize> = utils::bin_search(&nums, target, 0, nums.len() - 1);

    print!("index: {}\n", idx.unwrap());
}
