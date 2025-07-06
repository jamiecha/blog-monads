use crate::applicative::ApplicativeFunctor;

// 모나드 트레이트 정의
pub trait Monad<A>: ApplicativeFunctor<A> {
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

pub fn monad_examples() {
    println!("=== 모나드(Monad) 예제 ===");
    
    println!("\n1. Option 모나드 체이닝:");
    let result = Some(5)
        .bind(|x| if x > 0 { Some(x * 2) } else { None })
        .bind(|x| if x < 20 { Some(x + 1) } else { None })
        .bind(|x| Some(format!("결과: {}", x)));
    
    println!("  성공한 체이닝: {:?}", result);
    
    // 중간에 None이 나오면 전체가 None
    let result2 = Some(-5)
        .bind(|x| if x > 0 { Some(x * 2) } else { None })  // 여기서 None
        .bind(|x| if x < 20 { Some(x + 1) } else { None }) // 실행되지 않음
        .bind(|x| Some(format!("결과: {}", x)));            // 실행되지 않음
    
    println!("  실패한 체이닝: {:?}", result2);
    
    println!("\n2. Result 모나드 체이닝:");
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
    
    println!("  성공한 계산: {:?}", computation);
    
    // 에러가 발생하는 경우
    let error_computation = Ok(16.0)
        .bind(|x| divide(x, 0.0))  // 에러 발생!
        .bind(|x| sqrt(x))         // 실행되지 않음
        .bind(|x| Ok(x * 10.0));   // 실행되지 않음
    
    println!("  에러 발생: {:?}", error_computation);
    
    println!("\n3. 데이터베이스 조회 체이닝 (시뮬레이션):");
    fn get_user_by_id(id: i32) -> Option<String> {
        if id > 0 { Some(format!("사용자_{}", id)) } else { None }
    }
    
    fn get_user_profile(user: String) -> Option<String> {
        Some(format!("{}의 프로필", user))
    }
    
    fn get_user_preferences(profile: String) -> Option<String> {
        Some(format!("{}의 설정", profile))
    }
    
    let user = get_user_by_id(123)
        .bind(|user| get_user_profile(user))
        .bind(|profile| get_user_preferences(profile))
        .bind(|prefs| Some(format!("사용자 설정: {}", prefs)));
    
    println!("  데이터베이스 조회 결과: {:?}", user);
    
    println!("\n4. 모나드의 특징:");
    println!("  - 순차적 실행: 이전 연산의 결과가 다음 연산의 입력");
    println!("  - 에러 전파: 중간에 실패하면 전체 체인이 실패");
    println!("  - 조건부 실행: 값이 있을 때만 다음 연산 실행");
} 