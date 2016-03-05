use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;


#[derive(Debug)]
enum Sign {
    Plus,
    Minus,
}


impl Sign {
    pub fn from(literal: char) -> Result<Sign, String> {
        match literal {
            '+' => Ok(Sign::Plus),
            '-' => Ok(Sign::Minus),
            _   => Err(String::from("Called Sign::new() with neither '+' or '-'!")),
        }
    }
}


#[derive(Debug)]
struct Amount {
    pence: i64,
}


impl Amount {
    pub fn from(pounds_literal: &str) -> Result<Amount, String> {
        // TODO: Account for decimal point.
        Ok(Amount { pence: pounds_literal.parse().unwrap() })
    }
}


#[derive(Debug)]
struct Entry {
    sign: Sign,
    amount: Amount,  // pence
    tags: HashSet<String>,
}



// fn pounds_to_pence(pounds: String) -> Result<i64, String> {
// }


fn parseln(line: String) -> Result<Entry, String> {
    
    let mut tokens = line.split_whitespace();

    //  Expect first token "+" or "-".
    let sign_slice = tokens.next().unwrap();
    assert!(sign_slice.len() == 1);
    let sign_literal = sign_slice.chars().nth(0).unwrap();
    let sign = Sign::from(sign_literal).unwrap();

    // Expect second token in the form "12.34".
    let amount_literal = tokens.next().unwrap();
    let amount = Amount::from(amount_literal).unwrap();

    // Remaining tokens are taken as tags.
    Ok(Entry { sign: sign, amount: amount, tags: tokens.map(String::from).collect() })
}


fn main() {
    let args: Vec<_> = env::args().collect();

    assert!(args.len() == 2);
    let ref filename = args[1];
    // Nightly: let [_, ref filename] = &args[..];

    println!("Opening <{}>.", filename);

    let fbf = BufReader::new(File::open(filename).unwrap());
    let entries = fbf.lines().map(|l| { parseln(l.unwrap()).unwrap() });
    let travel_entries = entries.filter(|e| { e.tags.contains(&String::from("travel")) });
    travel_entries.map(|e| { println!("{:?}", e) }).count();  // count() call consumes the iter.
}
