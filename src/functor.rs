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

// Vec는 별도의 FunctorMut 트레이트를 사용해야 합니다
// (Iterator::map이 FnMut을 요구하기 때문)
pub trait FunctorMut<A> {
    type Wrapped<B>;
    fn fmap<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnMut(A) -> B;
}

impl<A> FunctorMut<A> for Vec<A> {
    type Wrapped<B> = Vec<B>;
    
    fn fmap<B, F>(self, f: F) -> Vec<B>
    where
        F: FnMut(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

pub fn main_functor() {
    // Option 펑터
    let some_value = Some(5);
    let none_value: Option<i32> = None;
    
    let doubled_some = Functor::fmap(some_value, |x| x * 2);
    let doubled_none = Functor::fmap(none_value, |x| x * 2);
    
    println!("Some(5) * 2 = {:?}", doubled_some); // Some(10)
    println!("None * 2 = {:?}", doubled_none);     // None
    
    // Result 펑터
    let success: Result<i32, String> = Ok(10);
    let failure: Result<i32, String> = Err("에러 발생".to_string());
    
    let success_doubled = Functor::fmap(success, |x| x * 2);
    let failure_doubled = Functor::fmap(failure, |x| x * 2);
    
    println!("성공 케이스: {:?}", success_doubled); // Ok(20)
    println!("실패 케이스: {:?}", failure_doubled); // Err("에러 발생")
}

 