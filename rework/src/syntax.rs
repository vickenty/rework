use std::fmt;
use std::iter::Extend;

pub type Kind = &'static str;

#[derive(Debug, Clone, Hash)]
pub enum Elem<'i> {
    Text { prefix: &'i str, text: &'i str },
    Node { kind: Kind, children: Vec<Elem<'i>> },
}

impl<'i> Elem<'i> {
    pub fn is(&self, kind: &str) -> bool {
        match *self {
            Elem::Node {
                kind: node_kind, ..
            } => node_kind == kind,
            _ => false,
        }
    }

    pub fn kind(&self) -> Option<&str> {
        match *self {
            Elem::Node { kind, .. } => Some(kind),
            _ => None,
        }
    }

    pub fn len(&self) -> Option<usize> {
        match *self {
            Elem::Node { ref children, .. } => Some(children.len()),
            _ => None,
        }
    }

    pub fn children(&self) -> Option<&[Elem<'i>]> {
        match *self {
            Elem::Node { ref children, .. } => Some(children),
            _ => None,
        }
    }

    pub fn children_mut(&mut self) -> Option<&mut Vec<Elem<'i>>> {
        match *self {
            Elem::Node {
                ref mut children, ..
            } => Some(children),
            _ => None,
        }
    }

    pub fn iter(&self) -> ::std::slice::Iter<Elem<'i>> {
        match *self {
            Elem::Node { ref children, .. } => children.iter(),
            _ => [].iter(),
        }
    }

    pub fn iter_mut(&mut self) -> ::std::slice::IterMut<Elem<'i>> {
        match *self {
            Elem::Node {
                ref mut children, ..
            } => children.iter_mut(),
            _ => [].iter_mut(),
        }
    }

    pub fn prefix(&self) -> Option<&str> {
        match *self {
            Elem::Text { prefix, .. } => Some(prefix),
            _ => None,
        }
    }

    pub fn text(&self) -> Option<&str> {
        match *self {
            Elem::Text { text, .. } => Some(text),
            _ => None,
        }
    }

    pub fn push(&mut self, elem: Elem<'i>) {
        self.children_mut()
            .expect("push() called on text node")
            .push(elem);
    }
}

impl<'i> Extend<Elem<'i>> for Elem<'i> {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = Elem<'i>>,
    {
        self.children_mut()
            .expect("extend() called on text node")
            .extend(iter)
    }
}

impl<'i> fmt::Display for Elem<'i> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Elem::Text { prefix, text } => write!(fmt, "{}{}", prefix, text),
            Elem::Node { ref children, .. } => {
                for child in children {
                    write!(fmt, "{}", child)?;
                }
                Ok(())
            }
        }
    }
}

pub fn walk<'a, 'i: 'a, F>(elem: &'a mut Elem<'i>, f: &mut F)
where
    F: FnMut(&mut Elem<'i>),
{
    f(elem);

    if let Some(children) = elem.children_mut() {
        for child in children {
            walk(child, f);
        }
    }
}

pub fn node_vec<'i>(kind: Kind, children: Vec<Elem<'i>>) -> Elem<'i> {
    Elem::Node {
        kind: kind,
        children: children,
    }
}

pub fn text<'i>(prefix: &'i str, text: &'i str) -> Elem<'i> {
    Elem::Text {
        prefix: prefix,
        text: text,
    }
}

#[macro_export]
macro_rules! node {
    ($kind:expr $(, $child:expr)*) => ($crate::syntax::node_vec($kind, vec![ $( $child ),* ]));
}

pub fn append<'i, T>(mut xs: Vec<Elem<'i>>, x: T) -> Vec<Elem<'i>>
where
    T: IntoIterator<Item = Elem<'i>> + 'i,
{
    xs.extend(x);
    xs
}

pub fn prepend<'i>(x: Elem<'i>, mut xs: Vec<Elem<'i>>) -> Vec<Elem<'i>> {
    xs.insert(0, x);
    xs
}
