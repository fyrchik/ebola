macro_rules! new_bind {
    ($t:ident, $f:expr) => {
        let t = $f;
        println!("3. t = {}", t);
    };
}

macro_rules! existing_ident {
    ($t:ident, $f:expr) => {
        $t = $f;
        println!("1. t = {}", $t);
    };
}

fn main() {
    let mut t = 0;

    existing_ident!(t, 1);
    println!("2. t = {}", t);

    new_bind!(t, 2);
    println!("4. t = {}", t);
}
