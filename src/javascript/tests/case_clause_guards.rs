use crate::assert_js;

#[test]
fn guards_cause_badmatch_to_render() {
    assert_js!(
        r#"pub fn main(x, y) {
  case x {
    1 -> 1
    _ if y -> 0
  }
}
"#,
        r#""use strict";

export function main(x, y) {
  if (x === 1) {
    return 1;
  } else if (y) {
    return 0;
  } else {
    throw new Error("Bad match");
  }
}
"#
    );
}

#[test]
fn referencing_pattern_var() {
    assert_js!(
        r#"pub fn main(xs) {
  case xs {
    #(x) if x -> 1
    _ -> 0
  }
}
"#,
        r#""use strict";

export function main(xs) {
  if (xs[0]) {
    let x = xs[0];
    return 1;
  } else {
    return 0;
  }
}
"#
    );
}

#[test]
fn rebound_var() {
    assert_js!(
        r#"pub fn main() {
  let x = False
  let x = True
  case x {
    _ if x -> 1
    _ -> 0
  }
}
"#,
        r#""use strict";

export function main() {
  let x = false;
  let x$1 = true;
  if (x$1) {
    return 1;
  } else {
    return 0;
  }
}
"#
    );
}

#[test]
fn operator_wrapping_right() {
    assert_js!(
        r#"pub fn main(xs, y, z) {
  case xs {
    #(x) if x == { y == z } -> 1
    _ -> 0
  }
}
"#,
        r#""use strict";

export function main(xs, y, z) {
  if (xs[0] === (y === z)) {
    let x = xs[0];
    return 1;
  } else {
    return 0;
  }
}
"#
    );
}

#[test]
fn operator_wrapping_left() {
    assert_js!(
        r#"pub fn main(xs, y, z) {
  case xs {
    #(x) if { x == y } == z -> 1
    _ -> 0
  }
}
"#,
        r#""use strict";

export function main(xs, y, z) {
  if ((xs[0] === y) === z) {
    let x = xs[0];
    return 1;
  } else {
    return 0;
  }
}
"#
    );
}

#[test]
fn eq_scalar() {
    assert_js!(
        r#"pub fn main(xs, y) {
  case xs {
    #(x) if x == y -> 1
    _ -> 0
  }
}
"#,
        r#""use strict";

export function main(xs, y) {
  if (xs[0] === y) {
    let x = xs[0];
    return 1;
  } else {
    return 0;
  }
}
"#
    );
}

#[test]
fn not_eq_scalar() {
    assert_js!(
        r#"pub fn main(xs, y) {
  case xs {
    #(x) if x != y -> 1
    _ -> 0
  }
}
"#,
        r#""use strict";

export function main(xs, y) {
  if (xs[0] !== y) {
    let x = xs[0];
    return 1;
  } else {
    return 0;
  }
}
"#
    );
}

#[test]
fn tuple_index() {
    assert_js!(
        r#"pub fn main(x, xs: #(Bool, Bool, Bool)) {
  case x {
    _ if xs.2 -> 1
    _ -> 0
  }
}
"#,
        r#""use strict";

export function main(x, xs) {
  if (xs[2]) {
    return 1;
  } else {
    return 0;
  }
}
"#
    );
}

// #[test]
// fn not_eq_complex() {
//     assert_js!(
//         r#"pub fn main(xs, y) {
//   case xs {
//     #(x) if xs != y -> x
//     _ -> 0
//   }
// }
// "#,
//         r#""use strict";
//
// export function main(xs, y) {
//   if (!$equal(xs, y)) {
//     let x = xs[0];
//     return x;
//   } else {
//     return 0;
//   }
// }
// "#
//     );
// }

// #[test]
// fn eq_complex() {
//     assert_js!(
//         r#"pub fn main(xs, y) {
//   case xs {
//     #(x) if xs == y -> x
//     _ -> 0
//   }
// }
// "#,
//         r#""use strict";
//
// export function main(xs, y) {
//   if ($equal(xs, y)) {
//     let x = xs[0];
//     return x;
//   } else {
//     return 0;
//   }
// }
// "#
//     );
// }
