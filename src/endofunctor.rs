pub fn endofunctor_examples() {
    println!("=== 엔도펑터(Endofunctor) 예제 ===");
    
    println!("\n1. 같은 범주 내에서의 변환:");
    
    // Option<i32> -> Option<String> (같은 Option 범주 내에서)
    let opt_num: Option<i32> = Some(42);
    let opt_str: Option<String> = opt_num.map(|x| x.to_string());
    println!("  Option<i32> -> Option<String>: {:?} -> {:?}", opt_num, opt_str);
    
    // Vec<i32> -> Vec<String> (같은 Vec 범주 내에서)
    let vec_num = vec![1, 2, 3];
    let vec_str: Vec<String> = vec_num.into_iter().map(|x| x.to_string()).collect();
    println!("  Vec<i32> -> Vec<String>: {:?} -> {:?}", vec![1, 2, 3], vec_str);
    
    // Result<i32, E> -> Result<String, E> (같은 Result 범주 내에서)
    let result_num: Result<i32, &str> = Ok(100);
    let result_str: Result<String, &str> = result_num.map(|x| format!("값: {}", x));
    println!("  Result<i32, E> -> Result<String, E>: {:?} -> {:?}", result_num, result_str);
    
    println!("\n2. 타입 생성자 F가 동일하게 유지됨:");
    println!("  F<A> -> F<B> (F는 변하지 않음, A와 B만 변함)");
    println!("  ✅ Option<i32> -> Option<String>");
    println!("  ✅ Vec<i32> -> Vec<String>");
    println!("  ✅ Result<i32, E> -> Result<String, E>");
    
    println!("\n3. Non-Endofunctor (다른 범주로 변환):");
    println!("  ❌ Option<i32> -> Vec<String> (다른 타입 생성자)");
    println!("  ❌ Vec<i32> -> Option<String> (다른 타입 생성자)");
    println!("  ❌ Result<i32, E> -> Option<String> (다른 타입 생성자)");
    
    println!("\n4. 'Endo'의 어원:");
    println!("  Endo (그리스어) = '내부의', '같은 곳의'");
    println!("  Endo + Functor = Endofunctor");
    println!("  = '같은 타입 생성자 내에서 함수를 적용할 수 있는 구조'");
} 