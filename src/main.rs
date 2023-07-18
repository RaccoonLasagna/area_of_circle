use clap::{ App, Arg };
fn main() {
    let matches = App::new("area_of_circle")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1)
        )
        .get_matches();
    let text = matches.values_of_lossy("text").unwrap();
    let r: f64 = text[0].parse::<f32>().unwrap().into();
    let pi = 3.1416;
    let area = r * r * pi;
    print!("{}", area)
}
