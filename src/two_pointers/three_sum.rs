fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {

    fn valid(nums: &Vec<i32>, out: &Vec<Vec<i32>>) -> bool{

        if out.contains(nums){
            return false
        }
    
        for i in out{
            if i.contains(&nums[0]) && i.contains(&nums[1]) && i.contains(&nums[2]){
                return false;
            }
        }
    
        if nums[0] == 0 && nums[1] == 0 && nums[2] == 0{
            return true
        }
    
        if nums[0] != nums[1] && nums[0] != nums[2] && nums[1] != nums[2]{
            return true
        }
        return false
    
    }

    let mut out: Vec<Vec<i32>> = Vec::new();

    for i in 0..nums.len(){
        let item = nums[i];

        let mut selection: Vec<i32> = vec![item];
        let mut start: usize = 0;
        let mut end: usize = nums.len() - 1;

        while end > 0 as usize{

            if start != i && end != i{

                if nums[start] + selection[0] + nums[end] == 0{
                    
                    selection.push(nums[start]);
                    selection.push(nums[end]);
    
                    if valid(&selection, &out){
                        out.push(selection.clone())
                    }

                }
            }

            println!("{} - {}", start, end);
            start += 1;
            selection = vec![item];

            if start >= end{
                start = 0;
                end = end - 1;
            }

            
        }

    }

    return out

}

pub fn main(){
    let nums: Vec<i32> = vec![-1,0,1,2,-1,-4];
    println!("{:?}", three_sum(nums))
}