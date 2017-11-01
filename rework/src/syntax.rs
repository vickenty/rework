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

#[derive(Debug, Copy, Clone)]
pub enum QueryOp<'a> {
    Any,
    Many,
    Kind(&'a str),
    Size(usize, usize),
    More(usize),
    Query(&'a [QueryOp<'a>]),
    Nth(usize),
    Last,
    PrefixPat(&'a str),
    PrefixEq(&'a str),
    TextPat(&'a str),
    TextEq(&'a str),
}

#[macro_export]
macro_rules! query_op {
    (*) => ($crate::syntax::QueryOp::Any);
    (..) => ($crate::syntax::QueryOp::Many);
    ($kind:ident) => ($crate::syntax::QueryOp::Kind(stringify!($kind)));
    ([ = $len:expr ]) => ($crate::syntax::QueryOp::Size($len, $len));
    ([ < $len:expr ]) => ($crate::syntax::QueryOp::Size(0, $len));
    ([ > $len:expr ]) => ($crate::syntax::QueryOp::More($len));
    (( $( $tt:tt )+ )) => ($crate::syntax::QueryOp::Query(query!($( $tt )+)));
    ({:first}) => ($crate::syntax::QueryOp::Nth(0));
    ({:nth $e:expr}) => ($crate::syntax::QueryOp::Nth($e));
    ({:last}) => ($crate::syntax::QueryOp::Last);
    ({:prefix ~ $pat:expr}) => ($crate::syntax::QueryOp::PrefixPat($pat));
    ({:prefix = $pat:expr}) => ($crate::syntax::QueryOp::PrefixEq($pat));
    ({:text ~ $pat:expr}) => ($crate::syntax::QueryOp::TextPat($pat));
    ({:text = $pat:expr}) => ($crate::syntax::QueryOp::TextEq($pat));
}

#[macro_export]
macro_rules! query {
    ( $( $tt:tt )+ ) => ( &[ $( query_op!($tt) ),+ ] );
}

pub fn find<'a, 'i: 'a>(root: &'a mut Elem<'i>, query: &[QueryOp], f: &mut FnMut(&mut Elem<'i>)) {
    let (op, rest) = match query.split_first() {
        Some(v) => v,
        None => return f(root),
    };

    match *op {
        QueryOp::Any => for child in root.iter_mut() {
            find(child, rest, f);
        },

        QueryOp::Many => {
            for child in root.iter_mut() {
                find(child, rest, f);
            }
            for child in root.iter_mut() {
                find(child, query, f);
            }
        }
        QueryOp::Kind(query_kind) => for child in root.iter_mut() {
            if child.is(query_kind) {
                find(child, rest, f);
            }
        },
        QueryOp::Size(min, max) => if let Some(count) = root.len() {
            if min <= count && count < max {
                find(root, rest, f);
            }
        },
        QueryOp::More(min) => if let Some(count) = root.len() {
            if count > min {
                find(root, rest, f);
            }
        },
        QueryOp::Query(subquery) => {
            let mut found = false;
            find(root, subquery, &mut |_| found = true);
            if found {
                find(root, rest, f);
            }
        }
        QueryOp::Nth(n) => if let Some(children) = root.children_mut() {
            if let Some(child) = children.get_mut(n) {
                find(child, rest, f);
            }
        },
        QueryOp::Last => if let Some(children) = root.children_mut() {
            if let Some(child) = children.last_mut() {
                find(child, rest, f);
            }
        },
        QueryOp::PrefixPat(pat) => if root.prefix().map(|s| s.contains(pat)).unwrap_or(false) {
            find(root, rest, f);
        },
        QueryOp::PrefixEq(pat) => if root.prefix().map(|s| s == pat).unwrap_or(false) {
            find(root, rest, f);
        },
        QueryOp::TextPat(pat) => if root.text().map(|s| s.contains(pat)).unwrap_or(false) {
            find(root, rest, f);
        },
        QueryOp::TextEq(pat) => if root.text().map(|s| s == pat).unwrap_or(false) {
            find(root, rest, f);
        },
    }
}

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
    ($kind:expr $(, $child:expr)*) => ($crate::syntax::node_vec($kind, vec![ $( $child ),* ]));
}
