trait CollectIntoArray<T> {
    fn collect_array<const LEN: usize>(self) -> [T; LEN];
}

impl<T: Copy, I: Iterator<Item = T>> CollectIntoArray<T> for I {
    fn collect_array<const LEN: usize>(self) -> [T; LEN] {
        let mut arr = [std::mem::MaybeUninit::<T>::uninit(); LEN];
        let count = self
            .into_iter()
            .enumerate()
            .take(LEN)
            .map(|(i, e)| {
                arr.get_mut(i)
                    .expect("array len is smaller than the iterator's")
                    .write(e);
            })
            .count();
        if count != LEN {
            panic!("array len is greater than the iterator's");
        }
        arr.map(|e| unsafe { e.assume_init() })
    }
}
