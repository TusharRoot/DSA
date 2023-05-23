fn main() {

    let mut test = Vec::new();
    test.push(3);
    test.push(3);

    let sum = 6;
    let data = two_sum(test, sum);
    // let mut v1 = Vec::new();
    // let mut data;
    // for i in &test{
    //     let index = test.iter().position(|&r| r == *i).unwrap();
    //     data = sum - i;
    //     for j in index..test.len(){
    //         if data == test[j]{
    //             v1.push(index);
    //             v1.push(j);
    //         }
    //     } 
    // }
    println!("{:?}",data);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut v1:Vec<i32> = Vec::new();
    let mut data;
    let mut flag:bool = false;
for i in &nums{
    let index = nums.iter().position(|&r| r == *i).unwrap();
    //6
    data = target - i;
    for j in index+1..nums.len(){
        if data == nums[j]{
            v1.push(index as i32);
            v1.push(j as i32);
            flag = true;
            break;
        }
    } 
    if flag{
        break;
    }
}
    return v1
}
