use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_result(list1: &mut Vec<i64>, list2: &mut Vec<i64>) -> io::Result<i64> {
    //read line
    let file: &str = "../input.txt";
    let file = File::open(file)?;
    let reader: BufReader<File> = BufReader::new(file);

    //sort both lists
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();

        list1.push(parts.next().unwrap().parse().unwrap());
        list2.push(parts.next().unwrap().parse().unwrap());
    }
    list1.sort();
    list2.sort();

    let mut total_distance = 0i64;
    total_distance += get_distance(&list1, &list2);
    Ok(total_distance)
}

//get absolute difference between corresponding indexes
fn get_distance(list1: &Vec<i64>, list2: &Vec<i64>) -> i64 {
    list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (*a as i64 - *b as i64).abs())
        .sum()
}
fn main() {
    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();

    //print distance of each value between the lists
    match get_result(&mut list1, &mut list2) {
        Ok(total_distance) => println!("Total distance between lists: {}", total_distance),
        Err(e) => eprintln!("Error reading file: {e}"),
    }
    //print sum of different values multiplied by # of occurances
    let similarity_score = get_similarity(&list1, &list2);
    println!("Similarity score: {}", similarity_score);
}

////part 2
// search for similarity in list 2 for each num of list 1
fn get_similarity(list1: &Vec<i64>, list2: &Vec<i64>) -> i64 {
    let mut similarity = 0;

    //count and compare each list1 index with each list2 index
    for &j in list1 {
        let mut counter = 0;
        for &k in list2 {
            if j == k {
                counter += 1;
            }
        }
        similarity += counter * j;
    }
    //return number of differences multiplied by the value
    similarity
}
