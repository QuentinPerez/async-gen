use async_gen::{generator, GeneratorState};
use std::pin::pin;

#[tokio::test]
async fn single_err() {
    let mut s = pin!(generator! {
        if true {
            Err("hello")?;
        } else {
            yield "world";
        }
        Result::<_, &str>::Ok(())
    });
    assert_eq!(s.resume().await, GeneratorState::Complete(Err("hello")));
}

#[tokio::test]
async fn yield_then_err() {
    let mut s = pin!(generator! {
        yield "hello";
        Err("world")?;
        Result::<_, &str>::Ok(())
    });
    assert_eq!(s.resume().await, GeneratorState::Yielded("hello"));
    assert_eq!(s.resume().await, GeneratorState::Complete(Err("world")));
}
