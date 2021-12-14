// Print a newline
fn putln() -> void = do
    putchar('\n')
end in

// Print a number, and a newline
fn putnumln(n: int) -> void = do
    putnum(n); putln()
end in

// Print a cartesian coordinate
fn putpoint(p: (int, int)) -> void = do
    let x = p.0,
        y = p.1
    in do
        putchar('(');
        putnum(x);
        putchar(',');
        putchar(' ');
        putnum(y);
        putchar(')');
    end
end in

// Print a cartesian coordinate and a newline
fn putpointln(p: (int, int)) -> void = do
    putpoint(p); putln()
end in

// Move a point with a change in X and a change in Y
fn move(p: (int, int), dx: int, dy: int) -> (int, int) =
    (p.0 + dx, p.1 + dy)
in

fn inc(n: &int) -> &int = do *n += 1; n end in
fn square(n: &int) -> &int = do *n *= *n; n end in

let n = 255,
    tup = alloc(1, (int, int))
in do
    *tup = (5, 6);
    tup->move(1, 2).putpointln;

    &n.inc.square->putnumln;
end