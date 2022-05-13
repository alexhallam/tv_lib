pub mod print {
    use calm_io::stdout;
    use calm_io::stdoutln;
    use crossterm::terminal::size;
    use csv::Reader;
    use csv::StringRecord;
    use owo_colors::OwoColorize;

    pub fn print_from_csv_str(data: &str) {
        // rename: print_csv_from_str
        // new fun: formt_csv_from_str
        let mut rdr: Reader<&[u8]> = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b',')
            .from_reader(data.as_bytes());

        // Make Reader<&[u8]> an iterable. Check each element is iterable. Collect into a <Vec<<_>>
        let rdr: Vec<StringRecord> = rdr
            .records()
            .into_iter()
            .map(|x| x.expect("a csv record"))
            .collect::<Vec<_>>();

        let row_display_option = 25;
        let lower_column_width = 2;
        let upper_column_width = 20;
        let sigfig = 3;

        let cols: usize = rdr[0].len();
        let rows: usize = rdr.len().min(row_display_option + 1);
        let rows_in_file: usize = rdr.len();
        let rows_remaining: usize = rows_in_file - rows;
        let ellipsis = '\u{2026}'.to_string();
        let row_remaining_text: String = format!("{} with {} more rows", ellipsis, rows_remaining);

        // csv gets records in rows. This makes them cols. Put cols in `v`
        let mut v: Vec<Vec<&str>> = Vec::new();
        for col in 0..cols {
            let column = rdr
                .iter()
                .take(rows)
                .map(|row: &StringRecord| row.get(col).unwrap())
                .collect();
            v.push(column)
        }

        // vector of formatted values `vf`
        let vf: Vec<Vec<String>> = v
            .iter()
            .map(|col| {
                format_csv::format_strings(col, lower_column_width, upper_column_width, sigfig)
            })
            .collect();

        let title = "";
        let title_option = title;
        let footer = "";
        let footer_option = footer;
        let meta_color: [u8; 3] = [143, 188, 187];
        let header_color: [u8; 3] = [94, 129, 172];
        let std_color: [u8; 3] = [216, 222, 233];
        let na_color: [u8; 3] = [191, 97, 106];
        let neg_num_color: [u8; 3] = [208, 135, 112];
        let extend_option = false;
        let term_tuple = size().unwrap();
        let is_tty = atty::is(atty::Stream::Stdout);
        let is_force_color = false;

        print_from_vec_vec(
            vf.clone(),
            rdr.clone(),
            rows,
            cols,
            extend_option,
            term_tuple,
            is_tty,
            is_force_color,
            meta_color,
            header_color,
            na_color,
            neg_num_color,
            std_color,
            rows_in_file,
            title_option,
            footer_option,
            rows_remaining,
            row_remaining_text,
        );
    }

    pub fn format_from_csv_str(data: &str) -> Vec<Vec<String>> {
        // rename: print_csv_from_str
        // new fun: formt_csv_from_str
        let mut rdr: Reader<&[u8]> = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b',')
            .from_reader(data.as_bytes());

        // Make Reader<&[u8]> an iterable. Check each element is iterable. Collect into a <Vec<<_>>
        let rdr: Vec<StringRecord> = rdr
            .records()
            .into_iter()
            .map(|x| x.expect("a csv record"))
            .collect::<Vec<_>>();

        let row_display_option = 25;
        let lower_column_width = 2;
        let upper_column_width = 20;
        let sigfig = 3;

        let cols: usize = rdr[0].len();
        let rows: usize = rdr.len().min(row_display_option + 1);

        // csv gets records in rows. This makes them cols. Put cols in `v`
        let mut v: Vec<Vec<&str>> = Vec::new();
        for col in 0..cols {
            let column = rdr
                .iter()
                .take(rows)
                .map(|row: &StringRecord| row.get(col).unwrap())
                .collect();
            v.push(column)
        }

        // vector of formatted values `vf`
        //let vf: Vec<Vec<String>> = v
        //    .iter()
        //    .map(|col| {
        //        format_csv::format_strings(col, lower_column_width, upper_column_width, sigfig)
        //    })
        //    .collect();

        let vf: Vec<Vec<String>> = v
            .iter()
            .map(|col| {
                format_csv::format_strings(col, lower_column_width, upper_column_width, sigfig)
            })
            .collect();

        let vfs: Vec<Vec<String>> = vf
            .iter()
            .map(|inner: &Vec<String>| -> Vec<String> {
                inner
                    .iter()
                    .map(|s: &String| -> &str {
                        // trim references the original String
                        s.trim()
                    })
                    .filter(|s| !s.is_empty())
                    .map(|trimmed: &str| {
                        // in order to make a new String, which is what we said we wanted for vfs, we need to tell it to make a new String with to_string or to_owned (they do the same thing)
                        trimmed.to_string()
                    })
                    .collect()
            })
            .collect();

        vfs
    }

    //println!("{:#?}", rdr);
    //println!("{:#?}", v);
    //println!("{:#?}", vf);

    pub fn print_from_vec_vec(
        vf: Vec<Vec<String>>,
        rdr: Vec<StringRecord>,
        rows: usize,
        cols: usize,
        extend_option: bool,
        term_tuple: (u16, u16),
        is_tty: bool,
        is_force_color: bool,
        meta_color: [u8; 3],
        header_color: [u8; 3],
        na_color: [u8; 3],
        neg_num_color: [u8; 3],
        std_color: [u8; 3],
        rows_in_file: usize,
        title_option: &str,
        footer_option: &str,
        rows_remaining: usize,
        row_remaining_text: String,
    ) {
        let mut vp = Vec::new();
        for r in 0..rows {
            let row = vf.iter().map(|col| col[r].to_string()).collect();
            vp.push(row);
        }

        let num_cols_to_print = if extend_option {
            cols
        } else {
            get_num_cols_to_print(cols, vp.clone(), term_tuple)
        };

        // color
        let meta_text = "tv dim:";
        let div = "x";
        let _ = match stdout!("{: <6}", "") {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::BrokenPipe => Ok(()),
                _ => Err(e),
            },
        };
        if is_tty || is_force_color {
            let _ = match stdoutln!(
                "{} {} {} {}",
                meta_text.truecolor(meta_color[0], meta_color[1], meta_color[2]),
                (rows_in_file - 1).truecolor(meta_color[0], meta_color[1], meta_color[2]),
                div.truecolor(meta_color[0], meta_color[1], meta_color[2]),
                (cols).truecolor(meta_color[0], meta_color[1], meta_color[2]),
            ) {
                Ok(_) => Ok(()),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::BrokenPipe => Ok(()),
                    _ => Err(e),
                },
            };
        } else {
            let _ = match stdoutln!("{} {} {} {}", meta_text, rows_in_file - 1, div, cols) {
                Ok(_) => Ok(()),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::BrokenPipe => Ok(()),
                    _ => Err(e),
                },
            };
        }
        // title
        if !format_csv::is_na(&title_option.clone()) {
            let _ = match stdout!("{: <6}", "") {
                Ok(_) => Ok(()),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::BrokenPipe => Ok(()),
                    _ => Err(e),
                },
            };
            if is_tty || is_force_color {
                let _ = match stdoutln!(
                    "{}",
                    title_option
                        .truecolor(meta_color[0], meta_color[1], meta_color[2])
                        .underline()
                        .bold()
                ) {
                    Ok(_) => Ok(()),
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::BrokenPipe => Ok(()),
                        _ => Err(e),
                    },
                };
            } else {
                let _ = match stdoutln!("{}", title_option) {
                    Ok(_) => Ok(()),
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::BrokenPipe => Ok(()),
                        _ => Err(e),
                    },
                };
            }
        }

        // header
        let _ = match stdout!("{: <6}", "") {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::BrokenPipe => Ok(()),
                _ => Err(e),
            },
        };
        //for col in 0..cols {
        for col in 0..num_cols_to_print {
            let text = vp[0].get(col).unwrap().to_string();
            if is_tty || is_force_color {
                let _ = match stdout!(
                    "{}",
                    text.truecolor(header_color[0], header_color[1], header_color[2])
                        .bold()
                ) {
                    Ok(_) => Ok(()),
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::BrokenPipe => Ok(()),
                        _ => Err(e),
                    },
                };
            } else {
                let _ = match stdout!("{}", text) {
                    Ok(_) => Ok(()),
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::BrokenPipe => Ok(()),
                        _ => Err(e),
                    },
                };
            }
        }
        //println!();
        // formats
        //print!("{: <6}", "");
        //for col in 0..cols{
        //    let add_space = vec_formats[col].len() - col_largest_width[col];
        //    let mut owned_string: String = vec_formats[col].to_string();
        //    let borrowed_string: &str = &" ".repeat(add_space);
        //    owned_string.push_str(borrowed_string);
        //    print!("{}",owned_string.truecolor(143, 188, 187).bold());
        //}
        let _ = match stdoutln!() {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::BrokenPipe => Ok(()),
                _ => Err(e),
            },
        };
        vp.iter()
            .enumerate()
            .take(rows)
            .skip(1)
            .for_each(|(i, row)| {
                if is_tty || is_force_color {
                    let _ = match stdout!(
                        "{: <6}",
                        i.truecolor(meta_color[0], meta_color[1], meta_color[2])
                    ) {
                        Ok(_) => Ok(()),
                        Err(e) => match e.kind() {
                            std::io::ErrorKind::BrokenPipe => Ok(()),
                            _ => Err(e),
                        },
                    };
                } else {
                    let _ = match stdout!("{: <6}", i) {
                        Ok(_) => Ok(()),
                        Err(e) => match e.kind() {
                            std::io::ErrorKind::BrokenPipe => Ok(()),
                            _ => Err(e),
                        },
                    };
                }
                row.iter().take(num_cols_to_print).for_each(|col| {
                    if is_tty || is_force_color {
                        let _ = match stdout!(
                            "{}",
                            if format_csv::is_na_string_padded(col) {
                                col.truecolor(na_color[0], na_color[1], na_color[2])
                            } else {
                                if format_csv::is_number(col) && format_csv::is_negative_number(col)
                                {
                                    col.truecolor(
                                        neg_num_color[0],
                                        neg_num_color[1],
                                        neg_num_color[2],
                                    )
                                } else {
                                    col.truecolor(std_color[0], std_color[1], std_color[2])
                                }
                            }
                        ) {
                            Ok(_) => Ok(()),
                            Err(e) => match e.kind() {
                                std::io::ErrorKind::BrokenPipe => Ok(()),
                                _ => Err(e),
                            },
                        };
                    } else {
                        let _ = match stdout!("{}", col) {
                            Ok(_) => Ok(()),
                            Err(e) => match e.kind() {
                                std::io::ErrorKind::BrokenPipe => Ok(()),
                                _ => Err(e),
                            },
                        };
                    }
                });
                let _ = match stdoutln!() {
                    Ok(_) => Ok(()),
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::BrokenPipe => Ok(()),
                        _ => Err(e),
                    },
                };
            });

        // additional row info
        if rows_remaining > 0 {
            let _ = match stdout!("{: <6}", "") {
                Ok(_) => Ok(()),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::BrokenPipe => Ok(()),
                    _ => Err(e),
                },
            };
            if is_tty || is_force_color {
                let _ = match stdout!(
                    "{}",
                    row_remaining_text.truecolor(meta_color[0], meta_color[1], meta_color[2])
                ) {
                    Ok(_) => Ok(()),
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::BrokenPipe => Ok(()),
                        _ => Err(e),
                    },
                };
            } else {
                let _ = match stdout!("{}", row_remaining_text) {
                    Ok(_) => Ok(()),
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::BrokenPipe => Ok(()),
                        _ => Err(e),
                    },
                };
            }
            let extra_cols_to_mention = num_cols_to_print;
            let remainder_cols = cols - extra_cols_to_mention;
            if extra_cols_to_mention < cols {
                let meta_text_and = "and";
                let meta_text_var = "more variables";
                let meta_text_comma = ",";
                let meta_text_colon = ":";
                if is_tty || is_force_color {
                    let _ = match stdout!(
                        " {} {} {}{}",
                        meta_text_and.truecolor(meta_color[0], meta_color[1], meta_color[2]),
                        remainder_cols.truecolor(meta_color[0], meta_color[1], meta_color[2]),
                        meta_text_var.truecolor(meta_color[0], meta_color[1], meta_color[2]),
                        meta_text_colon.truecolor(meta_color[0], meta_color[1], meta_color[2])
                    ) {
                        Ok(_) => Ok(()),
                        Err(e) => match e.kind() {
                            std::io::ErrorKind::BrokenPipe => Ok(()),
                            _ => Err(e),
                        },
                    };
                } else {
                    let _ = match stdout!(
                        " {} {} {}{}",
                        meta_text_and,
                        remainder_cols,
                        meta_text_var,
                        meta_text_colon
                    ) {
                        Ok(_) => Ok(()),
                        Err(e) => match e.kind() {
                            std::io::ErrorKind::BrokenPipe => Ok(()),
                            _ => Err(e),
                        },
                    };
                }
                for col in extra_cols_to_mention..cols {
                    let text = rdr[0].get(col).unwrap();
                    if is_tty || is_force_color {
                        let _ = match stdout!(
                            " {}",
                            text.truecolor(meta_color[0], meta_color[1], meta_color[2])
                        ) {
                            Ok(_) => Ok(()),
                            Err(e) => match e.kind() {
                                std::io::ErrorKind::BrokenPipe => Ok(()),
                                _ => Err(e),
                            },
                        };
                    } else {
                        let _ = match stdout!(" {}", text) {
                            Ok(_) => Ok(()),
                            Err(e) => match e.kind() {
                                std::io::ErrorKind::BrokenPipe => Ok(()),
                                _ => Err(e),
                            },
                        };
                    }

                    // The last column mentioned in foot should not be followed by a comma
                    if col + 1 < cols {
                        if is_tty || is_force_color {
                            let _ = match stdout!(
                                "{}",
                                meta_text_comma.truecolor(
                                    meta_color[0],
                                    meta_color[1],
                                    meta_color[2]
                                )
                            ) {
                                Ok(_) => Ok(()),
                                Err(e) => match e.kind() {
                                    std::io::ErrorKind::BrokenPipe => Ok(()),
                                    _ => Err(e),
                                },
                            };
                        } else {
                            let _ = match stdout!("{}", meta_text_comma) {
                                Ok(_) => Ok(()),
                                Err(e) => match e.kind() {
                                    std::io::ErrorKind::BrokenPipe => Ok(()),
                                    _ => Err(e),
                                },
                            };
                        }
                    }
                } // end extra cols mentioned in footer
            }
        }

        // footer
        if !format_csv::is_na(&footer_option.clone()) {
            let _ = match stdout!("{: <6}", "") {
                Ok(_) => Ok(()),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::BrokenPipe => Ok(()),
                    _ => Err(e),
                },
            };
            if is_tty || is_force_color {
                let _ = match stdoutln!(
                    "{}",
                    footer_option.truecolor(meta_color[0], meta_color[1], meta_color[2])
                ) {
                    Ok(_) => Ok(()),
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::BrokenPipe => Ok(()),
                        _ => Err(e),
                    },
                };
            } else {
                let _ = match stdoutln!("{}", footer_option) {
                    Ok(_) => Ok(()),
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::BrokenPipe => Ok(()),
                        _ => Err(e),
                    },
                };
            }
        }

        let _ = match stdoutln!() {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::BrokenPipe => Ok(()),
                _ => Err(e),
            },
        };
    } // end main

    // how wide will the print be?
    fn get_num_cols_to_print(cols: usize, vp: Vec<Vec<String>>, term_tuple: (u16, u16)) -> usize {
        let mut last = 0;
        let mut j = format!("{: <6}", "");
        for col in 0..cols {
            let text = vp[0].get(col).unwrap().to_string();
            j.push_str(&text);
            let total_width = j.chars().count();
            let term_width = term_tuple.0 as usize;
            if total_width > term_width {
                break;
            }
            last = col + 1;
        }
        last
    }

    pub mod format_csv {
        use itertools::Itertools;
        use lazy_static::lazy_static;
        use regex::Regex;
        use std::str::FromStr;
        use unicode_truncate::UnicodeTruncateStr;

        mod sigfig {
            use core::str;

            // The general logic and return values in this file were learned from the GNU R package pillar in the sigfig.R file.
            // A special thanks to the great code quality from Hadley Wickham, Jim Hester, and krlmlr
            //
            // Format numbers in decimal notation
            //
            // This formatting system is designed to make it as easy as possible to
            // compare columns of numbers.
            //
            // DecimalSplitsList
            //  val: f64, - the given float
            //  sigfig: i64, - the given sigfigs (default of 3)
            //  neg: bool, - Is a negative needed
            //  lhs: String, - Left-hand-side of decimal string
            //  rhs: f64, - Right-hand-side budget after spending the lhs digits
            //  dec: bool, - should the decimal be included in the print
            //
            //
            //
            //                                                                lhs == 0.0
            //                                   True                            │                            False            100.0
            //                    0.001        ┌─────────────────────────────────┴──────────────────────────────────┐          123.450
            //                   -0.12345      │                                                                    │          123456.0
            //                   -0.01         │                                                       1 +log10(lhs) >= sigfig
            //                                 │                          True                                      │               False
            //          n = ((floor(log10(abs(x))) + 1 - sigfig)          ┌─────────────────────────────────────────┴────────────────────┐
            //          r =(10^n) * round(x / (10^n))                     │                                                              │
            //          return r                                     rhs > 0.0                                                     has negative
            //                                 True                       │          False                       True                    │              False
            //                                 ┌──────────────────────────┴───────────────┐                         ┌────────────────────┴──────────────┐
            //                                 │                                          │                         │                                   │
            //                           has negative                                 has negative               concatonate:                     concatonate:
            //                                 │                                          │                      (-)                              (lhs)
            //                     ┌───────────┴─────────────┐                 ┌──────────┴─────────┐            (lhs)                            (point)
            //                     │                         │                 │                    │            (point)                          + sigfig - log10(lhs) from rhs
            //                     │                         │                                                   + sigfig - log10(lhs) from rhs
            //                     │                         │              concatonate:    concatonate:         (-12.345 -> -12.3)               (12.345 ->  12.3)
            //                     │                         │              (-)             (lhs)                (-1.1 -> -1.10)                  (1.1 -> 1.10)
            //                     │                         │              (lhs)
            //                     │                         │                              (1234.0 -> 1234)
            //                                                         (-1234.0 -> -1234)
            //            concatonate:           concatonate:
            //            (-)                    (lhs)
            //            (lhs)                  (point)
            //            (point)
            //            (-123.45 -> -123.)   (1234.50 -> 1234.)
            //
            //

            pub struct DecimalSplits {
                pub val: f64,
                pub sigfig: i64,
            }

            impl DecimalSplits {
                pub fn value(&self) -> f64 {
                    self.val
                }
                pub fn sig_fig(&self) -> i64 {
                    self.sigfig
                }
                pub fn neg(&self) -> bool {
                    is_neg(self.val)
                }
                pub fn lhs(&self) -> f64 {
                    get_lhs(self.val)
                }
                pub fn rhs(&self) -> f64 {
                    get_rhs(self.val)
                }
                //pub fn dec(&self) -> bool {
                //    is_decimal(self.val)
                //}
                pub fn final_string(&self) -> String {
                    get_final_string(
                        self.value(),
                        self.lhs(),
                        self.rhs(),
                        self.neg(),
                        self.sig_fig(),
                    )
                }
            }

            fn is_neg(x: f64) -> bool {
                x < 0.0
            }

            fn get_lhs(x: f64) -> f64 {
                x.trunc().abs()
            }

            fn get_rhs(x: f64) -> f64 {
                let xint = x.trunc();
                let frac = x - xint;
                frac.abs()
                //let s = format!("{:.12}", frac.abs()); //The 10 is arbitraty, but this condition puts a cap on sigfig size
                //let f: f64 = s.parse::<f64>().unwrap();
                //f
            }

            //fn is_decimal(x: f64) -> bool {
            //    let r: f64 = x.trunc() as f64;
            //    let l = x / r;
            //    l > 1.0
            //}

            pub fn get_final_string(x: f64, lhs: f64, rhs: f64, neg: bool, sigfig: i64) -> String {
                if lhs.abs() + rhs.abs() == 0.0 {
                    "0".to_string()
                } else if lhs == 0.0 {
                    //n = ((floor(log10(abs(x))) + 1 - sigfig)
                    //r =(10^n) * round(x / (10^n))
                    let n = x.abs().log10().floor() + 1.0 - sigfig as f64;
                    let r: f64 = 10f64.powf(n) * ((x / 10f64.powf(n)).round());
                    let tmp_string = r.to_string();
                    if tmp_string.len() > 13 {
                        // 13 is arbitraty. There may be a more general solution here!
                        // Problem: debug val: 0.0001 => final_string: "0.00009999999999999999"
                        let j = (x.abs().log10().floor()).abs() as usize;
                        if j >= sigfig as usize {
                            // long tail sigfigs
                            // 0.0001
                            // 0.001
                            let w = (x.abs().log10().floor()).abs() as usize;
                            let fstring = format!("{:.w$}", r, w = w);
                            fstring
                        } else {
                            // standard lhs only sigs
                            //-0.9527948462413667 -> -0.953
                            let fstring = format!("{:.w$}", r, w = (sigfig as usize));
                            fstring
                        }
                    } else {
                        //println!("{:?}", tmp_string);
                        tmp_string
                    }
                } else if lhs.log10() + 1.0 >= sigfig as f64 {
                    if rhs > 0.0 {
                        let total = lhs + rhs;
                        let total_string = total.to_string();
                        let total_clone = total_string.clone();
                        let split = total_clone.split('.');
                        let vec: Vec<&str> = split.collect();
                        let len_to_take = vec[0].len() + 1; // lhs + point
                        if neg {
                            //concatonate:
                            //(-)
                            //(lhs)
                            //(point)
                            //(-123.45 -> -123.)
                            let pos_string = (total_string[..len_to_take]).to_string();
                            let neg_string = "-".to_string();
                            [neg_string, pos_string].join("")
                        } else {
                            //concatonate:
                            //(lhs)
                            //(point)
                            //(123.45 -> 123.)
                            total_string[..len_to_take].to_string()
                        }
                    } else if neg {
                        //concatonate:
                        //(-)
                        //(lhs)
                        //(-1234.0 -> -1234)
                        let total = lhs + rhs;
                        let total_string = total.to_string();
                        let total_clone = total_string.clone();
                        let split = total_clone.split('.');
                        let vec: Vec<&str> = split.collect();
                        let len_to_take = vec[0].len(); // lhs
                        let pos_string = (total_string[..len_to_take]).to_string();
                        let neg_string = "-".to_string();
                        [neg_string, pos_string].join("")
                    } else {
                        //concatonate:
                        //(lhs)
                        //(1234.0 -> 1234)
                        //(100.0 -> 100)
                        //let total = lhs + rhs;
                        //let total_string = total.to_string();
                        let total_string = x.to_string();
                        let total_clone = total_string.clone();
                        let split = total_clone.split('.');
                        let vec: Vec<&str> = split.collect();
                        let len_to_take = vec[0].len(); // lhs
                        total_string[..len_to_take].to_string()
                    }
                } else if rhs == 0.0 {
                    //concatonate:
                    //(lhs)
                    //(point)
                    //+ sigfig - log10(lhs) from rhs
                    let total_string = x.to_string();
                    let total_clone = total_string.clone();
                    let split = total_clone.split('.');
                    let vec: Vec<&str> = split.collect();
                    let len_to_take_lhs = vec[0].len(); // point -> +1 to sigfig
                    total_string[..len_to_take_lhs].to_string()
                } else if neg {
                    //concatonate:
                    //(-)
                    //(lhs)
                    //(point)
                    //+ sigfig - log10(lhs) from rhs
                    //(-12.345 -> -12.3)
                    //(-1.2345 -> -1.23)
                    // need a rhs arguments here
                    //let total = lhs + rhs;
                    //let total_string = total.to_string();
                    let w: usize = (sigfig as usize) - 1;
                    let x = format!("{:.w$}", x, w = w);
                    let total_string = x.to_string();
                    let total_clone = total_string.clone();
                    let split = total_clone.split('.');
                    let vec: Vec<&str> = split.collect();
                    let len_to_take_lhs = vec[0].len(); // point -> +1 to sigfig
                                                        // The plus one at the end stands for the '.' character as lhs doesn't include it
                    let len_to_take_rhs =
                        std::cmp::min((sigfig as usize) - len_to_take_lhs, vec[1].len()) + 1;
                    let len_to_take = len_to_take_lhs + len_to_take_rhs + 1;
                    //println!("x: {:?}", x);
                    total_string[..len_to_take].to_string()
                } else {
                    //concatonate:
                    //(lhs)
                    //(point)
                    //+ sigfig - log10(lhs) from rhs
                    //(12.345 -> 12.3)
                    //(1.2345 -> 1.23)
                    // need a rhs arguments here
                    //let total = lhs + rhs;
                    //let total_string = total.to_string();
                    let w: usize = (sigfig as usize) - 1;
                    let x = format!("{:.w$}", x, w = w);
                    let total_string = x.to_string();
                    let total_clone = total_string.clone();
                    let split = total_clone.split('.');
                    let vec: Vec<&str> = split.collect();
                    let len_to_take_lhs = vec[0].len(); // point -> +1 to sigfig
                    let len_to_take_rhs = ((sigfig + 1) as usize) - len_to_take_lhs;
                    let len_to_take = len_to_take_lhs + len_to_take_rhs;

                    if len_to_take >= total_string.len() {
                        total_string
                    } else {
                        total_string[..len_to_take].to_string()
                    }
                }
            }

            #[test]
            fn test_f12345() {
                let f12345 = vec![12345.0, 1234.50, 123.45, 12.345, 1.2345, 0.12345, 0.0];
                let test_sigfig = vec![3, 3, 3, 3, 3, 3, 3];
                let test_neg = vec![false, false, false, false, false, false, false];
                let test_lhs = vec![12345.0, 1234.0, 123.0, 12.0, 1.0, 0.0, 0.0];
                let test_rhs = vec![
                    0.0,
                    0.5,
                    0.45000000000000284,
                    0.34500000000000064,
                    0.23449999999999993,
                    0.12345,
                    0.0,
                ];
                //let test_dec = vec![false, true, true, true, true, true, false];
                let test_final_string =
                    vec!["12345", "1234.", "123.", "12.3", "1.23", "0.123", "0"];

                for i in 0..f12345.len() {
                    let value = f12345[i];
                    let x = DecimalSplits {
                        val: value,
                        sigfig: 3,
                    };
                    //println!("{:#?}", list);
                    assert_eq!(x.val, f12345[i]);
                    assert_eq!(x.sigfig, test_sigfig[i]);
                    assert_eq!(x.neg(), test_neg[i]);
                    assert_eq!(x.lhs(), test_lhs[i]);
                    assert_eq!(x.rhs(), test_rhs[i]);
                    //assert_eq!(x.dec(), test_dec[i]);
                    assert_eq!(x.final_string(), test_final_string[i]);
                }
            }

            #[test]
            fn test_f100() {
                let f100 = vec![100.0, 10.0, 1.0, 0.1, 0.01, 0.001, 0.0001];
                let test_sigfig = vec![3, 3, 3, 3, 3, 3, 3];
                let test_neg = vec![false, false, false, false, false, false, false];
                let test_lhs = vec![100.0, 10.0, 1.0, 0.0, 0.0, 0.0, 0.0];
                let test_rhs = vec![0.0, 0.0, 0.0, 0.1, 0.01, 0.001, 0.0001];
                //let test_dec = vec![false, false, false, true, true, true, true];
                let test_final_string = vec!["100", "10", "1", "0.1", "0.01", "0.001", "0.0001"];

                for i in 0..f100.len() {
                    let value = f100[i];
                    println!("{}", value);
                    let x = DecimalSplits {
                        val: value,
                        sigfig: 3,
                    };
                    //println!("{:#?}", list);
                    assert_eq!(x.val, f100[i]);
                    assert_eq!(x.sigfig, test_sigfig[i]);
                    assert_eq!(x.neg(), test_neg[i]);
                    assert_eq!(x.lhs(), test_lhs[i]);
                    assert_eq!(x.rhs(), test_rhs[i]);
                    //assert_eq!(x.dec(), test_dec[i]);
                    assert_eq!(x.final_string(), test_final_string[i]);
                    println!("complete!");
                }
            }

            #[test]
            fn test_fn100() {
                let f100 = vec![-100.0, -10.0, -1.0, -0.1, -0.01, -0.001, -0.0001];
                let test_sigfig = vec![3, 3, 3, 3, 3, 3, 3];
                let test_neg = vec![true, true, true, true, true, true, true];
                let test_lhs = vec![100.0, 10.0, 1.0, 0.0, 0.0, 0.0, 0.0];
                let test_rhs = vec![0.0, 0.0, 0.0, 0.1, 0.01, 0.001, 0.0001];
                //let test_dec = vec![false, false, false, true, true, true, true];
                let test_final_string =
                    vec!["-100", "-10", "-1", "-0.1", "-0.01", "-0.001", "-0.0001"];

                for i in 0..f100.len() {
                    let value = f100[i];
                    println!("{}", value);
                    let x = DecimalSplits {
                        val: value,
                        sigfig: 3,
                    };
                    //println!("{:#?}", list);
                    assert_eq!(x.val, f100[i]);
                    assert_eq!(x.sigfig, test_sigfig[i]);
                    assert_eq!(x.neg(), test_neg[i]);
                    assert_eq!(x.lhs(), test_lhs[i]);
                    assert_eq!(x.rhs(), test_rhs[i]);
                    //assert_eq!(x.dec(), test_dec[i]);
                    assert_eq!(x.final_string(), test_final_string[i]);
                    println!("complete!");
                }
            }

            #[test]
            fn test_fn12345() {
                let f12345 = vec![-12345.0, -1234.50, -123.45, -12.345, -1.2345, -0.12345];
                let test_sigfig = vec![3, 3, 3, 3, 3, 3];
                let test_neg = vec![true, true, true, true, true, true, true];
                let test_lhs = vec![12345.0, 1234.0, 123.0, 12.0, 1.0, 0.0];
                let test_rhs = vec![
                    0.0,
                    0.5,
                    0.45000000000000284,
                    0.34500000000000064,
                    0.23449999999999993,
                    0.12345,
                ];
                //let test_dec = vec![false, true, true, true, true, true];
                let test_final_string =
                    vec!["-12345", "-1234.", "-123.", "-12.3", "-1.23", "-0.123"];

                for i in 0..f12345.len() {
                    let value = f12345[i];
                    let x = DecimalSplits {
                        val: value,
                        sigfig: 3,
                    };
                    //println!("{:#?}", list);
                    assert_eq!(x.val, f12345[i]);
                    assert_eq!(x.sigfig, test_sigfig[i]);
                    assert_eq!(x.neg(), test_neg[i]);
                    assert_eq!(x.lhs(), test_lhs[i]);
                    assert_eq!(x.rhs(), test_rhs[i]);
                    //assert_eq!(x.dec(), test_dec[i]);
                    assert_eq!(x.final_string(), test_final_string[i]);
                }
            }

            #[test]
            fn test_long_double() {
                // the `rhs` break on this test. This is intentional
                // This problem led to the creation of `rhs_string_len` which counts
                // length after the final string has been generated.
                let long_double = vec![-3.33333333, -1.11111111, 3.33333333, 1.11111111];
                let test_sigfig = vec![3, 3, 3, 3];
                let test_neg = vec![true, true, false, false];
                let test_lhs = vec![3.0, 1.0, 3.0, 1.0];
                let _test_rhs = vec![0.33333333, 0.11111111, 0.33333333, 0.11111111];
                //let test_dec = vec![true, true, true, true];
                let test_final_string = vec!["-3.33", "-1.11", "3.33", "1.11"];

                for i in 0..long_double.len() {
                    let value = long_double[i];
                    let x = DecimalSplits {
                        val: value,
                        sigfig: 3,
                    };
                    //println!("{:#?}", list);
                    assert_eq!(x.val, long_double[i]);
                    assert_eq!(x.sigfig, test_sigfig[i]);
                    assert_eq!(x.neg(), test_neg[i]);
                    assert_eq!(x.lhs(), test_lhs[i]);
                    //assert_eq!(list.rhs, test_rhs[i]);
                    //assert_eq!(x.dec(), test_dec[i]);
                    assert_eq!(x.final_string(), test_final_string[i]);
                }
            }

            #[test]
            fn test_norms() {
                // the `rhs` break on this test. This is intentional
                // This problem led to the creation of `rhs_string_len` which counts
                // length after the final string has been generated.
                let long_double = vec![
                    -0.7949012411113556,
                    1.1597467493978901,
                    -0.9527948462413667,
                    -1.2055600489348273,
                    -0.9964310089596907,
                    0.3968466566707523,
                    -0.7763342862202715,
                    0.6893169466075251,
                    //-0.8700625714479723,
                ];
                let test_sigfig = vec![
                    3, 3, 3, 3, 3, 3, 3, 3,
                    //3
                ];
                let test_lhs = vec![
                    0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                    //0.0
                ];
                //let test_dec = vec![true, true, true, true];
                let test_final_string = vec![
                    "-0.795", "1.16", "-0.953", "-1.21", "-0.996", "0.397", "-0.776",
                    "0.689",
                    //"-0.870",
                ];

                for i in 0..long_double.len() {
                    let value = long_double[i];
                    let x = DecimalSplits {
                        val: value,
                        sigfig: 3,
                    };
                    //println!("{:#?}", list);
                    assert_eq!(x.val, long_double[i]);
                    assert_eq!(x.sigfig, test_sigfig[i]);
                    //assert_eq!(x.neg(), test_neg[i]);
                    assert_eq!(x.lhs(), test_lhs[i]);
                    //assert_eq!(list.rhs, test_rhs[i]);
                    //assert_eq!(x.dec(), test_dec[i]);
                    assert_eq!(x.final_string(), test_final_string[i]);
                }
            }
        }

        /// Represents the type of a value.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ValueType {
            Boolean,
            Integer,
            Double,
            Date,
            Time,
            DateTime,
            Character,
            /// A missing value.
            Na,
        }

        pub fn is_logical(text: &str) -> bool {
            // col_logical -l, T,F,TRUE,FALSE,True,False,true,false,t,f,1,0
            lazy_static! {
                static ref R: Regex = Regex::new(
                    r"^true$|^false$|^t$|^f$|TRUE$|^FALSE$|^T$|^F$|^True|^False|^1$|^0$"
                )
                .unwrap();
            }
            R.is_match(text)
        }

        pub fn is_integer(text: &str) -> bool {
            //let integer = "5";
            lazy_static! {
                static ref R: Regex = Regex::new(r"^\s*([+-]?[1-9][0-9]*|0)\s*$").unwrap();
            }
            R.is_match(text)
        }

        pub fn is_number(text: &str) -> bool {
            is_integer(text) || is_double(text)
        }

        pub fn is_negative_number(text: &str) -> bool {
            lazy_static! {
                static ref R: Regex = Regex::new(r"^\s*-[0-9]*.?[0-9]*\s*$").unwrap();
            }
            R.is_match(text)
        }

        pub fn is_double(text: &str) -> bool {
            f64::from_str(text.trim()).is_ok()
        }

        pub fn is_time(text: &str) -> bool {
            //let time = "11:59:37 UTC";
            //https://stackoverflow.com/a/25873711
            lazy_static! {
                static ref R: Regex =
                    Regex::new(r"^(?:[01][0-9]|2[0123]):(?:[012345][0-9]):(?:[012345][0-9])$")
                        .unwrap();
            }
            R.is_match(text)
        }

        pub fn is_date(text: &str) -> bool {
            lazy_static! {
                static ref R: Regex = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
            }
            R.is_match(text)
        }

        pub fn is_date_time(text: &str) -> bool {
            //let datetime = "2020-10-09 11:59:37 UTC";
            //https://stackoverflow.com/a/25873711
            lazy_static! {
                static ref R: Regex =
                    Regex::new(r"^(?:[01][0-9]|2[0123]):(?:[012345][0-9]):(?:[012345][0-9])")
                        .unwrap();
            }
            R.is_match(text)
        }

        pub fn is_na(text: &str) -> bool {
            lazy_static! {
                static ref R: Regex = Regex::new(
                    r"^$|^(?:N(?:(?:(?:one|AN|a[Nn]|/A)|[Aa])|ull)|n(?:ull|an?|/a?)|(?:missing))$"
                )
                .unwrap();
            }
            R.is_match(text)
        }

        pub fn is_na_string_padded(text: &str) -> bool {
            lazy_static! {
                static ref R: Regex = Regex::new(
                    r"^$|(^|\s)(?:N(?:(?:(?:AN|a[Nn]|/A)|[Aa])|ull)|n(?:ull|an?|/a?)|(?:missing))\s*$"
                )
                .unwrap();
            }
            R.is_match(text)
        }

        // utilities

        pub fn infer_type_from_string(text: &str) -> ValueType {
            if is_time(text) {
                ValueType::Time
            } else if is_logical(text) {
                ValueType::Boolean
            } else if is_integer(text) {
                ValueType::Integer
            } else if is_date_time(text) {
                ValueType::DateTime
            } else if is_date(text) {
                ValueType::Date
            } else if is_double(text) {
                ValueType::Double
            } else if text.is_empty() | is_na(text) {
                ValueType::Na
            } else {
                ValueType::Character
            }
        }

        pub fn format_strings(
            vec_col: &[&str],
            lower_column_width: usize,
            upper_column_width: usize,
            sigfig: i64,
        ) -> Vec<String> {
            let ellipsis = '\u{2026}';

            let strings_and_fracts: Vec<(String, usize, usize)> = vec_col
                .iter()
                .map(|&string| format_if_na(string))
                .map(|string| format_if_num(&string, sigfig))
                .map(|string| {
                    // the string, and the length of its fractional digits if any
                    let (lhs, rhs) = if is_double(&string) {
                        let mut split = string.split('.');
                        (
                            split.next().map(|lhs| lhs.len()).unwrap_or_default(),
                            split.next().map(|rhs| rhs.len()).unwrap_or_default(),
                        )
                    } else {
                        (0, 0)
                    };
                    (string, lhs, rhs)
                })
                .collect();

            let max_fract: usize = strings_and_fracts
                .iter()
                .map(|(_, _, fract)| *fract)
                .max()
                .unwrap_or_default();
            let max_whole: usize = strings_and_fracts
                .iter()
                .map(|(_, whole, _)| *whole)
                .max()
                .unwrap_or_default();

            let strings_and_widths: Vec<(String, usize)> = strings_and_fracts
                .into_iter()
                .map(|(mut string, whole, fract)| {
                    if max_fract > 0 && is_double(&string) {
                        if whole < max_whole {
                            let mut s = String::new();
                            s.push_str(&" ".repeat(max_whole - whole));
                            s.push_str(&string);
                            string = s;
                        }

                        string.push_str(&" ".repeat(max_fract - fract));
                    } else if max_fract > 0 && is_na(&string) {
                        if 2 < max_whole {
                            let mut s = String::new();
                            s.push_str(&" ".repeat(max_whole - 2));
                            s.push_str(&string);
                            string = s;
                        }

                        string.push_str(&" ".repeat(max_fract - fract));
                    }
                    let len = string.chars().count();
                    // the string and its length
                    (string, len)
                })
                .collect();

            let max_width: usize = strings_and_widths
                .iter()
                .map(|(_, width)| *width)
                .max()
                .unwrap_or_default()
                .clamp(lower_column_width, upper_column_width);

            strings_and_widths
                .into_iter()
                .map(|(string, len)| {
                    if len > max_width {
                        let (rv, _) = string.unicode_truncate(max_width - 1);
                        let spacer: &str = &" ";
                        let string_and_ellipses = [rv.to_string(), ellipsis.to_string()].join("");
                        [string_and_ellipses, spacer.to_string()].join("")
                    } else {
                        let add_space = max_width - len + 1;
                        let borrowed_string: &str = &" ".repeat(add_space);
                        [string, "".to_string()].join(borrowed_string)
                    }
                })
                .collect()
        }

        pub fn format_if_na(text: &str) -> String {
            // todo add repeat strings for NA
            let missing_string_value = "NA";
            let string = if is_na(text) {
                missing_string_value
            } else {
                text
            };
            string.to_string()
        }

        pub fn format_if_num(text: &str, sigfig: i64) -> String {
            if let Ok(val) = text.parse::<f64>() {
                sigfig::DecimalSplits { val, sigfig }.final_string()
            } else {
                text.to_string()
            }
        }

        pub fn get_col_data_type(col: &[&str]) -> ValueType {
            // counts the frequency of the datatypes in the column
            // returns the most frequent while ignoring NA values.
            col.iter()
                .map(|x| infer_type_from_string(x))
                .filter(|x| !matches!(x, &ValueType::Na))
                .group_by(|&x| x)
                .into_iter()
                .map(|(key, group)| (key, group.count()))
                .max_by_key(|&(_, count)| count)
                .map(|(key, _)| key)
                .unwrap()
        }

        pub fn parse_delimiter(src: &str) -> Result<u8, String> {
            let bytes = src.as_bytes();
            match *bytes {
                [del] => Ok(del),
                [b'\\', b't'] => Ok(b'\t'),
                _ => Err(format!(
                    "expected one byte as delimiter, got {} bytes (\"{}\")",
                    bytes.len(),
                    src
                )),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::print::format_csv::parse_delimiter;
    #[test]
    fn one_byte_delimiter() {
        assert_eq!(parse_delimiter(","), Ok(b','));
        assert_eq!(parse_delimiter(";"), Ok(b';'));
        assert_eq!(parse_delimiter("|"), Ok(b'|'));
        assert_eq!(parse_delimiter(" "), Ok(b' '));
        assert_eq!(parse_delimiter("\t"), Ok(b'\t'));
    }

    #[test]
    fn tab_delimiter() {
        assert_eq!(parse_delimiter("\\t"), Ok(b'\t'));
    }

    #[test]
    fn delimiter_wrong_length() {
        assert_eq!(
            parse_delimiter(""),
            Err("expected one byte as delimiter, got 0 bytes (\"\")".to_string())
        );
        assert_eq!(
            parse_delimiter("too long"),
            Err("expected one byte as delimiter, got 8 bytes (\"too long\")".to_string())
        );
        assert_eq!(
            parse_delimiter("\\n"),
            Err("expected one byte as delimiter, got 2 bytes (\"\\n\")".to_string())
        );
    }
}
