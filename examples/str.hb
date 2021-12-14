fn pass() -> void = () in

// Get a line of user input and store it in a buffer
fn input(buf: &char) -> void =
    let ch = getchar(),
        i = 0
    in
        do
            for ((); ch != '\n' && ch != '\0'; i++) do
                buf[i] = ch;
                ch = getchar();
            end;
            buf[i] = '\0';
        end
in
    
// Print a newline
fn putln() -> void = putchar('\n') in

fn putnumln(n: int) -> void = do
    putnum(n); putln()
end in

fn streq(a: &char, b: &char) -> bool = do
    let i = 0, result = true in do
        for ((); a[i] == b[i] && a[i] != '\0'; i++) do
            pass()
        end;
        if a[i] != b[i] do
            result = false;
        end;
        result
    end
end in


fn putcharln(ch: char) -> void = do
    putchar(ch); putchar('\n')
end in

// Print a string
fn putstr(s: &char) -> void =
    let i = 0 in
        for ((); s[i] != '\0'; i++) do
            putchar(s[i]);
        end
in

// Print a string with a newline
fn putstrln(s: &char) -> void = do
    putstr(s);
    putln()
end in

fn fputstr(s: &char) -> void = do
    putstr(s);
    free(s)
end in

fn fputstrln(s: &char) -> void = do
    putstrln(s);
    free(s)
end in

    // Allocate some memory and read user input to it
    let buf = alloc(256, char) in do
        fputstrln("Hello world!");

        putchar('$');
        putchar(' ');
        input(buf);
        // Print it back and then print an exclamation point
        putchar('-');
        putchar('>');
        putchar(' ');

        fputstr(buf);

        putcharln('!');
    end