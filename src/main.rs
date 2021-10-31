use tf::{what, why, how};
use std::collections::BTreeMap;
use clap::{clap_app, crate_authors, crate_version, crate_description, AppSettings::ArgRequiredElseHelp};

fn compile_what(code: impl ToString) -> Result<String, what::Error>{
    let mut program = how::Program::default();
    use why::*;
    SP.set(why::TOTAL_REGISTERS, &mut program);
    FP.set(TOTAL_REGISTERS, &mut program);

    let w = what::parse(code.to_string())?;
    let w = w.compile(&BTreeMap::new(), &mut 0)?;
    match w.assemble(&mut program) {
        Ok(()) => {
            Ok(program.to_string())
        }
        Err(e) => {
            Err(what::Error::WhyError(e))
        }
    }
}

fn assemble_why(code: impl ToString) -> Result<String, why::Error>{
    let mut program = how::Program::default();
    use why::*;
    SP.set(why::TOTAL_REGISTERS, &mut program);
    FP.set(TOTAL_REGISTERS, &mut program);

    let mut scope = BTreeMap::new();
    scope.insert("putnum".to_string(), vec![
        Op::Putnum
    ]);
    scope.insert("putchar".to_string(), vec![
        Op::Putchar
    ]);
    scope.insert("getnum".to_string(), vec![
        Op::Getnum
    ]);
    scope.insert("getchar".to_string(), vec![
        Op::Getchar
    ]);
    scope.insert("dec".to_string(), vec![
        Op::Decrement(SP.deref().deref()),
        Op::Pop(TMP2)
    ]);
    scope.insert("inc".to_string(), vec![
        Op::Decrement(SP.deref().deref()),
        Op::Pop(TMP2)
    ]);

    let w = why::parse(code)?;
    w.assemble_with_scope(&scope, &mut program)?;
    Ok(program.to_string())
}

fn assemble_how(code: impl ToString) -> String {
    let code = code.to_string();
    let mut result = String::from("#include <stdio.h>\n#include <stdlib.h>\n\n#define TAPE_SIZE 30000\nvoid panic(char *msg) {\n    fprintf(stderr, \"panic: %s\\n\", msg);\n    exit(-1);\n}\nvoid print_tape(unsigned int *tape, unsigned int size) { for (unsigned int i = 0; i < size; i++) { printf(\"%u \", tape[i]); } printf(\"\\n\"); }\nunsigned int allocate(unsigned int *tape, unsigned int ptr, unsigned int *taken_cells) {\n    unsigned int requested_mem = tape[ptr];\n    unsigned int consecutive_zero_cells = 0;\n    for (int i=TAPE_SIZE-1; i>0; i--) {\n        if (taken_cells[i] == 0) {\n            consecutive_zero_cells++;\n        } else {\n            consecutive_zero_cells = 0;\n        }\n        if (consecutive_zero_cells >= requested_mem) {\n            unsigned int addr = i;\n            for (int j=0; j<requested_mem; j++) {\n                taken_cells[addr + j] = requested_mem - j;\n            }\n            return addr;\n        }\n    }\n    panic(\"no free memory\");\n}\nvoid free_mem(unsigned int *tape, unsigned int ptr, unsigned int *taken_cells) {\n    unsigned int address = tape[ptr];\n    unsigned int size = taken_cells[address];\n\n    for (int i=0; i<size; i++) {\n        taken_cells[address+i] = 0;\n        tape[address+i] = 0;\n    }\n}\nvoid zero(unsigned int *tape) {\n    for (int i = 0; i < TAPE_SIZE; i++) tape[i] = 0;\n}\nint main() {\n    unsigned int tape[TAPE_SIZE], taken_cells[TAPE_SIZE], ref_tape[256]; \n    unsigned int ptr = 0, ref_ptr = 0;\n    zero(tape);\n    zero(taken_cells);\n");
    for op in code.to_string().chars() {
        match op {
            '+' => result.push_str("tape[ptr]++;"),
            '-' => result.push_str("tape[ptr]--;"),
            '>' => result.push_str("ptr++;"),
            '<' => result.push_str("ptr--;"),
            '[' => result.push_str("while (tape[ptr]) {"),
            ']' => result.push_str("}"),
            ',' => result.push_str("tape[ptr] = getchar();"),
            '.' => result.push_str("putchar(tape[ptr]);"),
            '#' => result.push_str("scanf(\"%d\", &tape[ptr]);"),
            '$' => result.push_str("printf(\"%d\", tape[ptr]);"),
            '*' => result.push_str("ref_tape[ref_ptr++] = ptr; ptr = tape[ptr];"),
            '&' => result.push_str("ptr = ref_tape[--ref_ptr];"),
            '?' => result.push_str("tape[ptr] = allocate(tape, ptr, taken_cells);"),
            '!' => result.push_str("free_mem(tape, ptr, taken_cells);"),
            _ => {}
        }
    }
    result + "}"
}


