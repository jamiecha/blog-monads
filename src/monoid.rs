// 모노이드 트레이트 정의
pub trait Monoid {
    fn empty() -> Self;
    fn append(&self, other: &Self) -> Self;
}

// 문자열은 모노이드예요
impl Monoid for String {
    fn empty() -> Self {
        String::new()  // 빈 문자열이 항등원
    }
    
    fn append(&self, other: &Self) -> Self {
        format!("{}{}", self, other)  // 문자열 연결
    }
}

// 숫자도 모노이드예요 (덧셈 기준)
#[derive(Debug, Clone)]
pub struct Sum(i32);

impl Monoid for Sum {
    fn empty() -> Self {
        Sum(0)  // 0이 항등원
    }
    
    fn append(&self, other: &Self) -> Self {
        Sum(self.0 + other.0)
    }
}

// 벡터도 모노이드예요
impl<T: Clone> Monoid for Vec<T> {
    fn empty() -> Self {
        Vec::new()  // 빈 벡터가 항등원
    }
    
    fn append(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.extend_from_slice(other);
        result
    }
}

pub fn monoid_examples() {
    println!("=== 모노이드(Monoid) 예제 ===");
    
    println!("\n1. 문자열 모노이드:");
    let hello = "안녕".to_string();
    let world = "하세요".to_string();
    let result = hello.append(&world);
    println!("  '안녕' + '하세요' = '{}'", result);
    
    println!("\n2. 숫자 덧셈 모노이드:");
    let a = Sum(5);
    let b = Sum(3);
    let c = Sum(2);
    
    // 결합법칙 확인: (a + b) + c = a + (b + c)
    let left = a.append(&b).append(&c);
    let right = a.append(&b.append(&c));
    println!("결합법칙 확인:");
    println!("  (a + b) + c = {:?}", left);
    println!("  a + (b + c) = {:?}", right);
    println!("  결과: {}", if left.0 == right.0 { "일치" } else { "불일치" });
    
    // 항등원 확인
    let identity = Sum::empty();
    println!("\n항등원 확인:");
    println!("  a + 0 = {:?}", a.append(&identity));
    println!("  0 + a = {:?}", identity.append(&a));
    
    println!("\n3. 여러 문자열 결합:");
    let words = vec!["안녕", "하세요", "반갑습니다"];
    let result = words.iter().fold(String::empty(), |acc, word| acc.append(&word.to_string()));
    println!("결합 결과: '{}'", result);
} 