use super::frame::Frame;

/// JVM Stack
pub struct Stack {
    max_size: usize,
    size: usize,
    _top: Option<Box<Frame>>, // Stack is implemented as linked list
}

impl Stack {
    pub fn new(max_size: usize) -> Self {
        Stack {
            max_size,
            size: 0,
            _top: None,
        }
    }

    pub fn push(&mut self, mut frame: Frame) {
        if self.size >= self.max_size {
            panic!("java.lang.StackOverflowError");
        }

        if self._top.is_some() {
            frame.set_lower(self._top.take());
        }

        self._top = Some(Box::new(frame));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<Box<Frame>> {
        if self._top.is_none() {
            panic!("jvm stack is empty!");
        }
        let mut top = self._top.take();
        self._top = top.as_mut().unwrap().lower.take();

        self.size -= 1;

        top
    }

    pub fn top_mut(&mut self) -> &mut Frame {
        if self._top.is_none() {
            panic!("jvm stack is empty!");
        }
        self._top.as_mut().unwrap()
    }
}
