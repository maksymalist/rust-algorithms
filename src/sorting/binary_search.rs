fn binary_search(arr: Vec<i32>, target: i32) -> i32{
    let mut high: usize = arr.len() - 1;
    let mut low: usize = 0;

    while high > low{
        let mid = (high + low) / 2;

        if arr[mid] == target{
            return mid as i32;
        }

        if arr[mid] > target {
            high -= 1
        }

        if arr[mid] < target {
            low += 1
        }
    }

    return -1;

}

pub fn main(){
    let arr = vec![1,2,3,4,5,6,7,8,9,10];
    const TARGET: i32 = 5;

    let position: i32 = binary_search(arr, TARGET);

    println!("Found target {} at postion {}", TARGET, position)
}