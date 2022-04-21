//! 用于堆内存管理的裸单向链表
use core::marker::Copy;
use core::{fmt, ptr};

/// 链表节点，其数据为下一个节点
#[derive(Copy, Clone)]
pub struct LinkedList {
    head: *mut usize,
}

impl LinkedList {
    /// 创建空链表
    pub const fn new() -> LinkedList {
        LinkedList {
            head: ptr::null_mut(),
        }
    }

    /// 返回该链表是否为空
    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    /// 在链表头处添加节点
    pub unsafe fn push(&mut self, item: *mut usize) {
        *item = self.head as usize;
        self.head = item;
    }

    /// 弹出链表头处的节点
    pub fn pop(&mut self) -> Option<*mut usize> {
        if self.is_empty() {
            None
        } else {
            let item = self.head;
            self.head = unsafe { *item as *mut usize };
            Some(item)
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            curr: self.head,
            _list: self,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut {
            prev: &mut self.head as *mut *mut usize as *mut usize,
            curr: self.head,
            _list: self,
        }
    }
}

impl fmt::Debug for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

pub struct Iter<'a> {
    curr: *mut usize,
    _list: &'a LinkedList,
}

impl<'a> Iterator for Iter<'a> {
    type Item = *mut usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_null() {
            None
        } else {
            let item = self.curr;
            let next = unsafe { *item as *mut usize };
            self.curr = next;
            Some(item)
        }
    }
}

pub struct ListNode {
    prev: *mut usize,
    curr: *mut usize,
}

impl ListNode {
    pub fn pop(self) -> *mut usize {
        unsafe {
            *(self.prev) = *(self.curr);
        }
        self.curr
    }

    pub fn value(&self) -> *mut usize {
        self.curr
    }
}

pub struct IterMut<'a> {
    _list: &'a mut LinkedList,
    prev: *mut usize,
    curr: *mut usize,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = ListNode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_null() {
            None
        } else {
            let res = ListNode {
                prev: self.prev,
                curr: self.curr,
            };
            self.prev = self.curr;
            self.curr = unsafe { *self.curr as *mut usize };
            Some(res)
        }
    }
}
