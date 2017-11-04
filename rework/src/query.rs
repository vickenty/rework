use syntax::Elem;

#[derive(Debug, Clone)]
pub enum QueryOp<'a> {
    Any,
    Many,
    Kind(&'a str),
    Size(usize, usize),
    More(usize),
    Query(&'a [QueryOp<'a>]),
    QueryVec(Vec<QueryOp<'a>>),
    Nth(usize),
    Last,
    PrefixPat(&'a str),
    PrefixEq(&'a str),
    TextPat(&'a str),
    TextEq(&'a str),
}

#[macro_export]
macro_rules! query_op {
    (*) => ($crate::query::QueryOp::Any);
    (..) => ($crate::query::QueryOp::Many);
    ($kind:ident) => ($crate::query::QueryOp::Kind(stringify!($kind)));
    ([ = $len:expr ]) => ($crate::query::QueryOp::Size($len, $len + 1));
    ([ < $len:expr ]) => ($crate::query::QueryOp::Size(0, $len));
    ([ > $len:expr ]) => ($crate::query::QueryOp::More($len));
    (( $( $tt:tt )+ )) => ($crate::query::QueryOp::Query(query!($( $tt )+)));
    ({:first}) => ($crate::query::QueryOp::Nth(0));
    ({:nth $e:expr}) => ($crate::query::QueryOp::Nth($e));
    ({:last}) => ($crate::query::QueryOp::Last);
    ({:prefix ~ $pat:expr}) => ($crate::query::QueryOp::PrefixPat($pat));
    ({:prefix = $pat:expr}) => ($crate::query::QueryOp::PrefixEq($pat));
    ({:text ~ $pat:expr}) => ($crate::query::QueryOp::TextPat($pat));
    ({:text = $pat:expr}) => ($crate::query::QueryOp::TextEq($pat));
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
        QueryOp::QueryVec(ref subquery) => {
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
