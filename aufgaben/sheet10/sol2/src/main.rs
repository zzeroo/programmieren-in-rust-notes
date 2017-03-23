//! Helfer Tool wenn gerade keine Münze, kein Würfel oder ein Assistent griffbereit ist.
//!
//! - `coin` ("Wirft" Münze)
//! - `dice` ("Rollt" Würfel mit beliebig vielen Seiten)
//! - `choose` (Wählt aus einer Liste von Elementen mehrere Elemente aus)
//!
//! Wie zu sehen ist, lässt sich der `dice` Unterbefehl mit dem Parameter `--sides` anpassen.
//! Beim `choose` Unterbefehl kann durch den `--count` Befehl bestimmt werden,
//! wie viele Elemente aus der Liste ausgewählt werden sollen
//! (Achtung: Es sollen keine Elemente doppelt gewählt werden;
//! zwei mal "Peter" wäre im obigen Beispiel ungültig).
//!
//! Das ganze Programm lässt noch einen globalen Parameter zu: `--times`.
//! Mit diesem kann man bestimmen, wie oft der angegebene Unterbefehl ausgeführt werden soll.
//!
//! Beispiel:
//! ```bash
//! $ flip --times=3 coin
//! heads
//! heads
//! tails
//! $ flip --times=2 choose --count=2 Ursula Peter Sabine
//! ["Peter", "Sabine"]
//! ["Sabine", "Ursula"]
//! ```

extern crate clap;
extern crate rand;

use clap::{Arg, App, SubCommand};
use rand::Rng;


fn coin(times: u32) {
    for _ in 1..times + 1 {
        let num = rand::thread_rng().gen_range(0, 2);
        match num {
            0 => println!("heads"),
            1 => println!("tails"),
            _ => unreachable!()
        }
    }
}

fn dice(times: u32, sides: u32) {
    for _ in 1..times + 1 {
        let num = rand::thread_rng().gen_range(1, sides + 1);
        println!("{}", num);
    }
}

fn choose(times: u32, count: u32, choises: Vec<&str>) {
    for _ in 1..times + 1
    {
        let mut res: Vec<&str> = vec![];
        for c in 0..count {
            let num = rand::thread_rng().gen_range(0, count + 1);
            res.push(&choises.get(num as usize).unwrap());
        }
        println!("{:?}", res);
    }
}

fn main() {
    let matches = App::new("flip")
        .version("1.0")
        .author("zzeroo <co@zzeroo.com>")
        .about("Helfer Tool wenn gerade keine Münze, kein Würfel oder ein Assistent griffbereit ist.")
        .arg(Arg::with_name("times")
            .long("times")
            .help("Wie oft soll der angegebene Unterbefehl ausgeführt werden?")
            .global(true)
            .takes_value(true))
        .subcommand(SubCommand::with_name("coin")
            .about("\"Wirft\" Münze"))
        .subcommand(SubCommand::with_name("dice")
            .about("\"Rollt\" Würfel mit beliebig vielen Seiten")
            .arg(Arg::with_name("sides")
                .long("sides")
                .takes_value(true)
                .help("Wieviele Seiten hat der Würfel?")))
        .subcommand(SubCommand::with_name("choose")
            .about("Wählt aus einer Liste von Elementen mehrere Elemente aus.")
            .arg(Arg::with_name("count")
                .long("count")
                .takes_value(true)
                .help("Wie viele Elemente sollen aus der Liste ausgewählt werden?"))
            .args(&[Arg::with_name("choises")
                .help("Auswahl z.B. \"Ursula Peter Sabine\"")
                .required(true)
                .multiple(true)]
            ))
        .get_matches();


    // Globalen Paramenter
    let times = matches.value_of("times").unwrap_or("1").parse::<u32>().unwrap();

    if let Some(ref matches) = matches.subcommand_matches("coin") {
        coin(times);
    }

    if let Some(ref matches) = matches.subcommand_matches("dice") {
        let sides = matches.value_of("sides").unwrap_or("6").parse::<u32>().unwrap();
        dice(times, sides);
    }

    if let Some(ref matches) = matches.subcommand_matches("choose") {
        use std::ascii::AsciiExt;
        let count = matches.value_of("count").unwrap_or("1").parse::<u32>().unwrap();
        let mut choises = matches.values_of("choises").unwrap().collect::<Vec<_>>();
        choises.sort_by(|a, b| a.cmp(b));
        choises.dedup_by(|a, b| a == b);

        // Sinnlose count Langen beschneiden
        let count = if count > choises.len() as u32 { choises.len() as u32 } else { count };
        choose(times, count, choises);
    }

}
