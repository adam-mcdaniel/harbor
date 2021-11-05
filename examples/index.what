
let ptr = alloc(10, int) in do
    ptr[0] = 20;
    ptr[1] = 19;

    let x: &int = &ptr[2] in
        *x = 5;

    putnum(*ptr); putchar('\n');
    putnum(*(ptr + 1)); putchar('\n');
    putnum(ptr[2]); putchar('\n');
end