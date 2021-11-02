
fn sumVec(arr: &[u32]) ->Option<u32> {
    let mut total=0;
    for i in 0..arr.len() {
        total += arr[i]
        
    }
    let diti=std::u32::MAX as u32;

        if total<= diti{
            return Some(total);
        }else{
            return None;
        }
    

}


fn main() {
    let arr =[1,2,3,4,5,6,7,8,9,10,11];
    let ans= sumVec(&arr);
    println!("{:?}", ans);

}
