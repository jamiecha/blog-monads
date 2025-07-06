use crate::functor::Functor;

// 어플리커티브 펑터 트레이트
pub trait ApplicativeFunctor<A>: Functor<A> {
    #[allow(dead_code)]
    fn pure(value: A) -> Self;
    #[allow(dead_code)]
    fn apply<B, F>(self, f: Self::Wrapped<F>) -> Self::Wrapped<B>
    where
        F: FnOnce(A) -> B;
}

// Option에 대한 어플리커티브 구현
impl<A> ApplicativeFunctor<A> for Option<A> {
    fn pure(value: A) -> Self {
        Some(value)
    }
    
    fn apply<B, F>(self, f: Option<F>) -> Option<B>
    where
        F: FnOnce(A) -> B,
    {
        match (self, f) {
            (Some(value), Some(func)) => Some(func(value)),
            _ => None,
        }
    }
}

// Result에 대한 어플리커티브 구현
impl<A, E> ApplicativeFunctor<A> for Result<A, E> {
    fn pure(value: A) -> Self {
        Ok(value)
    }
    
    fn apply<B, F>(self, f: Result<F, E>) -> Result<B, E>
    where
        F: FnOnce(A) -> B,
    {
        match (self, f) {
            (Ok(value), Ok(func)) => Ok(func(value)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

pub fn applicative_functor_examples() {
    println!("=== 어플리커티브 펑터(Applicative Functor) 예제 ===");
    
    println!("\n1. 기본 사용법:");
    let value = Some(5);
    let func = Some(|x: i32| x * 2);
    let result = value.apply(func);
    println!("  Some(5)에 2배 함수 적용: {:?}", result);
    
    println!("\n2. 여러 값 조합 예제:");
    let name = Some("김철수");
    let age = Some(25);
    let email = Some("kim@example.com");
    
    // 모든 값이 있을 때만 Person 생성
    let person = match (name, age, email) {
        (Some(n), Some(a), Some(e)) => Some(Person { 
            name: n.to_string(), 
            age: a, 
            email: e.to_string() 
        }),
        _ => None,
    };
    println!("  완전한 사람: {:?}", person);
    
    println!("\n3. Result 어플리커티브 예제:");
    let success_value: Result<i32, String> = Ok(10);
    let success_func: Result<Box<dyn FnOnce(i32) -> i32>, String> = Ok(Box::new(|x| x * 3));
    let result = success_value.apply(success_func);
    println!("  Result 어플리커티브: {:?}", result);
    
    println!("\n4. 어플리커티브 펑터의 특징:");
    println!("  - 함수도 컨테이너 안에 담아서 적용");
    println!("  - F<A>와 F<A→B>를 조합해서 F<B> 생성");
    println!("  - 병렬적으로 여러 값을 조합 가능");
    println!("  - 하나라도 실패하면 전체 실패");
} 