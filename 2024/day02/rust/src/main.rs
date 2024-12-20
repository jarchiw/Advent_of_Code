use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_rules(report: &str) -> i8 {
    //make report list of int levels
    let report_list: Vec<i32> = report
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut unsafe_count: i8 = 0;
    //iterate through levels with rules:
    //rules: 1) Either increasing or decreasing, 2) between 1-3 difference.
    if report_list[0] <= report_list[1] {
        for (i, j) in report_list.iter().zip(report_list.iter().skip(1)) {
            if j <= i || (j - i).abs() < 1 || (j - i).abs() > 3 {
                unsafe_count += 1;
            }
        }
    } else if report_list[0] > report_list[1] {
        for (i, j) in report_list.iter().zip(report_list.iter().skip(1)) {
            if j >= i || (i - j).abs() < 1 || (i - j).abs() > 3 {
                unsafe_count += 1;
            }
        }
    }
    //return # of unsafe levels within single report
    unsafe_count
}

fn main() -> Result<(), io::Error> {
    //read line
    let file: &str = "../input.txt";
    let file = File::open(file)?;
    let reader: BufReader<File> = BufReader::new(file);

    //count safe lines & single exception unsafe lines
    let mut safe_reports: i16 = 0;
    let mut acceptable_reports: i16 = 0;

    for line in reader.lines() {
        let report = line?;
        let un_safe = get_rules(&report);
        if un_safe == 0 {
            safe_reports += 1;
        } else if un_safe == 1 {
            acceptable_reports += 1;
        }
    }
    acceptable_reports += safe_reports;
    //part1 output safe lines
    //part2 output safe lines with exception of 1 unsafe line
    println!("Total safe reports: {}", safe_reports);
    println!("Total safe reports: {}", acceptable_reports);
    Ok(())
}
