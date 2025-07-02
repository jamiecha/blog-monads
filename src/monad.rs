use crate::applicative::Applicative;

// 모나드 트레이트 정의
pub trait Monad<A>: Applicative<A> {
    fn bind<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(A) -> Self::Wrapped<B>;
}

// Option 모나드
impl<A> Monad<A> for Option<A> {
    fn bind<B, F>(self, f: F) -> Option<B>
    where
        F: FnOnce(A) -> Option<B>,
    {
        match self {
            Some(value) => f(value),
            None => None,
        }
    }
}

// Result 모나드
impl<A, E> Monad<A> for Result<A, E> {
    fn bind<B, F>(self, f: F) -> Result<B, E>
    where
        F: FnOnce(A) -> Result<B, E>,
    {
        match self {
            Ok(value) => f(value),
            Err(e) => Err(e),
        }
    }
}

pub fn main_monad() {
    // Option 모나드 체이닝
    let result = Some(5)
        .bind(|x| if x > 0 { Some(x * 2) } else { None })
        .bind(|x| if x < 20 { Some(x + 1) } else { None })
        .bind(|x| Some(format!("결과: {}", x)));
    
    println!("Option 체이닝: {:?}", result); // Some("결과: 11")
    
    // 중간에 None이 나오면 전체가 None
    let result2 = Some(-5)
        .bind(|x| if x > 0 { Some(x * 2) } else { None })  // 여기서 None
        .bind(|x| if x < 20 { Some(x + 1) } else { None }) // 실행되지 않음
        .bind(|x| Some(format!("결과: {}", x)));            // 실행되지 않음
    
    println!("실패한 체이닝: {:?}", result2); // None
    
    // Result 모나드 체이닝
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("0으로 나눌 수 없습니다".to_string())
        } else {
            Ok(a / b)
        }
    }
    
    fn sqrt(x: f64) -> Result<f64, String> {
        if x < 0.0 {
            Err("음수의 제곱근을 구할 수 없습니다".to_string())
        } else {
            Ok(x.sqrt())
        }
    }
    
    let computation = Ok(16.0)
        .bind(|x| divide(x, 4.0))  // 16 / 4 = 4
        .bind(|x| sqrt(x))         // sqrt(4) = 2
        .bind(|x| Ok(x * 10.0));   // 2 * 10 = 20
    
    println!("계산 결과: {:?}", computation); // Ok(20.0)
    
    // 에러가 발생하는 경우
    let error_computation = Ok(16.0)
        .bind(|x| divide(x, 0.0))  // 에러 발생!
        .bind(|x| sqrt(x))         // 실행되지 않음
        .bind(|x| Ok(x * 10.0));   // 실행되지 않음
    
    println!("에러 결과: {:?}", error_computation);
} 