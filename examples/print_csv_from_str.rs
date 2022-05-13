fn main() {
    // This examples shows how to take any csv-like string and make print it using
    // tv_lib styling.
    let data = "\
City,State,Population,Latitude,Longitude
Davidsons Landing,AK,,65.2419444,-165.2716667
Kenai,AK,7610,60.5544444,-151.2583333
Oakman,AL,,33.7133333,-87.3886111";

    // print_from_csv_str
    tv_lib::print::print_from_csv_str(data);
}
