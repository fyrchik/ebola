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

// nary_op! macro is a n-ary zipWith
#[allow(dead_code)]
macro_rules! nary_op {
    ($r:ident $t:ty, $op:expr, $v1:ident $(, $vs:ident)*) => {
        {
            $r.truncate(0);
            nary_op_aux!($r $t, $op, $v1.iter() $(, $vs)*);
        }
    };
}

#[allow(dead_code)]
macro_rules! vectorize {
    ($r:ident, $op:expr, $name:ident $res:ty $(, $narg:ident $arg:ty)*) => {
        #[allow(dead_code)]
        fn $r($name: &mut Vec<$res> $(, $narg: &Vec<$arg>)*) {
            $name.truncate(0);
            nary_op!($name $res, $op $(, $narg)*);
        }
    }
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

#[allow(dead_code)]
macro_rules! compose12 {
    ($t:ident, $f:expr, $g1:expr , $g2:expr, $($arg:expr)*) => {
        let a1 = $g1($(&$arg),+);
        let a2 = $g2($(&$arg),+);
        let mut t = $f(&a1, &a2);
        println!("t = {}", t);
    };
}

#[allow(dead_code)]
macro_rules! compose12mut {
    ($t:ident, $f:expr, $g1:expr , $g2:expr, $($arg:expr)*) => {
        let a1 = $g1($(&$arg),+);
        let a2 = $g2($(&$arg),+);
        $t = $f(&a1, &a2);
        println!("t = {}", $t);
    };
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

    fn sum(a: &i32, b: &i32) -> i32 {
        return *a + *b;
    }
    vectorize!(vect_sum, sum, r i32, a i32, b i32);

    let mut r: Vec<i32> = vec![];
    vect_sum(&mut r, &a, &b);
    println!("[{} {} {}]", r[0], r[1], r[2]);

    fn sum2(a: &i32, b: &i32) -> i32 {
        *a * *a + *b * *b
    }

    let mut t = 0;

    compose12mut!(t, sum, sum, sum2, 1 2);
    println!("t = {}", t);

    compose12!(t, sum, sum, sum2, 2 3);
    println!("t = {}", t);
}
