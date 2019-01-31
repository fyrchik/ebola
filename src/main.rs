macro_rules! nary_op {
    ($r:ident, $op:expr, $f:expr) => {
        for w in $f {
            $r.push($op(w))
        }
    };
    ($r:ident, $op:expr, $f1:expr, $f2:tt $(, $ff:tt)*) => {
        nary_op!(
            $r,
            |(a, b) $(, $ff)*| -> i32 { $op(a, b $(, $ff)*) },
            $f1.zip($f2.iter()) $(, $ff)*);
    };
}

#[allow(dead_code)]
fn kek(r: &mut Vec<i32>, a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>, d: &Vec<i32>) {
    nary_op!(
        r,
        |a: &i32, b: &i32, c: &i32, d: &i32| -> i32 { *a + *b + *c - *d },
        a.iter(),
        b,
        c,
        d
    )
}

fn main() {
    let mut r = vec![];
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let c = vec![7, 8, 9];
    let d = vec![5, 5, 5];

    kek(&mut r, &a, &b, &c, &d);

    println!("[{} {} {}]", r[0], r[1], r[2]);
}
