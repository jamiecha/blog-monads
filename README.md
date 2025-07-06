# Rust 모나드 블로그 예제

이 프로젝트는 Rust에서 함수형 프로그래밍의 핵심 개념들을 구현한 예제입니다.

## 포함된 개념들

### 1. 모노이드 (Monoid)
- **위치**: `src/monoid.rs`
- **구현**: `String`, `Sum`, `Vec<T>`
- **특징**: 결합법칙과 항등원을 만족하는 이진 연산

### 2. 펑터 (Functor)
- **위치**: `src/functor.rs`
- **구현**: `Option<T>`, `Result<T, E>`
- **특징**: `map` 연산을 통해 값을 변환

### 3. 엔도펑터 (Endofunctor)
- **위치**: `src/endofunctor.rs`
- **예제**: 같은 범주 내에서의 변환 예제들

### 4. 어플리커티브 펑터 (Applicative Functor)
- **위치**: `src/applicative.rs`
- **구현**: `Option<T>`, `Result<T, E>`
- **특징**: `pure`와 `apply` 연산으로 여러 값을 조합

### 5. 모나드 (Monad)
- **위치**: `src/monad.rs`
- **구현**: `Option<T>`, `Result<T, E>`
- **특징**: `bind` 연산으로 순차적 계산 체이닝

## 실행 방법

```bash
# 빌드
cargo build

# 실행
cargo run
```

## 출력 예시

```
안녕하세요
왼쪽: Sum(10), 오른쪽: Sum(10)
a + 0 = Sum(5)
0 + a = Sum(5)
--------------------------------
Some(5) * 2 = Some(10)
None * 2 = None
성공 케이스: Ok(20)
실패 케이스: Err("에러 발생")
--------------------------------
Option: Some("42")
Vec: ["1", "2", "3"]
Result: Ok("값: 100")
--------------------------------
어플리커티브 펑터 결과: Some(10)
3 + 7 = Some(10)
3 + None = None
완전한 사람: Some(Person { name: "김철수", age: 25, email: "kim@example.com" })
불완전한 사람: None
--------------------------------
Option 체이닝: Some("결과: 11")
실패한 체이닝: None
계산 결과: Ok(20.0)
에러 결과: Err("0으로 나눌 수 없습니다")
--------------------------------
```

## 프로젝트 구조

```
src/
├── main.rs          # 메인 진입점
├── monoid.rs        # 모노이드 관련
├── functor.rs       # 펑터 관련
├── endofunctor.rs   # 엔도펑터 예제
├── applicative.rs   # 어플리커티브 펑터 관련
└── monad.rs         # 모나드 관련
```

## 학습 목표

이 프로젝트를 통해 다음을 학습할 수 있습니다:

1. **함수형 프로그래밍의 기본 개념들**
2. **Rust의 트레이트 시스템 활용**
3. **제네릭 프로그래밍**
4. **타입 안전성과 컴파일 타임 검증**
5. **모듈화된 코드 구조**

## 라이선스

MIT License 