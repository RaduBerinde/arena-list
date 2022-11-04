use std::marker::PhantomData;

fn main() {
    let mut a = Arena::<Elem>::new(0);
    let mut l = List::<Elem>::new();
    let e1 = a.alloc(Elem::new(1));
    let e2 = a.alloc(Elem::new(2));
    let e3 = a.alloc(Elem::new(3));
    l.push_front(&mut a, e2);
    print(&l, &a);
    l.push_front(&mut a, e1);
    l.push_back(&mut a, e3);
    print(&l, &a);
    l.remove(&mut a, e2);
    print(&l, &a);
    l.remove(&mut a, e1);
    print(&l, &a);
    l.remove(&mut a, e3);
    print(&l, &a);
}

fn print(l: &List<Elem>, a: &Arena<Elem>) {
    let mut vals = Vec::new();
    let mut e = l.head;
    while !e.is_null() {
        let elem = a.get(e);
        vals.push(elem.val);
        e = elem.next;
    }
    println!("{:?}", vals);
}

#[derive(Clone, Copy)]
struct Elem {
    val: i32,
    prev: Addr<Elem>,
    next: Addr<Elem>,
}

impl Elem {
    fn new(val: i32) -> Self {
        Self {
            val,
            prev: Addr::null(),
            next: Addr::null(),
        }
    }
}

impl ListLinks for Elem {
    type EnclosingType = Elem;

    fn get_links(&self) -> (Addr<Elem>, Addr<Elem>) {
        (self.prev, self.next)
    }

    fn set_prev(&mut self, addr: Addr<Elem>) {
        self.prev = addr;
    }

    fn set_next(&mut self, addr: Addr<Elem>) {
        self.next = addr;
    }
}

#[derive(Clone, Copy)]
struct Addr<T: Copy> {
    index: usize,
    phantom: PhantomData<T>,
}

impl<T: Copy> Addr<T> {
    fn new(index: usize) -> Self {
        assert!(index != 0);
        Self {
            index: index,
            phantom: PhantomData,
        }
    }

    fn null() -> Self {
        Self {
            index: 0,
            phantom: PhantomData,
        }
    }

    fn is_null(&self) -> bool {
        self.index == 0
    }
}

struct Arena<T: Copy> {
    arena: Vec<T>,
    free_idx: Vec<Addr<T>>,
}

impl<T: Copy> Arena<T> {
    fn new(initial_capacity: usize) -> Self {
        Self {
            arena: Vec::with_capacity(initial_capacity),
            free_idx: Vec::new(),
        }
    }

    fn alloc(&mut self, val: T) -> Addr<T> {
        if let Some(addr) = self.free_idx.pop() {
            assert!(!addr.is_null());
            self.arena[addr.index - 1] = val;
            addr
        } else {
            self.arena.push(val);
            Addr::new(self.arena.len())
        }
    }

    fn free(&mut self, addr: Addr<T>) {
        assert!(!addr.is_null());
        self.free_idx.push(addr);
    }

    fn get(&self, addr: Addr<T>) -> &T {
        assert!(!addr.is_null());
        &self.arena[addr.index - 1]
    }

    fn get_mut(&mut self, addr: Addr<T>) -> &mut T {
        assert!(!addr.is_null());
        &mut self.arena[addr.index - 1]
    }
}

trait ListLinks {
    type EnclosingType: Copy;
    fn get_links(&self) -> (Addr<Self::EnclosingType>, Addr<Self::EnclosingType>);
    fn set_prev(&mut self, addr: Addr<Self::EnclosingType>);
    fn set_next(&mut self, addr: Addr<Self::EnclosingType>);
}

struct List<T: Copy + ListLinks> {
    head: Addr<T>,
    tail: Addr<T>,
}

impl<T: Copy + ListLinks<EnclosingType = T>> List<T> {
    fn new() -> Self {
        Self {
            head: Addr::null(),
            tail: Addr::null(),
        }
    }

    fn push_front(&mut self, arena: &mut Arena<T>, addr: Addr<T>) {
        let elem = arena.get_mut(addr);
        elem.set_prev(Addr::null());
        elem.set_next(self.head);

        if !self.head.is_null() {
            arena.get_mut(self.head).set_prev(addr);
        }
        self.head = addr;
        if self.tail.is_null() {
            self.tail = addr
        }
    }

    fn push_back(&mut self, arena: &mut Arena<T>, addr: Addr<T>) {
        let elem = arena.get_mut(addr);
        elem.set_prev(self.tail);
        elem.set_next(Addr::null());

        if !self.tail.is_null() {
            arena.get_mut(self.tail).set_next(addr);
        }
        self.tail = addr;
        if self.head.is_null() {
            self.head = addr
        }
    }

    fn remove(&mut self, arena: &mut Arena<T>, addr: Addr<T>) {
        let (prev, next) = arena.get(addr).get_links();
        if prev.is_null() {
            self.head = next;
        } else {
            arena.get_mut(prev).set_next(next);
        }
        if next.is_null() {
            self.tail = prev;
        } else {
            arena.get_mut(next).set_prev(prev);
        }
        let elem = arena.get_mut(addr);
        elem.set_next(Addr::null());
        elem.set_prev(Addr::null());
    }
}
