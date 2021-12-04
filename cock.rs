use viuer::{print_from_file, Config};

fn main() {
    let conf = Config {
        x: 20,
        y: 4,
        width: Some(80),
        height: Some(25),
        ..Default::default()
    };

    print_from_file("promethen.png", &conf).expect("epic fail");
}