fn main() {
    let matches = clap_app!(tf =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())

        (@group input =>
            (@arg what: -w --what "Compile What source")
            (@arg why:  -y --why  "Assemble Why assembler")
            (@arg how:  -h --how  "Assemble How brainfuck dialect\n(a 32-bit superset of brainfuck)")
        )
        (@arg FILE: +required "Input file")
        (@arg OUTPUT: -o +takes_value "Optionally specify output file")
    )
    .setting(ArgRequiredElseHelp)
    .get_matches();


    if let Some(input_file) = matches.value_of("FILE") {
        // Get the contents of the input file
        if let Ok(contents) = std::fs::read_to_string(input_file) {
            let compile_result = if matches.is_present("why") {
                match assemble_why(contents) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("{}", e);
                        return;
                    }
                }
            } else if matches.is_present("how") {
                assemble_how(contents)
            } else {
                match compile_what(contents) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("{}", e);
                        return;
                    }
                }
            };

            if let Some(output_file) = matches.value_of("OUTPUT") {
                std::fs::write(output_file, compile_result).unwrap();
            } else {
                println!("{}", compile_result);
            }
        } else {
            eprintln!("Could not read input file");
        }
    } else {
        eprintln!("No input file specified");
    }
    


    // } else if matches.is_present("how") {
        // compile(&cwd, &input_file, contents, TS)

    // Same as previous examples...

    /*
    parse(r#"
    let double =
        frame (1) -> 1 do
            let x = ($FP @) in
                x x +
            end
        end,

        fact = 
            frame (1) -> 1 do
                let n = ($FP) in
                    #1
                    
                    let acc = ($FP #1 +) in

                        while (n@) do
                            n@ acc@ * acc=
                            n dec
                        end

                        acc@
                    end
                    
                    dump %int
                end
            end,

        putendl = 
            (#10 putchar),
    in
        #5 fact putnum
        putendl
    end
    "#).unwrap().assemble_with_scope(&scope, &mut program);
    */


    // let asm = Expr::Let("fact".to_string(),
    //     Type::Function(vec![Type::Integer], Box::new(Type::Integer)),
    //     Box::new(Expr::Function(
    //         vec![("x".to_string(), Type::Integer)],
    //         Type::Integer,
    //         Box::new(Expr::Let("y".to_string(),
    //             Type::Integer,
    //             Box::new(Expr::Integer(10)),
    //             Box::new(Expr::Let("z".to_string(),
    //                 Type::Integer,
    //                 Box::new(Expr::Integer(2)),
    //                 Box::new(Expr::Mul(
    //                     Box::new(Expr::Add(
    //                         Box::new(Expr::Variable("x".to_string())),
    //                         Box::new(Expr::Variable("y".to_string())),
    //                     )),
    //                     Box::new(Expr::Variable("z".to_string())),
    //                 ))
    //             ))
    //         ))
    //     )),
    //     Box::new(Expr::Block(vec![
    //         // Expr::Call(
    //         //     "fact".to_string(),
    //         //     vec![Expr::Integer(18)]
    //         // )
    //         Expr::Putnum(
    //             Box::new(Expr::Add(
    //                 Box::new(Expr::Call(
    //                     "fact".to_string(),
    //                     vec![Expr::Integer(5)]
    //                 )),
    //                 Box::new(Expr::Call(
    //                     "fact".to_string(),
    //                     vec![Expr::Integer(5)]
    //                 )),
    //             ))
    //         ),
    //         Expr::Putchar(Box::new(Expr::Character('\n')))
    //     ]))
    // )

    // println!("{}", compile_what(r#"
    // "#).unwrap());



    // .unwrap()
    // .compile(&BTreeMap::new(), &mut 0).unwrap();
    // asm.assemble(&mut program).unwrap();
    
