extern crate rustc_serialize;
extern crate docopt;

mod evaluator;

use docopt::*;
use evaluator::{Evaluator, VariablesContext};

const USAGE: &'static str = "
SIRS epidemic model solver using Euler's method.

Usage:
  sirs_solver --t-start=<b_value> --t-end=<e_value> --t-delta=<d_value> --s-zero=<s_value> --i-zero=<i_value> --r-zero=<r_value> --s-rate=<s_rate_value> --i-rate=<i_rate_value> --r-rate=<r_rate_value>
  sirs_solver (-h | --help)
  sirs_solver --version

Options:
  -h --help                Show this screen.
  --version                Show version.
  --t-start=<b_value>      Begin of time [default: 0].
  --t-end=<e_value>        End of time.
  --t-delta=<d_value>      Delta time [default: 0.01].
  --s-zero=<s_value>       Number of suspecible at the beginning.
  --i-zero=<i_value>       Number of infected at the beginning.
  --r-zero=<r_value>       Number of recovered at the beginning.
  --s-rate=<s_rate_value>  Suspecion rate of the disease.
  --i-rate=<i_rate_value>  Infection rate of the disease.
  --r-rate=<r_rate_value>  Recovery rate of the disease.
";

#[derive(RustcDecodable)]
struct Args {
    flag_version: bool,
    flag_s_zero: Option<f32>,
    flag_i_zero: Option<f32>,
    flag_r_zero: Option<f32>,
    flag_t_start: Option<f32>,
    flag_t_end: Option<f32>,
    flag_t_delta: Option<f32>,
    flag_s_rate: Option<f32>,
    flag_i_rate: Option<f32>,
    flag_r_rate: Option<f32>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("sirs_solver v 0.2.0");
        return;
    }

    let infection_rate = args.flag_i_rate.unwrap_or_else(|| {panic!("infection_rate is not defined")});
    let recovery_rate = args.flag_r_rate.unwrap_or_else(|| {panic!("recovery_rate is not defined")});
    let imm_off_rate = args.flag_s_rate.unwrap_or_else(|| {panic!("susception_rate is not defined")});

    let s_b = args.flag_s_zero.unwrap_or_else(|| {panic!("s_b is not defined")});
    let i_b = args.flag_i_zero.unwrap_or_else(|| {panic!("i_b is not defined")});
    let r_b = args.flag_r_zero.unwrap_or_else(|| {panic!("r_b is not defined")});
    let n = s_b + i_b + r_b;

    let t_begin = args.flag_t_start.unwrap_or_else(|| {panic!("t_begin is not defined")});
    let t_end = args.flag_t_end.unwrap_or_else(|| {panic!("t_end is not defined")});
    let t_delta = args.flag_t_delta.unwrap_or_else(|| {panic!("t_delta is not defined")});

    let s_func = move |data: &VariablesContext| {
        data["S"] + (- infection_rate * data["S"] * data["I"] / n + imm_off_rate * data["R"]) * t_delta
    };
    let i_func = move |data: &VariablesContext| {
        data["I"] + (infection_rate * data["S"] * data["I"] / n - recovery_rate * data["I"]) * t_delta
    };
    let r_func = move |data: &VariablesContext| {
        data["R"] + ( recovery_rate * data["I"] - imm_off_rate * data["R"]) * t_delta
    };

    let mut e = Evaluator::new();
    e.add_start_value("S".to_string(), s_b);
    e.add_start_value("I".to_string(), i_b);
    e.add_start_value("R".to_string(), r_b);
    e.add_function("S".to_string(), Box::new(s_func));
    e.add_function("I".to_string(), Box::new(i_func));
    e.add_function("R".to_string(), Box::new(r_func));

    e.evaluate(t_begin, t_end, t_delta);

    let s: Vec<f32> = e.get_data_vec("S").unwrap();
    let i: Vec<f32> = e.get_data_vec("I").unwrap();
    let r: Vec<f32> = e.get_data_vec("R").unwrap();

    println!("t,S,I,R");
    for iter in 0..s.len() {
        println!("{}, {}, {}, {}",iter, s[iter], i[iter], r[iter] );
    }
}
