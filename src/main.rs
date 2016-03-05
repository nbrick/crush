use std::env;
use std::fs::File;
use std::io::{stderr, Write, BufRead, BufReader};
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
    let first_token = try!(match tokens.next() {
        Some(token) => Ok(token),
        None => Err("Found no tokens"),
    });

    let amount_token = try!(match tokens.next() {
        Some(t) => Ok(t),
        None => Err(String::from("Found no second token")),
    });

    let amount = try!(match amount_token.parse() {
        Ok(t) => Ok(t),
        Err(_) => Err("Failed to parse amount token"),
    });

    let delta = try!(match first_token {
        "+" => Ok(Delta { pence: amount }),
        "-" => Ok(Delta { pence: -amount }),
        _ => Err(String::from("First token was neither \"+\" or \"-\"")),
    });

    // Remaining tokens are taken as tags.
    Ok(Entry { delta: delta, tags: tokens.map(String::from).collect() })
}


fn sum_delta(entries: &Vec<Entry>) -> Delta {
    Delta { pence: entries.iter().fold(0, |s, ref d| s + d.delta.pence) }
}


fn main() {
    let args: Vec<_> = env::args().collect();

    assert!(args.len() == 3);
    let ref filename = args[1];
    let ref filter_tag = args[2];
    // Nightly: let [_, ref filename] = &args[..];

    println!("Opening <{}>.", filename);

    let input = BufReader::new(File::open(filename).unwrap());
    let entries = input.lines().enumerate()
        .map(|(n, l)| { (n, l.expect("Failed to read input line")) })
        .map(|(n, l)| { (n, parseln(l)) })
        .filter(|&(ref n, ref res_e)| {
            match res_e {
                &Ok(_) => true,
                &Err(ref err) => {
                    writeln!(&mut stderr(), "crush@{}:{}: {}", filename, n+1, err).unwrap();
                    false
                },
            }
        }).map(|(_, ok_e)| { ok_e.unwrap() });

    let chosen_entries: Vec<_> = entries.filter(|e| { e.tags.contains(filter_tag) }).collect();
    for e in &chosen_entries { println!("{:?}", e); }
    println!("Total: {:?} from {:?} entries", sum_delta(&chosen_entries), chosen_entries.len());
}
