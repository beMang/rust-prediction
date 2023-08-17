use std::fmt::Display;

pub struct Tree<K, V> where K: PartialOrd, V: Display {
    root: Link<K, V>
}

struct Node<K, V> where K: PartialOrd, V: Display{
    key: K,
    value: V,
    left: Link<K, V>,
    right: Link<K, V>,
}

struct Link<K, V> where K: PartialOrd, V: Display {
    content: Option<Box<Node<K, V>>>
}

impl<K, V> Tree<K, V> where K: PartialOrd, V: Display {
    pub fn insert(&mut self, key: K, value: V) {
        self.root.insert(key, value);
    }

    pub fn get(& self, key: K) -> Option<&V>{
        self.root.get(key)
    }

    pub fn print_value(&self) {
        self.root.print_value();
    }

    pub fn new(key: K, value: V) -> Tree<K, V> {
        Tree { root: Link { content: Some(Box::new(Node { key, value, left: Link { content: None }, right: Link { content: None } })) } }
    }
}

impl <K, V> Link<K, V>  where K: PartialOrd, V: Display {
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

    fn get(& self, key: K) -> Option<&V>{
        match &self.content {
            None => None,
            Some(node) => {
                if node.key==key {
                    Some(&node.value)
                } else if node.key>key {
                    node.left.get(key)
                } else {
                    node.right.get(key)
                }
            }
        }
    }

    fn print_value(&self) {
        match &self.content {
            None => println!("I'm a feuille"),
            Some(node)=>println!("Node value : {}", node.value)
        }
    }
}