use std::fmt;

#[derive(Debug, Clone)]
pub enum Elem<'i> {
    Text { prefix: &'i str, text: &'i str },
    Node { kind: Kind, children: Vec<Elem<'i>> },
}

pub fn walk<'a, 'i: 'a, F>(elem: &'a mut Elem<'i>, f: &mut F)
where
    F: FnMut(&mut Elem<'i>),
{
    f(elem);

    if let Elem::Node {
        ref mut children, ..
    } = *elem
    {
        for child in children {
            walk(child, f);
        }
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

pub type Kind = &'static str;

pub fn text<'i>(prefix: &'i str, text: &'i str) -> Elem<'i> {
    Elem::Text {
        prefix: prefix,
        text: text,
    }
}

pub fn node_vec<'i>(kind: Kind, children: Vec<Elem<'i>>) -> Elem<'i> {
    Elem::Node {
        kind: kind,
        children: children,
    }
}

#[macro_export]
macro_rules! node {
    ($kind:expr $(, $child:expr)*) => (node_vec($kind, vec![ $( $child ),* ]));
}
