// in second.rs
// pub says we want people outside this module to be able to use List
#[derive(Default)]
pub struct List<T> {
    head: Link<T>,
}


type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        // match &self.head {
        //     None => None,
        //     Some(node) => Some(&node.elem)
        // }

        // we also use as_ref
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // match &mut self.head {
        //     None => None,
        //     Some(node) => Some(&mut node.elem)
        // }

        // 上面的方法可以用 map 简化
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        // 1. 这里使用 map 主要是避免写 match 表达式过于繁琐,
        // node 值即为 match 表达式的 node 值，i在这里类型为 &Box<Node<T>>
        // 2. self 是借用类型，使用到 head 也必须要使用 head 的借用类型，防止 self 被借出不完整
        // 3. 闭包里面 &**node 主要是因为 as_ref()
        // node 类型为 &Box<Node<T>>
        // next 期望为 Option<&Node<T>>,去掉 map 自动封装的 Option 也就是 &Node<T>
        // 所以先用 一个 * 得到 Box<Node<T>>, 再一个 * 得到 Node<T>,最后一个 & 得到 &Node<T>

        // self.head.as_ref().map(|node| &**node) 插件提示优化为
        // self.head.as_deref()
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        // 这里解释和 iter 的原理是一样的
        // self.head.as_mut().map(|node| &mut **node) 插件提示可以优化为
        // self.head.as_deref_mut()
        IterMut { next: self.head.as_deref_mut() }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}


impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // 这个 语句和下面语句是等效的 self.next = node.next.as_ref().map(|node| &**node);
            // todo 了解 node.next.as_deref() 的具体一样
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}


impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            // node.next.as_mut().map(|node| &mut **node) 插件提示优化为
            // node.next.as_deref_mut()
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}


impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn second_test() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();

        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        if let Some(value) = list.peek_mut() { *value = 42 }

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
