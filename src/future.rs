pub trait Future {
    type Output;

    fn poll(&mut self, ctx: &Context) -> Self::Output;

    
}

pub trait TryFuture {
    type Ok;
    type Error;

    fn try_poll(&mut self, ctx: &Context) -> Poll<Result<Self::Ok, Self::Error>>;
}

impl<F, T, E> TryFuture for F
    where
        F: Future<Output = Result<T, E>>,
    {
        type Ok = T;
        type Error = E;

        fn try_poll(&mut self, cx: &Context) -> Poll<F::Output> {
            self.poll(cx)
        }
    }


pub enum Poll<T> {
    Pending,
    Ready(T)
}

pub struct Context<'a> {
    waker: &'a Waker
}

impl<'a> Context<'a> {
    pub fn from_waker(waker: &'a Waker) -> Self {
        Self { waker }
    }

    pub fn waker(&self) -> &'a Waker {
        self.waker
    }
}

pub struct Waker;
