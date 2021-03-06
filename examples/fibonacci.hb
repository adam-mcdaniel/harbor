fn putnumln(n: int) -> void = do
    putnum(n);
    putchar('\n')
end in

fn fib(n: int) -> void =
    let a = 0,
        b = 1,
        c = 0
    in
        while n != 0 do
            putnumln(b);
            c = a;
            a = b;
            b = c + b;
            n = n - 1;
        end
in do
    fib(getnum())
end