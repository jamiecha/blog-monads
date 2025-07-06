pub fn real_world_monads() {
    println!("=== 실제 Rust에서의 모나드 ===");
    
    // Option 모나드
    println!("\n1. Option 모나드:");
    let result = Some(5)
        .map(|x| x * 2)
        .and_then(|x| if x > 10 { Some(x) } else { None })
        .map(|x| format!("결과: {}", x));
    println!("  체이닝 결과: {:?}", result);
    
    // Result 모나드
    println!("\n2. Result 모나드:");
    let result: Result<i32, String> = Ok(10)
        .and_then(|x| if x > 0 { Ok(x * 2) } else { Err("음수입니다".to_string()) })
        .map(|x| x + 1);
    println!("  에러 처리 결과: {:?}", result);
    
    // Iterator 모나드
    println!("\n3. Iterator 모나드:");
    let result: Vec<i32> = vec![1, 2, 3, 4, 5]
        .into_iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * 2)
        .collect();
    println!("  컬렉션 처리 결과: {:?}", result);
    
    // 에러 처리의 단순화 예제
    println!("\n4. 에러 처리의 단순화:");
    
    // 모나드 없이 (중첩된 match)
    fn process_data_old(data: Option<i32>) -> Option<String> {
        match data {
            Some(value) => {
                match validate(value) {
                    Some(valid) => {
                        match transform(valid) {
                            Some(result) => Some(format!("결과: {}", result)),
                            None => None,
                        }
                    }
                    None => None,
                }
            }
            None => None,
        }
    }
    
    // 모나드 사용 (깔끔한 체이닝)
    fn process_data_new(data: Option<i32>) -> Option<String> {
        data.and_then(validate)
            .and_then(transform)
            .map(|result| format!("결과: {}", result))
    }
    
    fn validate(value: i32) -> Option<i32> {
        if value > 0 { Some(value) } else { None }
    }
    
    fn transform(value: i32) -> Option<i32> {
        if value < 100 { Some(value * 2) } else { None }
    }
    
    let test_data = Some(25);
    println!("  구식 방법: {:?}", process_data_old(test_data));
    println!("  모나드 사용: {:?}", process_data_new(test_data));
    
    println!("\n5. 모나드의 장점:");
    println!("  - 가독성 향상: 복잡한 로직도 한눈에 파악 가능");
    println!("  - 에러 처리 단순화: 중첩된 match 제거");
    println!("  - 조합 가능성: 작은 함수들을 조합해서 복잡한 로직 구성");
    println!("  - 타입 안전성: 컴파일 타임에 에러 처리 보장");
} 