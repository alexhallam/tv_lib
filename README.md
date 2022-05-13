
<h1 align="center">Tidy Viewer Library (tv_lib)</h1>
<p align="center">This is a Rust library that allows users to grab csv text as formatted by the <a href="https://github.com/alexhallam/tv">tv cli</a> command line utility and use it as they see fit</p>

# tv lib example

```rust
fn main() {
    // define path
    let in_file_path: &str = "examples/data/uspop.csv";
    // one liner
    let vec_vec_str = tv_lib::print::read_csv(in_file_path);
    //print
    println!("{:?}", vec_vec_str[2][0..6].to_vec());
    println!("{:?}", vec_vec_str[3][0..6].to_vec());
}
```

```shell
["Population", "NA", "7610", "NA", "NA", "NA"]
["Latitude", "65.2", "60.5", "33.7", "31.7", "32.3"]
```

In this small example see the features ported from [tv](https://github.com/alexhallam/tv).

1. Missing values are filled with NA
2. Latitude is normally a decimal that goes out 7 places! See that the sig figs are set to only show one significant digit.

Remember, the goal is tv is about data visualization. In any practical calculation it is important to have a long decimal for lat/lon?

# Installation

Include in Cargo.toml

```toml
tv-lib = "0.1.0"
```

# Examples 

Examples may be run with `cargo run --example <file in examples dir>`

The following are available

```shell
cargo run --example short_read_csv
cargo run --example print_csv_from_str
cargo run --example format_csv_from_str
cargo run --example read_then_format_csv_from_str
cargo run --example read_then_print_csv_from_str
```

# Examples Explicit

Some times it is too hard to want to go into a directory to copy and paste a chunk of code 😉.

## Format from strings

```rust
fn main() {
    // define path
    let in_file_path: &str = "examples/data/uspop.csv";
    // one liner
    let vec_vec_str = tv_lib::print::read_csv(in_file_path);
    //print
    println!("{:?}", vec_vec_str);
}
```

```rust
// format_from_csv_str
fn main() {
    let data = "\
City,State,Population,Latitude,Longitude
Davidsons Landing,AK,,65.2419444,-165.2716667
Kenai,AK,7610,60.5544444,-151.2583333
Oakman,AL,,33.7133333,-87.3886111";

    let vec_vec_str = tv_lib::print::format_from_csv_str(data);
    println!("{:?}",vec_vec_str)
}
```

```rust
// print_csv_from_str
fn main() {
    let data = "\
City,State,Population,Latitude,Longitude
Davidsons Landing,AK,,65.2419444,-165.2716667
Kenai,AK,7610,60.5544444,-151.2583333
Oakman,AL,,33.7133333,-87.3886111";
    tv_lib::print::print_from_csv_str(data);
}
```

## Read CSV Then Format
```rust
use csv::Reader;
use csv::Writer;
use std::fs::File;

fn main() {
    let in_file_path: &str = "examples/data/uspop.csv";

    // csv::ReaderBuilder
    let mut r: Reader<File> = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_path(in_file_path)
        .unwrap();

    // Collect items in reader
    let rdr = r
        .records()
        .into_iter()
        .map(|x| x.expect("a csv record"))
        .collect::<Vec<_>>();

    // convert items to wtr
    let rows: usize = rdr.len();
    let mut wtr = Writer::from_writer(vec![]);
    for row in 0..rows {
        wtr.write_record(&rdr[row]).unwrap();
    }

    // convert wtr to string
    let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();

    // use tv_lib: print_from_csv_str
    let vec_vec_str = tv_lib::print::format_from_csv_str(data.as_str());

    println!("{:?}", vec_vec_str[1][0..6].to_vec());
    println!("{:?}", vec_vec_str[1][0..6].to_vec());
    println!("{:?}", vec_vec_str[2][0..6].to_vec());
    println!("{:?}", vec_vec_str[3][0..6].to_vec());
    println!("{:?}", vec_vec_str[4][0..6].to_vec());
}
```

```rust
use csv::Reader;
use csv::Writer;
use std::fs::File;

fn main() {

    // path
    let in_file_path: &str = "examples/data/uspop.csv";

    // csv::ReaderBuilder
    let mut r: Reader<File> = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_path(in_file_path)
        .unwrap();

    // Collect items in reader
    let rdr = r
        .records()
        .into_iter()
        .map(|x| x.expect("a csv record"))
        .collect::<Vec<_>>();

    // convert items to wtr
    let rows: usize = rdr.len();
    let mut wtr = Writer::from_writer(vec![]);
    for row in 0..rows {
        wtr.write_record(&rdr[row]).unwrap();
    }

    // convert wtr to string
    let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();

    // use tv_lib: print_from_csv_str
    tv_lib::print::print_from_csv_str(data.as_str());
}

```