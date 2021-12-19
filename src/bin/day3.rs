use std::fs;
use std::env;
use std::io::{self, BufRead};

fn count_bits(codes: &Vec<String>) -> Vec<i32> {
    let size = codes[0].chars().count();
    let mut net_counts: Vec<i32> = vec![0; size];
    for code in codes {
        for (idx, bit) in code.chars().enumerate() {
            if bit == '1' {
                net_counts[idx] += 1;
            }
            else {
                net_counts[idx] -= 1;
            }
        }
    }
    net_counts
}

fn parse_gamma_epsilon(counts: &Vec<i32>) -> (i32, i32) {
    let mut gamma_string = String::new();
    for num in counts {
        if num > &0 {
            gamma_string.push('1');
        }
        else if num < &0 {
            gamma_string.push('0');
        }
        else {
            panic!("No most common bit...wutdo?");
        }
    }
    let mut epsilon_string = String::new();
    for bit in gamma_string.chars() {
        match bit {
            '0' => epsilon_string.push('1'),
            '1' => epsilon_string.push('0'),
            _ => panic!("shouldn't happen")
        }
    }
    (i32::from_str_radix(gamma_string.as_str(), 2).unwrap(), i32::from_str_radix(epsilon_string.as_str(), 2).unwrap())
}

fn find_o2(codes: &Vec<i32>, idx: i32) -> i32 {
    if codes.len() == 0 {
        panic!("Something is wrong...");
    }

    if codes.len() == 1 {
        return codes[0];
    }

    let mask: i32 = 1 << (11 - idx);
    let mut one_cnt = 0;
    let mut zero_cnt = 0;
    for code in codes {
        if (mask & *code) != 0 {
            one_cnt += 1;
        }
        else {
            zero_cnt += 1;
        }
    }

    if one_cnt > zero_cnt {
        let mut candidates = Vec::new();
        for code in codes {
            if (*code & mask) > 0 {
                candidates.push(*code);
            }
        }
        return find_o2(&candidates, idx + 1);
    }
    else if zero_cnt > one_cnt {
        let mut candidates = Vec::new();
        for code in codes {
            if (*code & mask) == 0 {
                candidates.push(*code);
            }
        }
        return find_o2(&candidates, idx + 1);
    }
    else {
        let mut candidates = Vec::new();
        for code in codes {
            if (*code & mask) > 0 {
                candidates.push(*code);
            }
        }
        return find_o2(&candidates, idx + 1);
    }
}

fn find_co2(codes: &Vec<i32>, idx: i32) -> i32 {
    if codes.len() == 0 {
        panic!("Something is wrong...");
    }

    if codes.len() == 1 {
        return codes[0];
    }

    let mask: i32 = 1 << (11 - idx);
    let mut one_cnt = 0;
    let mut zero_cnt = 0;
    for code in codes {
        if (mask & *code) != 0 {
            one_cnt += 1;
        }
        else {
            zero_cnt += 1;
        }
    }

    if one_cnt < zero_cnt {
        let mut candidates = Vec::new();
        for code in codes {
            if (*code & mask) > 0 {
                candidates.push(*code);
            }
        }
        return find_co2(&candidates, idx + 1);
    }
    else if zero_cnt < one_cnt {
        let mut candidates = Vec::new();
        for code in codes {
            if (*code & mask) == 0 {
                candidates.push(*code);
            }
        }
        return find_co2(&candidates, idx + 1);
    }
    else {
        let mut candidates = Vec::new();
        for code in codes {
            if (*code & mask) == 0 {
                candidates.push(*code);
            }
        }
        return find_co2(&candidates, idx + 1);
    }

}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1])?;
    let reader = io::BufReader::new(file);
    let mut codes: Vec<String> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(text) => codes.push(text),
            _ => panic!("Read failed for line, {:?}", line),
        }
    }

    let count = count_bits(&codes);
    let (gamma, epsilon) = parse_gamma_epsilon(&count);
    println!("{}", gamma * epsilon);

    // Feel like binary could make this easier for part 2...
    let mut bitcodes: Vec<i32> = Vec::new();
    for code in &codes {
        bitcodes.push(i32::from_str_radix(code, 2).unwrap());
    }
    // all my code 12 bits anyway
    let o2 = find_o2(&bitcodes, 0);
    let co2 = find_co2(&bitcodes, 0);
    println!("{}", o2 * co2);

    Ok(())
}