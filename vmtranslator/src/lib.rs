
pub fn translate(source: &str) -> String {
    let mut output = String::new();
    let mut label_count: u16 = 0;

    output.push_str("@256\nD=A\n@SP\nM=D\n");
    output.push_str("@3\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    output.push_str("@4\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    output.push_str("@HALT\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    output.push_str("@SP\nA=M\nM=0\n@SP\nM=M+1\n");
    output.push_str("@SP\nA=M\nM=0\n@SP\nM=M+1\n");
    output.push_str("@SP\nA=M\nM=0\n@SP\nM=M+1\n");
    output.push_str("@SP\nA=M\nM=0\n@SP\nM=M+1\n");
    output.push_str("@SP\nD=M\n@5\nD=D-A\n@2\nD=D-A\n@ARG\nM=D\n");
    output.push_str("@SP\nD=M\n@LCL\nM=D\n");
    output.push_str("@Add\n0;JMP\n");
    output.push_str("(HALT)\n@HALT\n0;JMP\n");

    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") { continue; }
        let asm = match line {
            "add" => translate_add(),
            "sub" => translate_sub(),
            "neg" => translate_neg(),
            "eq"  => translate_cmp("eq", &mut label_count),
            "lt"  => translate_cmp("lt", &mut label_count),
            "gt"  => translate_cmp("gt", &mut label_count),
            "and" => translate_and(),
            "or"  => translate_or(),
            "not" => translate_not(),
            _ if line.starts_with("push")     => translate_push(line),
            _ if line.starts_with("pop")      => translate_pop(line),
            _ if line.starts_with("call")     => {
                let parts: Vec<&str> = line.split_whitespace().collect();
                translate_call(parts[1], parts[2], &mut label_count)
            }
            _ if line.starts_with("function") => {
                let parts: Vec<&str> = line.split_whitespace().collect();
                translate_function(parts[1], parts[2])
            }
            "return" => translate_return(),
            _        => panic!("unknown command: {}", line),
        };
        output.push_str(&asm);
    }

    output
}

fn translate_push(line: &str) -> String {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let i: u16 = parts[2].parse().unwrap();
    match parts[1] {
        "constant"                              => format!("@{i}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"),
        "local" | "argument" | "this" | "that" => translate_push_segment(parts[1], parts[2]),
        "temp"                                  => translate_push_fixed(5, i),
        "pointer"                               => translate_push_fixed(3, i),
        "static"                                => translate_push_fixed(16, i),
        _ => panic!("unknown push segment: {}", parts[1]),
    }
}

fn translate_pop(line: &str) -> String {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let i: u16 = parts[2].parse().unwrap();
    match parts[1] {
        "local" | "argument" | "this" | "that" => translate_pop_segment(parts[1], parts[2]),
        "temp"                                  => translate_pop_fixed(5, i),
        "pointer"                               => translate_pop_fixed(3, i),
        "static"                                => translate_pop_fixed(16, i),
        _ => panic!("unknown pop segment: {}", parts[1]),
    }
}

fn translate_add() -> String {
    "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=D+M\n@SP\nM=M+1\n".to_string()
}

fn translate_sub() -> String {
    "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M-D\n@SP\nM=M+1\n".to_string()
}

fn translate_neg() -> String {
    "@SP\nM=M-1\nA=M\nM=-M\n@SP\nM=M+1\n".to_string()
}

fn translate_cmp(op: &str, label_count: &mut u16) -> String {
    let jump = match op {
        "eq" => "JEQ",
        "lt" => "JLT",
        "gt" => "JGT",
        _    => panic!("unknown op: {}", op),
    };

    let true_label  = format!("CMP_TRUE_{}", label_count);
    let end_label   = format!("CMP_END_{}", label_count);
    *label_count += 1;

    format!(
        "@SP\nM=M-1\nA=M\nD=M\n\
         @SP\nM=M-1\nA=M\nD=M-D\n\
         @{true_label}\nD;{jump}\n\
         @SP\nA=M\nM=0\n\
         @{end_label}\n0;JMP\n\
         ({true_label})\n\
         @SP\nA=M\nM=-1\n\
         ({end_label})\n\
         @SP\nM=M+1\n"
    )
}

fn translate_and() -> String {
    "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=D&M\n@SP\nM=M+1\n".to_string()
}

fn translate_or() -> String {
    "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=D|M\n@SP\nM=M+1\n".to_string()
}

fn translate_not() -> String {
    "@SP\nM=M-1\nA=M\nM=!M\n@SP\nM=M+1\n".to_string()
}

fn segment_base(segment: &str) -> &str {
    match segment {
        "local"    => "LCL",
        "argument" => "ARG",
        "this"     => "THIS",
        "that"     => "THAT",
        _          => panic!("unknown segment: {}", segment),
    }
}

fn translate_push_segment(segment: &str, i: &str) -> String {
    let base = segment_base(segment);
    format!(
        "@{i}\nD=A\n@{base}\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
    )
}

fn translate_pop_segment(segment: &str, i: &str) -> String {
    let base = segment_base(segment);
    format!(
        "@{i}\nD=A\n@{base}\nD=D+M\n@R13\nM=D\n@SP\nM=M-1\nA=M\nD=M\n@R13\nA=M\nM=D\n"
    )
}

fn translate_push_fixed(base: u16, i: u16) -> String {
    let addr = base + i;
    format!("@{addr}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n")
}

fn translate_pop_fixed(base: u16, i: u16) -> String {
    let addr = base + i;
    format!("@SP\nM=M-1\nA=M\nD=M\n@{addr}\nM=D\n")
}

fn translate_call(function: &str, n_args: &str, label_count: &mut u16) -> String {
    let return_label = format!("{function}_RETURN_{label_count}");
    *label_count += 1;
    format!(
        "@{return_label}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\
         @LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\
         @ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\
         @THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\
         @THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\
         @SP\nD=M\n@5\nD=D-A\n@{n_args}\nD=D-A\n@ARG\nM=D\n\
         @SP\nD=M\n@LCL\nM=D\n\
         @{function}\n0;JMP\n\
         ({return_label})\n"
    )
}

fn translate_function(function: &str, n_locals: &str) -> String {
    let n: u16 = n_locals.parse().unwrap();
    let mut out = format!("({function})\n");
    for _ in 0..n {
        out.push_str("@SP\nA=M\nM=0\n@SP\nM=M+1\n");
    }
    out
}

fn translate_return() -> String {
    "@LCL\nD=M\n@R14\nM=D\n\
     @R14\nD=M\n@5\nD=D-A\nA=D\nD=M\n@R15\nM=D\n\
     @SP\nM=M-1\nA=M\nD=M\n@ARG\nA=M\nM=D\n\
     @ARG\nD=M\n@SP\nM=D+1\n\
     @R14\nD=M\n@1\nD=D-A\nA=D\nD=M\n@THAT\nM=D\n\
     @R14\nD=M\n@2\nD=D-A\nA=D\nD=M\n@THIS\nM=D\n\
     @R14\nD=M\n@4\nD=D-A\nA=D\nD=M\n@LCL\nM=D\n\
     @R14\nD=M\n@3\nD=D-A\nA=D\nD=M\n@ARG\nM=D\n\
     @R15\nA=M\n0;JMP\n".to_string()
}