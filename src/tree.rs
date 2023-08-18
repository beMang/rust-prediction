use std::fmt::Display;

pub struct Tree<K, V> where K: PartialOrd {
    root: Link<K, V>
}

struct Node<K, V> where K: PartialOrd{
    key: K,
    value: V,
    left: Link<K, V>,
    right: Link<K, V>,
}

struct Link<K, V> where K: PartialOrd {
    content: Option<Box<Node<K, V>>>
}

impl<K, V> Tree<K, V> where K: PartialOrd {
    pub fn insert(&mut self, key: K, value: V) {
        self.root.insert(key, value);
    }

    pub fn get(& self, key: &K) -> Option<&V>{
        self.root.get(key)
    }

    pub fn get_mut(&mut self, key:&K) -> Option<&mut V> {
        self.root.get_mut(key)
    }

    pub fn new(key: K, value: V) -> Tree<K, V> {
        Tree { root: Link { content: Some(Box::new(Node { key, value, left: Link { content: None }, right: Link { content: None } })) } }
    }

    pub fn new_empty() -> Tree<K, V> {
        Tree { root: Link { content: None } }
    }
}

impl <K,V> Tree<K, V> where K: PartialOrd + Display, V: Display {
    pub fn print(&self) {
        self.root.print(0);
    }
}

impl <K, V> Link<K, V>  where K: PartialOrd {
    fn insert(&mut self, key: K, value: V) {
        match &mut self.content {
            None => self.content = Some(Box::new(Node{ key, value, left: Link { content: None }, right: Link { content: None }})),
            Some(node) => {
                if node.key > key {
                    node.left.insert(key, value);
                } else if node.key < key {
                    node.right.insert(key, value);
                } else {
                    println!("Already present !");
                }
            }
        };
    }

    fn get(& self, key: &K) -> Option<&V>{
        match &self.content {
            None => None,
            Some(node) => {
                if node.key==*key {
                    Some(&node.value)
                } else if node.key>*key {
                    node.left.get(key)
                } else {
                    node.right.get(key)
                }
            }
        }
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V>{
        match &mut self.content {
            None => None,
            Some(node) => {
                if node.key==*key {
                    Some(&mut node.value)
                } else if node.key>*key {
                    node.left.get_mut(key)
                } else {
                    node.right.get_mut(key)
                }
            }
        }
    }
}

impl <K, V> Link<K, V>  where K: PartialOrd + Display, V: Display {
    fn print(&self, level: i16) {
        for _i in 0..level {
            print!("-");
        }
        print!(">");
        match &self.content {
            None => println!("Leaf"),
            Some(node)=>{
                println!("{} => {}", node.key, node.value);
                node.left.print(level+1);
                node.right.print(level+1);
            }
        }
    }
}