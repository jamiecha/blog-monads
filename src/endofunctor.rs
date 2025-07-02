pub fn endofunctor_examples() {
    // Option<i32> -> Option<String> (같은 Option 범주 내에서)
    let opt_num: Option<i32> = Some(42);
    let opt_str: Option<String> = opt_num.map(|x| x.to_string());
    
    // Vec<i32> -> Vec<String> (같은 Vec 범주 내에서)
    let vec_num = vec![1, 2, 3];
    let vec_str: Vec<String> = vec_num.into_iter().map(|x| x.to_string()).collect();
    
    // Result<i32, E> -> Result<String, E> (같은 Result 범주 내에서)
    let result_num: Result<i32, &str> = Ok(100);
    let result_str: Result<String, &str> = result_num.map(|x| format!("값: {}", x));
    
    println!("Option: {:?}", opt_str);   // Some("42")
    println!("Vec: {:?}", vec_str);      // ["1", "2", "3"]
    println!("Result: {:?}", result_str); // Ok("값: 100")
} 