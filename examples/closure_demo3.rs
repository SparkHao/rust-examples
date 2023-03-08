struct Cacher<T, F>
    where
        T: Fn(F) -> F,
        F: Copy,
    {
        query: T,
        value: Option<F>,
    }

impl<T, F> Cacher<T, F>
where
    T: Fn(F) -> F,
    F: Copy,
{
    fn new(query: T) -> Cacher<T, F> {
        Cacher {
            query,
            value: None,
        }
    }

    // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
    fn value(&mut self, arg: F) -> F {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let mut c = Cacher::new(|x| x);
    dbg!(c.value(32));
    let mut c = Cacher::new(|x| x);
    dbg!(c.value("abc"));
}