//     parse(r#"
//     "#).unwrap().assemble_with_scope(&scope, &mut program).unwrap();


    // push_str("test!", &mut program);
    
    // Op::PushLiteral(Literal(6)).assemble(&mut program);
    // Op::Alloc.assemble(&mut program);

    // Op::StoreAt(R0, 1).assemble(&mut program);

    // Op::LoadFrom(R0, 1).assemble(&mut program);
    // Op::Store(6).assemble(&mut program);
    // Op::LoadFrom(R0, 1).assemble(&mut program);
    
    // Op::PushLiteral(Literal(26)).assemble(&mut program);
    // Op::PushLiteral(Literal(25)).assemble(&mut program);
    // Op::Eq.assemble(&mut program);

    /*
    Op::PushLiteral(Literal(33)).assemble(&mut program);
    Op::PushLiteral(Literal(63)).assemble(&mut program);
    Op::Frame(2, 1, vec![
        Op::LoadFrom(FP.deref(), 2),
        Op::Frame(2, 0, vec![
            Op::LoadFrom(FP.deref().offset(0), 1),
            Op::Putchar,
            Op::LoadFrom(FP.deref().offset(1), 1),
            Op::Putchar,
        ]),

        Op::LoadFrom(FP.deref().offset(1), 1),
        Op::Putchar,
        Op::LoadFrom(FP.deref().offset(0), 1),
        Op::Putchar,
        Op::LoadFrom(FP.deref().offset(0), 1),
    ]).assemble(&mut program);
    Op::Putchar.assemble(&mut program);
    */
    
    /*
    Op::PushLiteral(Literal(5)).assemble(&mut program)?;
    Op::PushLiteral(Literal(10)).assemble(&mut program)?;
    Op::LoadFrom(FP, 1).assemble(&mut program)?;
    Op::Putnum.assemble(&mut program)?;
    Op::PushLiteral(Literal(10)).assemble(&mut program)?;
    Op::Putchar.assemble(&mut program)?;
    Op::Frame(2, 1, vec![
        Op::LoadFrom(FP, 1),
        Op::Putnum,
        Op::PushLiteral(Literal(10)),
        Op::Putchar,

        Op::LoadFrom(FP.deref(), 2),
        Op::Frame(2, 1, vec![
            Op::LoadFrom(FP, 1),
            Op::Putnum,
            Op::PushLiteral(Literal(10)),
            Op::Putchar,

            Op::LoadFrom(FP.deref(), 1),
            Op::LoadFrom(FP.deref().offset(1), 1),
            Op::Add,
        ]),
        
        Op::LoadFrom(FP, 1),
        Op::Putnum,
        Op::PushLiteral(Literal(10)),
        Op::Putchar,

        Op::LoadFrom(FP.deref().offset(1), 1),
        Op::Add,
    ]).assemble(&mut program)?;
    Op::LoadFrom(FP, 1).assemble(&mut program)?;
    Op::Putnum.assemble(&mut program)?;
    Op::PushLiteral(Literal(10)).assemble(&mut program)?;
    Op::Putchar.assemble(&mut program)?;

    Op::PushLiteral(Literal(10)).assemble(&mut program)?;
    Op::Putchar.assemble(&mut program)?;
    Op::Putnum.assemble(&mut program)?;
    Op::PushLiteral(Literal(10)).assemble(&mut program)?;
    Op::Putchar.assemble(&mut program)?;
    */


    

    // Op::PushLiteral(Literal(20)).assemble(&mut program)?;
    // Op::Let(
    //     String::from("double"),
    //     vec![
    //         Op::Frame(1, 1, vec![
    //             Op::Let(
    //                 String::from("x"),
    //                 vec![
    //                     Op::LoadFrom(FP.deref(), 1),
    //                 ],
    //                 vec![
    //                     Op::Macro(String::from("x")),
    //                     Op::Macro(String::from("x")),
    //                     Op::Add
    //                 ]
    //             )
    //         ])
    //     ],
    //     vec![
    //         Op::Macro(String::from("double")),
    //     ]
    // ).assemble(&mut program)?;
    // Op::Putnum.assemble(&mut program)?;
    // Op::PushLiteral(Literal(10)).assemble(&mut program)?;
    // Op::Putchar.assemble(&mut program)?;
    
    // Op::Putchar.assemble(&mut program);

    // Op::While(vec![
    //     Op::Getchar,
    //     Op::StoreAt(R0, 1),
    //     Op::LoadFrom(R0, 1),
    // ], vec![
    //     Op::LoadFrom(R0, 1),
    //     Op::PushLiteral(Literal(10)),
    //     Op::Sub,
    //     Op::StoreAt(R1, 1),


    //     Op::If(vec![
    //         Op::LoadFrom(R1, 1),
    //     ], vec![
    //         Op::LoadFrom(R0, 1),
    //         Op::PushLiteral(Literal(1)),
    //         Op::Add,
    //         Op::Putchar
    //     ]),
    //     Op::If(vec![
    //         Op::LoadFrom(R1, 1),
    //         Op::Not,
    //     ], vec![
    //         Op::Putchar
    //     ]),
    // ]).assemble(&mut program);


    // Op::PushLiteral(Literal(1)).assemble(&mut program);
    // Op::StoreAt(R0, 1).assemble(&mut program);
    // Op::PushLiteral(Literal(1)).assemble(&mut program);
    // Op::StoreAt(R1, 1).assemble(&mut program);
    // Op::If(vec![
    //     Op::LoadFrom(R0, 1),
    // ], vec![
    //     Op::If(vec![
    //         Op::LoadFrom(R1, 1),
    //     ], vec![
    //         Op::PushLiteral(Literal(33)),
    //         Op::Putchar,
    //     ]),
    //     // Op::PushLiteral(Literal(63)),
    //     Op::Putchar,
    // ]).assemble(&mut program);




    // Op::While(vec![
    //     Op::LoadFrom(R0, 1),
    // ], vec![
    //     Op::PushLiteral(Literal(10)),
    //     Op::StoreAt(R1, 1),
    //     Op::While(vec![
    //         Op::LoadFrom(R1, 1),
    //     ], vec![
    //         Op::PushLiteral(Literal(33)),
    //         Op::Putchar,
    //         Op::Decrement(R1),
    //     ]),
    //     Op::PushLiteral(Literal(10)),
    //     Op::Putchar,
    //     Op::Decrement(R0),
    // ]).assemble(&mut program);

    // Op::PushAddress(Address(20)).assemble(&mut program);
    // Op::Store(6).assemble(&mut program);

    // program.begin_loop();
    // program.minus(1);
    // program.end_loop();
    // program.plus(5);

    // program.deref();
    // program.begin_loop();
    // program.put();
    // program.right(1);
    // program.end_loop();
    // program.refer();

    // program.comment("pushing 33");
    // Op::PushLiteral(Literal(33)).assemble(&mut program);
    // program.comment("pushing 34");
    // Op::PushLiteral(Literal(34)).assemble(&mut program);
    // program.comment("popping into R0");
    // Op::Pop(R0).assemble(&mut program);
    // program.comment("printing R0");
    // R0.put(&mut program);
    // program.comment("popping into R0");
    // Op::Pop(R0).assemble(&mut program);
    // program.comment("printing R0");
    // R0.put(&mut program);
    
    


    // program.comment("done popping");

    // R0.set(33, &mut program);
    
    // let x = Location::Address(Address(11));
    // let y = Location::Address(Address(12));

    // prog.comment("setting x to 5");
    // x.set(5, &mut prog);
    // prog.comment("setting y to 10");
    // y.set(10, &mut prog);
    // copy_cell(
    //     Location::Deref(Box::new(x)),
    //     y,
    //     &mut prog
    // );

    // println!("{}", prog);
}