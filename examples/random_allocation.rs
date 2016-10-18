// Calculations to do with security of routing system

#![feature(inclusive_range_syntax)]
#![allow(non_snake_case)]   // this is maths

extern crate rustc_serialize;
extern crate docopt;
extern crate routing_sims;

use docopt::Docopt;
use routing_sims::*;


const USAGE: &'static str = "
Proofs / probablity computation tool.

Sim = random allocation to groups; calculate the probability of compromise given
that malicious nodes are randomly distributed with no targetting/rejoining.

Usage:
    proofs [-h | --help]
    proofs [-n NUM] [-r VAL] [-k RANGE] [-q RANGE] [-a]

Options:
    -h --help   Show this message
    -n NUM      Number of nodes, total.
    -r VAL      Either number of compromised nodes (e.g. 50) or percentage (default is 10%).
    -k RANGE    Group size, e.g. 10-20.
    -q RANGE    Quorum size, e.g. 5-20.
    -a          Show probabilities of any group being compromised instead of a specific group
";

#[derive(RustcDecodable)]
struct Args {
    flag_n: Option<NN>,
    flag_r: Option<String>,
    flag_k: Option<String>,
    flag_q: Option<String>,
    flag_a: bool,
}
    
fn main(){
    let args: Args = Docopt::new(USAGE)
            .and_then(|dopt| dopt.decode())
            .unwrap_or_else(|e| e.exit());
    
    let n = args.flag_n.unwrap_or(1000);
    let r = if let Some(mut s) = args.flag_r {
        if s.ends_with('%') {
            let _ = s.pop();
            (n as RR * s.parse::<RR>().expect("In '-r x%', x should be a real number")) as NN
        } else {
            s.parse::<NN>().expect("In '-r N', N should be a whole number or percentage")
        }
    } else {
        (n as RR * 0.1) as NN
    };
    fn parse_range(s: &str) -> (NN, NN) {
        let ERR: &'static str = "In a range, syntax should be 'x-y' where x and y are whole numbers";
        let i = s.find('-').expect(ERR);
        let lb = s[0..i].parse::<NN>().expect(ERR);
        let ub = s[i+1..s.len()].parse::<NN>().expect(ERR);
        (lb, ub)
    }
    let k = args.flag_k.map_or((8, 12), |s| parse_range(&s));
    let q = args.flag_q.map_or((5, 12), |s| parse_range(&s));
    let any = args.flag_a;
    
    if any {
        println!("Expected number of compromised groups, assuming fixed group size, where");
    } else {
        println!("Probability of one specific group being compromised, where");
    }
    println!("Total nodes n = {}", n);
    println!("Compromised nodes r = {}", r);
    println!("Group size on horizontal axis (cols)");
    println!("Qurom size on vertical axis (rows)");
    
    let W0: usize = 3;      // width first column
    let W1: usize = 24;     // width other columns
    
    // header:
    print!("{1:0$}", W0, "");
    for ki in k.0 ... k.1 {
        print!("{1:0$}", W1, ki);
    }
    println!("");
    //rest:
    for qi in q.0 ... q.1 {
        print!("{1:0$}", W0, qi);
        for ki in k.0 ... k.1 {
            let mult = if any { (n as RR) / (ki as RR) } else { 1.0 };
            
            if qi > ki {
                print!("{1:>0$}", W1, "-");
                continue;
            }
            let p = probQRChosen(n, r, ki, qi) * mult;
            print!("{1:0$.e}", W1, p);
        }
        println!("");
    }
}