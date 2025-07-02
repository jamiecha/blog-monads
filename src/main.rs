// 모듈 선언
mod monoid;
mod functor;
mod applicative;
mod monad;
mod endofunctor;

// 모듈에서 함수들을 가져오기
use monoid::main_monoid;
use functor::main_functor;
use applicative::main_applicative;
use monad::main_monad;
use endofunctor::endofunctor_examples;

fn main() {
    main_monoid();
    println!("--------------------------------");
    main_functor();
    println!("--------------------------------");
    endofunctor_examples();
    println!("--------------------------------");
    main_applicative();
    println!("--------------------------------");
    main_monad();
    println!("--------------------------------");
}