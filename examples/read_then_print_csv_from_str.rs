use csv::Reader;
use csv::Writer;
use std::fs::File;

fn main() {
    // This examples shows how to read from a path then use tv_lib to print
    // The goal is to get the data into a &str for print_from_csv_str to work
    //
    // Steps:
    // Collect items in reader -> convert items to wtr -> convert wtr to String

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

    // print_from_csv_str
    tv_lib::print::print_from_csv_str(data.as_str());
}
