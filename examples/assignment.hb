// Print a newline
fn putln() -> void = do
    putchar('\n')
end in

// Print a number, and a newline
fn putnumln(n: int) -> void = do
    putnum(n); putln()
end in

fn inc(n: &int) -> &int = do
    *n += 1;
    n
end in

fn square(n: &int) -> &int = do
    *n *= *n;
    n
end in

let x = 5,
    ptr = alloc(int)
in do
    x *= 2;

    *ptr = x;
    ptr->putnumln;
    ptr[0] /= 3;
    ptr[0].putnumln
end