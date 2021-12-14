fn putnumln(n: int) -> void = 
    do
        putnum(n);
        putchar('\n');
    end
in
fn fact(n: int) -> int =
    let acc: int = 1 in
        do
            while n != 0 do
                acc = acc * n;
                n = n - 1;
            end;
            acc;
        end
    end
in
fn fib(n: int) -> void =
    let a: int = 0 in
        let b: int = 1 in
            let c: int = 0 in
                while n != 0 do
                    putnumln(b);
                    c = a;
                    a = b;
                    b = c + b;
                    n = n - 1;
                end
            end
        end
    end
in
fn test(n: int) -> int =
    let a: int = 10 in
        do
            let b: int = 1 in
                b
            end;
            a + n;
        end
    end
in
    do
        putchar('$');
        putchar(' ');
        putnumln(fact(getnum()));

        let a: &int = alloc(10, int) in
            let ptr: &int = a in
                do
                    *a = 10;
                    putnumln(*ptr);
                    free(a);
                    putnumln(*ptr);
                end
            end
        end;
    end