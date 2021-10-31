#include <stdio.h>
#include <stdlib.h>




int main() {
    printf("#include <stdio.h>\n#include <stdlib.h>\n\n#define TAPE_SIZE 30000\nvoid panic(char *msg) {\n    fprintf(stderr, \"panic: %%s\\n\", msg);\n    exit(-1);\n}\nvoid print_tape(unsigned int *tape, unsigned int size) { for (unsigned int i = 0; i < size; i++) { printf(\"%%u \", tape[i]); } printf(\"\\n\"); }\nunsigned int allocate(unsigned int *tape, unsigned int ptr, unsigned int *taken_cells) {\n    unsigned int requested_mem = tape[ptr];\n    unsigned int consecutive_zero_cells = 0;\n    for (int i=TAPE_SIZE-1; i>0; i--) {\n        if (taken_cells[i] == 0) {\n            consecutive_zero_cells++;\n        } else {\n            consecutive_zero_cells = 0;\n        }\n        if (consecutive_zero_cells >= requested_mem) {\n            unsigned int addr = i;\n            for (int j=0; j<requested_mem; j++) {\n                taken_cells[addr + j] = requested_mem - j;\n            }\n            return addr;\n        }\n    }\n    panic(\"no free memory\");\n}\nvoid free_mem(unsigned int *tape, unsigned int ptr, unsigned int *taken_cells) {\n    unsigned int address = tape[ptr];\n    unsigned int size = taken_cells[address];\n\n    for (int i=0; i<size; i++) {\n        taken_cells[address+i] = 0;\n        tape[address+i] = 0;\n    }\n}\nvoid zero(unsigned int *tape) {\n    for (int i = 0; i < TAPE_SIZE; i++) tape[i] = 0;\n}\nint main() {\n    unsigned int tape[TAPE_SIZE], taken_cells[TAPE_SIZE], ref_tape[256]; \n    unsigned int ptr = 0, ref_ptr = 0;\n    zero(tape);\n    zero(taken_cells);\n");

    while (1) {
        switch (getchar()) {
        case '+':
            printf("    tape[ptr]++;\n");
            break;
        case '-':
            printf("    tape[ptr]--;\n");
            break;
        case '<':
            printf("    ptr--;\n");
            break;
        case '>':
            printf("    ptr++;\n");
            break;
        case '[':
            printf("    while (tape[ptr]) {\n");
            break;
        case ']':
            printf("    }\n");
            break;
        case ',':
            printf("    tape[ptr] = getchar();\n");
            break;
        case '.':
            printf("    putchar(tape[ptr]);\n");
            break;
        case '#':
            printf("    scanf(\"%%d\", &tape[ptr]);\n");
            break;
        case '$':
            printf("    printf(\"%%d\", tape[ptr]);\n");
            break;
        case '*':
            printf("    ref_tape[ref_ptr++] = ptr; ptr = tape[ptr];\n");
            break;
        case '&':
            printf("    ptr = ref_tape[--ref_ptr];\n");
            break;
        case '?':
            printf("    tape[ptr] = allocate(tape, ptr, taken_cells);\n");
            break;
        case '!':
            printf("    free_mem(tape, ptr, taken_cells);\n");
            break;
        case EOF:
            // printf("print_tape(tape, 50);}\n");
            printf("}\n");

            exit(0);
        default:
            break;
        }
    }
}
