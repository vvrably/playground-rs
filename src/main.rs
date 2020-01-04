use std::env;
use std::fs;
use strsim::jaro_winkler;
use rayon::prelude::*;


fn main() {
    // Read input files to strings
    let args: Vec<String> = env::args().map(String::from).collect();
    let file_a = args.get(1).expect("No input file a");
    let file_b = args.get(2).expect("No input file b");

    let contents_a = fs::read_to_string(&file_a)
        .expect("Something went wrong with reading the file");
    let contents_b = fs::read_to_string(&file_b)
        .expect("Something went wrong with reading the file");

    // One word per line
    let names_a: Vec<&str> = contents_a.lines().collect();
    let names_b: Vec<&str> = contents_b.lines().collect();

    // Main outer loop
    let result = names_a
        .par_iter()
        .map(|&a| {
            // Main inner loop
            // to get the most similar string together with its jaro-winkler similarity
            let (b_str, b_val) = names_b
                .par_iter()
                .fold(|| None, |max, &b| match max {
                    None => Some((b, jaro_winkler(b, a))),
                    Some((max_str, max_val)) => {
                        let b_val = jaro_winkler(b, a);
                        if b_val > max_val {
                            Some((b, b_val))
                        } else {
                            Some((max_str, max_val))
                        }
                    }
                })
                .map(|x| x.unwrap())
                // Reduce
                .max_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
                .unwrap();

            (a, b_str, b_val)
        })
        .collect::<Vec<_>>();

    // Save to file
    let path = "data/output.txt";
    let output = result
        .iter()
        .fold(String::new(), |acc, x| {
            let (a, b, val) = x;
            let line = format!("{},{},{:.10}\n", a, b, val);
            acc + &line
        });

    fs::write(&path, output).unwrap();

}
