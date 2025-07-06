// 펑터 트레이트 정의 (FnOnce용)
pub trait Functor<A> {
    type Wrapped<B>;
    fn fmap<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(A) -> B;
}

// Option은 Functor예요
impl<A> Functor<A> for Option<A> {
    type Wrapped<B> = Option<B>;
    fn fmap<B, F>(self, f: F) -> Option<B>
    where
        F: FnOnce(A) -> B,
    {
        match self {
            Some(value) => Some(f(value)),
            None => None,
        }
    }
}

// Result는 Functor예요
impl<A, E> Functor<A> for Result<A, E> {
    type Wrapped<B> = Result<B, E>;
    fn fmap<B, F>(self, f: F) -> Result<B, E>
    where
        F: FnOnce(A) -> B,
    {
        match self {
            Ok(value) => Ok(f(value)),
            Err(e) => Err(e),
        }
    }
}

pub fn functor_examples() {
    println!("=== 펑터(Functor) 예제 ===");
    
    println!("\n1. Option 펑터:");
    let some_value = Some(5);
    let none_value: Option<i32> = None;
    
    let doubled_some = Functor::fmap(some_value, |x| x * 2);
    let doubled_none = Functor::fmap(none_value, |x| x * 2);
    
    println!("  Some(5) * 2 = {:?}", doubled_some);
    println!("  None * 2 = {:?}", doubled_none);
    
    println!("\n2. Result 펑터:");
    let success: Result<i32, String> = Ok(10);
    let failure: Result<i32, String> = Err("에러 발생".to_string());
    
    let success_doubled = Functor::fmap(success, |x| x * 2);
    let failure_doubled = Functor::fmap(failure, |x| x * 2);
    
    println!("  성공 케이스: {:?}", success_doubled);
    println!("  실패 케이스: {:?}", failure_doubled);
    
    println!("\n3. 실제 사용 예제:");
    let user_id = Some(123);
    let user_name = user_id.fmap(|id| format!("user_{}", id));
    println!("  사용자 ID 변환: {:?}", user_name);
}

 