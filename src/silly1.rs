pub struct Stack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: None,
        });

        self.push_node(new_node);
    }

    fn push_node(&mut self, mut node: Box<Node<T>>) {
        node.next = self.head.take();
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|node| {
            node.elem
        })
    }

    fn pop_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Some(boxed_node) = current_link {
            current_link = boxed_node.next;
        }
    }
}

pub struct List<T> {
    left: Stack<T>,
    right: Stack<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { left: Stack::new(), right: Stack::new() }
    }

    pub fn push_left(&mut self, elem: T) { self.left.push(elem) }
    pub fn push_right(&mut self, elem: T) { self.right.push(elem) }

    pub fn pop_left(&mut self) -> Option<T> { self.left.pop() }
    pub fn pop_right(&mut self) -> Option<T> { self.right.pop() }
}
