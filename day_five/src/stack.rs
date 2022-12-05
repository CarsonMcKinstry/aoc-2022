#[derive(Clone)]
pub(crate) struct Stack {
    stack: Vec<char>,
  }
  
  impl Stack {
    pub(crate) fn new() -> Self {
      Stack { stack: Vec::new() }
    }

    pub(crate) fn from_str(input: &str) -> Self {
        let mut stack = Stack::new();

        for c in input.chars() {
            stack.push(c)
        }
        
        stack
    }

    pub(crate) fn length(&self) -> usize {
      self.stack.len()
    }
  
    pub(crate) fn pop(&mut self) -> Option<char> {
      self.stack.pop()
    }
  
    pub(crate) fn push(&mut self, item: char) {
      self.stack.push(item)
    }
  
    pub(crate) fn is_empty(&self) -> bool {
      self.stack.is_empty()
    }
  
    pub(crate) fn peek(&self) -> Option<&char> {
      self.stack.last()
    }
  }