
pub fn digi_graph_main() {
    println!("digi_graph_main");
}

fn char_to_number(c: char) -> usize {
    c as usize - '0' as usize
}

fn turth_table_print(
    inputs: &Vec<Vec<char>>, 
    outruth: &Vec<char>,
    input_num: usize,
    truth_num: usize

) {
    println!("turth_table_print:");
    for truth_index in 0..truth_num {
        print!("F: ");
        for x_index in 0..input_num {
            print!("{}  ", &inputs[truth_index][x_index]);
        }
        print!("= {}", outruth[truth_index]);
        println!("");
    }
    println!("");
}

fn sumOf_min_nomia(
    inputs: Vec<Vec<char>>, 
    outruth: Vec<char>,
    input_num: usize,
    truth_num: usize

) {
    turth_table_print(&inputs, &outruth, input_num, truth_num);

    fn min_nomia(truth_index: usize, x_vec: &Vec<char>) {
        let mut logic_str: String = String::from("");
        for xi in 0..x_vec.len() {
            if x_vec[xi] == '0' {
                logic_str.push_str(&format!("(not x{xi})"));
            } else {
                logic_str.push_str(&format!("x{xi}"));
            }
            if xi != x_vec.len() - 1 {
                logic_str.push_str(" and ");
            }
        }

        println!("m{truth_index}: {:?} -> {}", x_vec, logic_str);
    }

    for truth_index in 0..truth_num {
        if outruth[truth_index] == '1' {
            min_nomia(truth_index, &inputs[truth_index]);
        }
    }
}


pub fn wave_test1() {
    // F = 
    let wave_str: Vec<Vec<char>> = vec![
        vec!['3', '1', '8'],
        vec!['0', '0', '0', '1'],
        vec!['0', '0', '1', '0'],
        vec!['0', '1', '0', '1'],
        vec!['0', '1', '1', '0'],
        vec!['1', '0', '0', '1'],
        vec!['1', '0', '1', '1'],
        vec!['1', '1', '0', '1'],
        vec!['1', '1', '1', '0'],
    ];

    let input_num: usize = char_to_number(wave_str[0][0]);
    let output_num: usize = char_to_number(wave_str[0][1]);
    let truth_cond: usize = char_to_number(wave_str[0][2]);

    for logic_index in 0..output_num {
        println!("logic {logic_index}: ");

        let mut all_inputs: Vec<Vec<char>> = Vec::new();
        let mut outruth: Vec<char> = Vec::new();

        for truth_index in 0..truth_cond {
            let wave: &Vec<char> = &wave_str[1 + truth_index];
            let mut inputs: Vec<char> = Vec::new();
            for x_index in 0..input_num {
                inputs.push(wave[x_index]);
            }
            all_inputs.push(inputs);
            outruth.push(wave[input_num + logic_index]);
        }

        sumOf_min_nomia(all_inputs, outruth, input_num, truth_cond);
        
    }
}

pub fn wave_test2() {
    // F = X2 + ~X1
    let wave_str: Vec<Vec<char>> = vec![
        vec!['2', '1', '4'],
        vec!['0', '0', '1'],
        vec!['0', '1', '1'],
        vec!['1', '0', '0'],
        vec!['1', '1', '1'],
    ];

}

pub fn say_xvectors(input_num: usize) -> Vec<String> {
    let mut xvectors: Vec<String> = Vec::new();

    for i in 0..(1 << input_num) {
        let x_binary_str: String = format!("{:0width$b}", i, width = input_num);
        // println!("p{i}: {x_binary_str}");
        xvectors.push(x_binary_str);
    }
    return xvectors
}