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


fn putbool(b: bool) -> void = do
    if b do
        putchar('t');
        putchar('r');
        putchar('u');
        putchar('e')
    end;
        
    if !b do
        putchar('f');
        putchar('a');
        putchar('l');
        putchar('s');
        putchar('e')
    end;
end in

fn putboolln(b: bool) -> void = do
    putbool(b); putln();
end in

// Print a cartesian coordinate and a newline
fn putpointln(p: (int, int)) -> void = do
    putpoint(p); putln()
end in

fn test(a: (int, int), b: (char, char, (bool, char))) -> void = do
    putpointln(a);

    putchar('(');
    putchar(b.0);
    putchar(',');
    putchar(' ');
    putchar(b.1);
    putchar(',');
    putchar(' ');
    putchar('(');
    putbool(b.2.0);
    putchar(',');
    putchar(' ');
    putchar(b.2.1);
    putchar(')');
    putchar(')');
    putln()
end in

fn add(a: int, b: int) -> int = a + b in

fn move(p: (int, int), dx: int, dy: int) -> (int, int) = (p.0 + dx, p.1 + dy) in

// Do stuff!
let x = (1, 2, (3, 4)) in do
    let count = 5,
        points = alloc(count, (int, int)),
        i = 0
    in
        do
            putnumln(1);
            for (i = 0; i!=count; i++) do
                points[i] = (i + i, i * i);
            end;

            putnumln(2);
            for (i = 0; i!=count; i++) do
                points[i].putpointln;
            end;

            putnumln(3);
            test((5, 6), ('a', 'b', (true, 'c')));

            free(points)
        end
end
