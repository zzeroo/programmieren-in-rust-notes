
enum PartyKind {
    Formal,
    Informal,
}

struct Party {
    kind: PartyKind,
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn wassup(name: &str) {
    println!("Wassuuuuup, {}?", name);
}


fn main() {
    let party = Party { kind: PartyKind::Informal, };

    let greeter = match party.kind {
        PartyKind::Formal => hello,
        PartyKind::Informal => wassup,
    };

    greeter("Peter");
    greeter("Heike");
    greeter("Jörg");
}
