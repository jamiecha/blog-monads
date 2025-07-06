// 모듈 선언
mod monoid;
mod functor;
mod applicative;
mod monad;
mod endofunctor;
mod real_world_monads;

// 모듈에서 함수들을 가져오기
use monoid::monoid_examples;
use functor::functor_examples;
use applicative::applicative_functor_examples;
use monad::monad_examples;
use endofunctor::endofunctor_examples;
use real_world_monads::real_world_monads;

fn main() {
    monoid_examples();
    println!("--------------------------------");
    functor_examples();
    println!("--------------------------------");
    endofunctor_examples();
    println!("--------------------------------");
    applicative_functor_examples();
    println!("--------------------------------");
    monad_examples();
    println!("--------------------------------");
    real_world_monads();
    println!("--------------------------------");
}