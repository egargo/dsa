#[derive(Debug)]
pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T: std::fmt::Debug> Stack<T> {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    #[allow(dead_code)]
    pub fn push(&mut self, element: T) {
        self.stack.push(element)
    }

    #[allow(dead_code, unused_results)]
    pub fn pop(&mut self) -> Option<T> {
        match self.size() {
            0 => None,
            _ => Some(self.stack.remove(self.stack.len() - 1)),
        }
    }

    #[allow(dead_code)]
    pub fn top(&self) -> Option<&T> {
        match self.size() {
            0 => None,
            _ => Some(self.stack.get(0).unwrap()),
        }
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        match self.size() {
            0 => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        return self.stack.len();
    }
}

fn main() {
    let mut stack: Stack<u32> = Stack::new();

    assert_eq!(stack.pop(), None);
    assert_eq!(stack.top(), None);
    assert_eq!(stack.is_empty(), true);
    assert_eq!(stack.size(), 0);

    stack.push(2);
    stack.push(12);
    stack.push(22);

    assert_eq!(stack.size(), 3);
    assert_eq!(stack.top(), Some(&2));
    stack.pop();
    assert_eq!(stack.top(), Some(&2));
    assert_eq!(stack.is_empty(), false);
    assert_eq!(stack.size(), 2);
}

#[test]
fn test() {
    let mut stack: Stack<u32> = Stack::new();

    assert_eq!(stack.pop(), None);
    assert_eq!(stack.top(), None);
    assert_eq!(stack.is_empty(), true);
    assert_eq!(stack.size(), 0);

    stack.push(2);
    stack.push(12);
    stack.push(22);

    assert_eq!(stack.size(), 3);
    assert_eq!(stack.top(), Some(&2));
    stack.pop();
    assert_eq!(stack.top(), Some(&2));
    assert_eq!(stack.is_empty(), false);
    assert_eq!(stack.size(), 2);
}
