use core::fmt;
use std::ptr::null_mut;
use std::alloc::{alloc, dealloc, Layout};

//a struct with <T> works similarly to a template class in C++
pub struct Node<T> {
    //the value at this current node.
    val: T,

    //the pointer to the next node
    next: *mut Node<T>
}


pub struct LinkedList<T> {
    //the length of the LinkedList
    len: usize,

    //the beginning of the LinkedList. More info about the syntax above.
    head: *mut Node<T>
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            val: elem,
            next: null_mut() as *mut Node<T>
        }
    }
}

//this allows us to display to the console
impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        unsafe {
            if self.head == null_mut() {
                return write!(f, "This LinkedList is empty");
            }
            let mut curr = self.head;
            loop {
                if (*curr).next == null_mut() {
                    buf += format!("{}", (*curr).val).as_str();
                    break;
                }
                else {
                    buf += format!("{} -> ", (*curr).val).as_str();
                    curr = (*curr).next;
                }
            }
        }
        write!(f, "{}", buf)
    }
}


//this is where we implement all the functions we need
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            head: null_mut() as *mut Node<T>,
        }
    }
    
    pub fn append(&mut self, elem: T) {
        unsafe {
            let mut curr = self.head;
            let mut prev = curr;
            loop {
                if curr != null_mut() {
                    prev = curr;
                    curr = (*curr).next;
                }
                else {
                    let layout = Layout::new::<Node<T>>();
                    let ptr = alloc(layout);
                    *(ptr as *mut Node<T>) = Node::new(elem);
                    curr = ptr as *mut Node<T>;       

                    if self.len == 0 {
                        self.head = curr;
                    }
                    else {
                        (*prev).next = curr;
                    }

                    self.len += 1;
                    break;
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
        unsafe {
            let mut curr = self.head;
            self.head = null_mut();
            loop {
                if curr != null_mut() {
                    let prev = curr;
                    curr = (*curr).next;
                    dealloc(prev as *mut u8, Layout::new::<Node<T>>())
                }
                else {
                    break;
                }
            }
        }
    }

    pub fn get_element(&self, position: usize) -> Result<&T, &str> {
        if position >= self.len {
            return Err("Position out of range");
        }
        unsafe {
            let mut curr = self.head;
            for _i in 0..position {
                if curr != null_mut() {
                    curr = (*curr).next;
                }
                else {
                    return Err("Position out of range");
                }
            }
            return Ok(&(*curr).val);
        }
    }

    pub fn get_length(&self) -> usize {
        //we don't need a semicolon on every line, Rust can interpret this as a usize
        self.len
    }

    pub fn insert(&mut self, position: usize, elem: T) -> Result<(), &str> {
        if self.len == 0 && position == 0 {
            self.append(elem);
            return Ok(());
        }
        else if position >= self.len {
            return Err("Position out of range");
        }
        unsafe {
            let mut curr = self.head;
            let mut prev = curr;
            for _i in 0..position {
                if curr != null_mut() {
                    prev = curr;
                    curr = (*curr).next;
                }
                else {
                    return Err("Position out of range");
                }
            }

            let next_node = curr;
            let layout = Layout::new::<Node<T>>();
            let ptr = alloc(layout);
            *(ptr as *mut Node<T>) = Node::new(elem);
            curr = ptr as *mut Node<T>;
            
            (*prev).next = curr;
            (*curr).next = next_node;

            self.len += 1;
            Ok(())
        }
    }

    pub fn is_empty(&self) -> bool {
        //we don't need a semicolon on every line, Rust can interpret this as a bool
        self.len == 0
    }

    pub fn remove(&mut self, position: usize) -> Result<(), &str> {
        if position >= self.len {
            return Err("Position out of range");
        }
        unsafe {
            let mut curr = self.head;

            if self.head == null_mut() {
                return Err("Position out of range");
            }

            let mut prev = curr;
            for _i in 0..position {
                if curr != null_mut() {
                    prev = curr;
                    curr = (*curr).next;
                }
                else {
                    return Err("Position out of range");
                }
            }
            
            let next_node = (*curr).next;
            let layout = Layout::new::<Node<T>>();
            dealloc(curr as *mut u8, layout);
            
            (*prev).next = next_node;

            if self.len == 1 {
                self.head = null_mut();
            }

            self.len -= 1;
            Ok(())
        }
    }

    pub fn replace(&mut self, position: usize, elem: T) -> Result<(), &str> {
        if position >= self.len {
            return Err("Position out of range");
        }
        unsafe {
            let mut curr = self.head;
            for _i in 0..position {
                if curr != null_mut() {
                    curr = (*curr).next;
                }
                else {
                    return Err("Position out of range");
                }
            }
            (*curr).val = elem;
            Ok(())
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn appending_popping() {
        let mut list = LinkedList::<i32>::new();
        
        //check if adding items to the list
        list.append(5);
        list.append(6);
        list.append(7);

        assert_eq!(list.get_element(0).unwrap(), &5);
        assert_eq!(list.get_element(1).unwrap(), &6);
        assert_eq!(list.get_element(2).unwrap(), &7);
        
        //check if removing the items from the list properly
        assert_eq!(list.remove(2), Ok(()));
        assert_eq!(list.remove(1), Ok(()));
        assert_eq!(list.remove(0), Ok(()));
        assert_eq!(list.remove(0), Err("Position out of range"));

        assert_eq!(list.get_element(0), Err("Position out of range"));
        assert_eq!(list.get_element(1), Err("Position out of range"));
        assert_eq!(list.get_element(2), Err("Position out of range"));
    }

    #[test]
    fn inserting_removing() {
        let mut list = LinkedList::<String>::new();

        //check if inserting properly
        assert_eq!(list.insert(0, "Hi mom".into()), Ok(()));
        assert_eq!(list.insert(0, "Hi dad".into()), Ok(()));
        assert_eq!(list.insert(6, "Hey malcolm".into()), Err("Position out of range"));

        //check if removing properly
        assert_eq!(list.remove(5), Err("Position out of range"));
        assert_eq!(list.remove(1), Ok(()));
        assert_eq!(list.remove(4), Err("Position out of range"));
    }


}

