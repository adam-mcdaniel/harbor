
fn putnumln(n: int) -> void =
    do 
        putnum(n);
        putchar('\n');
    end
in

// Invalid type signature for return type
fn test(n: int) -> void =
    n + 1
in
    do
        // Undefined variable `f`
        // Invalid syntax
        putnumln(f(5);
    end