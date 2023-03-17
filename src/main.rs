extern crate prettytable;
use clap::Parser;
use prettytable::{Cell, Row, Table};

#[derive(Parser, Debug)]
#[command(name = "terrific table")]
#[command(author = "Xinchang Zheng (zhengxinchang@big.ac.cn)")]
#[command(version = "1.0")]
#[command(about = "
Rust based table formatter for tabular data
Author: Xinchang Zheng <zhengxinchang@big.ac.cn>
------------------------------------------------
How to use:

<some command> | ttable         // Will use \t as delimiter as default.
<some command> | ttable -d ','  // Will use user specified ',' as delimiter.
<some command> | ttable -r      // Will show row index.
<some command> | ttable -c      // Will show column index.
<some command> | ttable -a      // Will show row and column index.
", long_about = None)]
struct Cli {
    /// delimiter default is \t
    #[arg(short, long)]
    delimiter: Option<char>,
    /// show row index
    #[arg(short, long)]
    rowidx: bool,
    /// show column index
    #[arg(short, long)]
    colidx: bool,
    /// show row and column index
    #[arg(short = 'a', long)]
    index: bool,
    /// max rows to show
    #[arg(short, long, default_value_t = 50000)]
    maxrow: u64,
}

fn main() {
    let cli = Cli::parse();
    let show_col_idx: bool;
    let show_row_idx: bool;
    if cli.index {
        show_col_idx = true;
        show_row_idx = true;
    } else {
        show_col_idx = cli.colidx;
        show_row_idx = cli.rowidx;
    }

    let delimiter = cli.delimiter.unwrap_or('\t');
    let mut msg_table = Table::new();
    let mut max_col = 0;
    let mut line_num = 0;
    loop {
        let mut line = String::new();
        let line_len = std::io::stdin().read_line(&mut line).unwrap();
        line_num += 1;
        if line_len == 0 || line_num > cli.maxrow {
            break;
        }
        let line_vec = line.split(delimiter).collect::<Vec<&str>>();
        if line_vec.len() > max_col {
            max_col = line_vec.len();
        }
        let mut row = line_vec.iter().map(|x| Cell::new(x)).collect::<Vec<Cell>>();
        if show_row_idx {
            row.insert(0, Cell::new(&line_num.to_string()));
        }
        msg_table.add_row(Row::new(row));
    }

    let mut col_title = Vec::new();
    if show_col_idx {
        if show_row_idx {
            for x in 0..(max_col + 1) {
                col_title.push(Cell::new(&x.to_string()));
            }
        } else {
            for x in 1..(max_col + 1) {
                col_title.push(Cell::new(&x.to_string()));
            }
        }
        msg_table.set_titles(Row::new(col_title));
    }
    msg_table.printstd();
}
