use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;


#[derive(Debug)]
struct Delta {
    pence: i64,
}


#[derive(Debug)]
struct Entry {
    delta: Delta,
    tags: HashSet<String>,
}


fn parseln(line: String) -> Result<Entry, String> {

    let mut tokens = line.split_whitespace();

    //  Expect first token "+" or "-".
    let sign_slice = tokens.next().unwrap();
    assert!(sign_slice.len() == 1);
    let sign_literal = sign_slice.chars().nth(0).unwrap();

    let amount = tokens.next().unwrap().parse().unwrap();
    let delta = try!(match sign_literal {
        '+' => Ok(Delta { pence: amount }),
        '-' => Ok(Delta { pence: -amount }),
        _ => Err(String::from("First token was neither '+' or '-'!")),
    });

    // Remaining tokens are taken as tags.
    Ok(Entry { delta: delta, tags: tokens.map(String::from).collect() })
}


fn main() {
    let args: Vec<_> = env::args().collect();

    assert!(args.len() == 3);
    let ref filename = args[1];
    let ref filter_tag = args[2];
    // Nightly: let [_, ref filename] = &args[..];

    println!("Opening <{}>.", filename);

    let fbf = BufReader::new(File::open(filename).unwrap());
    let entries = fbf.lines().map(|l| { parseln(l.unwrap()).unwrap() });

    let chosen_entries = entries.filter(|e| { e.tags.contains(filter_tag) });
    chosen_entries.map(|e| { println!("{:?}", e) }).count();  // count() call consumes the iter.
}
