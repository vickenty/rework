parse! {
    item one {
        use foo;
    }
    item alias {
        use foo as bar;
    }
    item sublist {
        use foo::{bar, baz};
    }
    item sublist_alias {
        use foo::{bar as bar, baz as baz};
    }
    item abs {
        use ::foo;
    }
    item super_kw {
        use super::foo;
    }
    item self_kw {
        use self::foo;
    }
    item glob {
        use *;
    }
    item glob2 {
        use foo::*;
    }
    item glob3 {
        use ::*;
    }
}
