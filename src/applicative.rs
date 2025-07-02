use crate::functor::Functor;

// 어플리케이티브 펑터 트레이트
pub trait Applicative<A>: Functor<A> {
    fn pure(value: A) -> Self;
    fn apply<B, F>(self, f: Self::Wrapped<F>) -> Self::Wrapped<B>
    where
        F: FnOnce(A) -> B;
}

// Option에 대한 어플리케이티브 구현
impl<A> Applicative<A> for Option<A> {
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

// Result에 대한 어플리케이티브 구현
impl<A, E> Applicative<A> for Result<A, E> {
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

// 실용적인 헬퍼 함수들
pub fn lift2<A, B, C, F>(f: F) -> impl Fn(Option<A>, Option<B>) -> Option<C>
where
    F: Fn(A, B) -> C,
{
    move |opt_a, opt_b| match (opt_a, opt_b) {
        (Some(a), Some(b)) => Some(f(a, b)),
        _ => None,
    }
}

pub fn main_applicative() {
    // 기본 사용법
    let value = Some(5);
    let func = Some(|x: i32| x * 2);
    let result = value.apply(func);
    println!("어플리케이티브 결과: {:?}", result); // Some(10)
    
    // 두 값을 조합하기
    let add = lift2(|x: i32, y: i32| x + y);
    let a = Some(3);
    let b = Some(7);
    let c = None;
    
    println!("3 + 7 = {:?}", add(a, b)); // Some(10)
    println!("3 + None = {:?}", add(a, c)); // None
    
    // 여러 값 조합 예제
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        email: String,
    }
    
    fn make_person(name: String, age: u32, email: String) -> Person {
        Person { name, age, email }
    }
    
    let name = Some("김철수".to_string());
    let age = Some(25);
    let email = Some("kim@example.com".to_string());
    let invalid_age: Option<u32> = None;
    
    // 모든 값이 있을 때
    let person1 = match (name.clone(), age, email.clone()) {
        (Some(n), Some(a), Some(e)) => Some(make_person(n, a, e)),
        _ => None,
    };
    
    // 하나라도 없으면 None
    let person2 = match (name, invalid_age, email) {
        (Some(n), Some(a), Some(e)) => Some(make_person(n, a, e)),
        _ => None,
    };
    
    println!("완전한 사람: {:?}", person1);
    println!("불완전한 사람: {:?}", person2);
} 