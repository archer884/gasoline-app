#[derive(Serialize)]
pub struct Collection<T> {
    count: usize,
    items: Vec<T>,
}

impl<T> Collection<T> {
    pub fn new(items: Vec<T>) -> Collection<T> {
        Collection {
            count: items.len(),
            items: items,
        }
    }
}
