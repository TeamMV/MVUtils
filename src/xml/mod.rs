use hashbrown::HashMap;

pub struct Document {
    root: Node,
}

impl Document {
    pub fn get_root(&self) -> &Node {
        &self.root
    }
}

pub struct Node {
    children: Option<Vec<Node>>,
    text: Option<String>,
    attributes: Attributes,
}

impl Node {
    pub fn has_children(&self) -> bool {
        self.children.is_some()
    }

    pub fn children(&self) -> Option<&[Node]> {
        self.children.as_deref()
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    pub fn get_text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.has(name)
    }

    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name)
    }

    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }
}

pub struct Attributes {
    internal: HashMap<String, String>,
}

impl Attributes {
    pub fn empty() -> Self {
        Self {
            internal: HashMap::new(),
        }
    }

    pub fn has(&self, name: &str) -> bool {
        self.internal.contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.internal.get(name).map(String::as_str)
    }
}
