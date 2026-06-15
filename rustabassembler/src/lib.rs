use std::collections::HashMap;

pub fn assemble(source: &str) -> Vec<u16> {
    let mut table = first_scan(source);
    let program = second_scan(source, &mut table);
    program
}

fn first_scan(source: &str) -> HashMap<String, u16> {
    let mut table = populate_table();
    let mut line_number: u16 = 0;
    
    for line in source.lines() {

        let line = line.trim();
        if line.is_empty() || line.starts_with("//") { continue; }

        if line.starts_with('(') && line.ends_with(')') {
            let label = &line[1..line.len()-1];
            table.insert(label.to_string(), line_number);
            continue;
        }

        line_number += 1;
    }
    table
}

fn populate_table() -> HashMap<String, u16> {
    let mut table = HashMap::new();
    table.insert("R0".to_string(),     0);
    table.insert("R1".to_string(),     1);
    table.insert("R2".to_string(),     2);
    table.insert("R3".to_string(),     3);
    table.insert("R4".to_string(),     4);
    table.insert("R5".to_string(),     5);
    table.insert("R6".to_string(),     6);
    table.insert("R7".to_string(),     7);
    table.insert("R8".to_string(),     8);
    table.insert("R9".to_string(),     9);
    table.insert("R10".to_string(),   10);
    table.insert("R11".to_string(),   11);
    table.insert("R12".to_string(),   12);
    table.insert("R13".to_string(),   13);
    table.insert("R14".to_string(),   14);
    table.insert("R15".to_string(),   15);
    table.insert("SCREEN".to_string(), 16384);
    table.insert("KBD".to_string(),    24576);
    table.insert("SP".to_string(),     0);
    table.insert("LCL".to_string(),    1);
    table.insert("ARG".to_string(),    2);
    table.insert("THIS".to_string(),   3);
    table.insert("THAT".to_string(),   4);
    table
}

fn second_scan(source: &str, table: &mut HashMap<String, u16>) -> Vec<u16> {
    let mut program: Vec<u16> = Vec::new();
    let mut next_var: u16 = 16;

    for line in source.lines() {
        let line = line.trim();


        if line.is_empty() || line.starts_with("//") { continue; }
        if line.starts_with('(') { continue; }

        if let Some(symbol) = line.strip_prefix('@') {

            if let Ok(n) = symbol.parse::<u16>() {
                program.push(n);
            } else {
                if !table.contains_key(symbol) {
                    table.insert(symbol.to_string(), next_var);
                    next_var += 1;
                }
                program.push(*table.get(symbol).unwrap());
            }
        } else {
            program.push(parse_c(line));
        }
    }
    program
}

fn parse_c(line: &str) -> u16 {
    let (dest, rest) = match line.find('=') {
        Some(i) => (&line[..i], &line[i+1..]),
        None    => ("", line),
    };
    let (comp, jump) = match rest.find(';') {
        Some(i) => (&rest[..i], &rest[i+1..]),
        None    => (rest, ""),
    };
    let c = encode_comp(comp.trim());
    let d = encode_dest(dest.trim());
    let j = encode_jump(jump.trim());

    0b1110_0000_0000_0000 | c | d | j
}

fn encode_comp(comp: &str) -> u16 {
    let bits: u16 = match comp {
        "0"   => 0b0_101010,
        "1"   => 0b0_111111,
        "-1"  => 0b0_111010,
        "D"   => 0b0_001100,
        "A"   => 0b0_110000,
        "!D"  => 0b0_001101,
        "!A"  => 0b0_110001,
        "-D"  => 0b0_001111,
        "-A"  => 0b0_110011,
        "D+1" => 0b0_011111,
        "A+1" => 0b0_110111,
        "D-1" => 0b0_001110,
        "A-1" => 0b0_110010,
        "D+A" => 0b0_000010,
        "D-A" => 0b0_010011,
        "A-D" => 0b0_000111,
        "D&A" => 0b0_000000,
        "D|A" => 0b0_010101,
        "M"   => 0b1_110000,
        "!M"  => 0b1_110001,
        "-M"  => 0b1_110011,
        "M+1" => 0b1_110111,
        "M-1" => 0b1_110010,
        "D+M" => 0b1_000010,
        "D-M" => 0b1_010011,
        "M-D" => 0b1_000111,
        "D&M" => 0b1_000000,
        "D|M" => 0b1_010101,
        _     => panic!("unknown comp: {}", comp),
    };

    let a = (bits >> 6) & 1;
    let c =  bits & 0b111111;
    (a << 12) | (c << 6)
}

fn encode_dest(dest: &str) -> u16 {
    match dest {
        ""    => 0b000 << 3,
        "M"   => 0b001 << 3,
        "D"   => 0b010 << 3,
        "MD"  => 0b011 << 3,
        "A"   => 0b100 << 3,
        "AM"  => 0b101 << 3,
        "AD"  => 0b110 << 3,
        "AMD" => 0b111 << 3,
        _     => panic!("unknown dest: {}", dest),
    }
}

fn encode_jump(jump: &str) -> u16 {
    match jump {
        ""    => 0b000,
        "JGT" => 0b001,
        "JEQ" => 0b010,
        "JGE" => 0b011,
        "JLT" => 0b100,
        "JNE" => 0b101,
        "JLE" => 0b110,
        "JMP" => 0b111,
        _     => panic!("unknown jump: {}", jump),
    }
}