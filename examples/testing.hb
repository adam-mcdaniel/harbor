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
    do
        // Allocate memory on the heap and manipulate it.
        let ptr: &int = alloc(10, int) in
            do
                *ptr = 3 - 2 + 4 * 5;
                putnumln(*ptr);
                free(ptr);
            end
        end;

        // Compute n numbers of the fibonacci sequence
        // based on user input
        putchar('$');
        putchar(' ');
        fib(getnum());
    end