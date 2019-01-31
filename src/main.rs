macro_rules! nary_op_aux {
    ($r:ident $t:ty, $op:expr, $f:expr) => {
        for w in $f {
            $r.push($op(w))
        }
    };
    ($r:ident $t:ty, $op:expr, $f1:expr, $f2:tt $(, $ff:tt)*) => {
        nary_op_aux!(
            $r $t,
            |(a, b) $(, $ff)*| -> $t { $op(a, b $(, $ff)*) },
            $f1.zip($f2.iter()) $(, $ff)*);
    };
}

macro_rules! nary_op {
    ($r:ident $t:ty, $op:expr, $v1:ident $(, $vs:ident)*) => {nary_op_aux!($r $t, $op, $v1.iter() $(, $vs)*);};
}

#[allow(dead_code)]
fn kek(r: &mut Vec<i32>, a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>, d: &Vec<i32>) {
    nary_op!(
        r i32,
        (|a: &i32, b: &i32, c: &i32, d: &i32| -> i32 { *a + *b + *c - *d }),
        a,
        b,
        c,
        d
    )
}

// different types
#[allow(dead_code)]
fn if_then_else(r: &mut Vec<String>, a: &Vec<bool>, b: &Vec<i32>, c: &Vec<i32>) {
    fn ite(cond: &bool, r1: &i32, r2: &i32) -> String {
        if *cond {
            return (*r1).to_string();
        }
        return (*r2).to_string();
    }
    nary_op!(r String, ite, a, b, c)
}

fn main() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let c = vec![7, 8, 9];
    let d = vec![5, 5, 5];

    let mut r = vec![];
    kek(&mut r, &a, &b, &c, &d);
    println!("[{} {} {}]", r[0], r[1], r[2]);

    let mut r: Vec<String> = vec![];
    if_then_else(&mut r, &vec![true, false, true], &b, &c);
    println!("[{} {} {}]", r[0], r[1], r[2]);
}
