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
in putnumln(fact(getnum()))