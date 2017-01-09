extern crate jubatus;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::f64;
use jubatus::anomaly::client::AnomalyClient;
use jubatus::common::datum::Datum;

static NAME: &'static str = "anom_kddcup";

fn main() {
    let mut anom = AnomalyClient::new("127.0.0.1:9199", NAME);

    let path = "kddcup.data_10_percent.txt";
    let f = match File::open(path) {
        Err(why) => panic!("could not open: {}", why),
        Ok(file) => file,
    };

    let buf = BufReader::new(&f);
    for line in buf.lines() {
        let l = line.unwrap();
        let c: Vec<&str> = l.split(",").collect();
        let duration = c[0];
        let protocol_type = c[1];
        let service = c[2];
        let flag = c[3];
        let src_bytes = c[4];
        let dst_bytes = c[5];
        let land = c[6];
        let wrong_fragment = c[7];
        let urgent = c[8];
        let hot = c[9];
        let num_failed_logins = c[10];
        let logged_in = c[11];
        let num_compromised = c[12];
        let root_shell = c[13];
        let su_attempted = c[14];
        let num_root = c[15];
        let num_file_creations = c[16];
        let num_shells = c[17];
        let num_access_files = c[18];
        let num_outbound_cmds = c[19];
        let is_host_login = c[20];
        let is_guest_login = c[21];
        let count = c[22];
        let srv_count = c[23];
        let serror_rate = c[24];
        let srv_serror_rate = c[25];
        let rerror_rate = c[26];
        let srv_rerror_rate = c[27];
        let same_srv_rate = c[28];
        let diff_srv_rate = c[29];
        let srv_diff_host_rate = c[30];
        let dst_host_count = c[31];
        let dst_host_srv_count = c[32];
        let dst_host_same_srv_rate = c[33];
        let dst_host_diff_srv_rate = c[34];
        let dst_host_same_src_port_rate = c[35];
        let dst_host_srv_diff_host_rate = c[36];
        let dst_host_serror_rate = c[37];
        let dst_host_srv_serror_rate = c[38];
        let dst_host_rerror_rate = c[39];
        let dst_host_srv_rerror_rate = c[40];
        let label = c[41];

        let mut d = Datum::new();
        for (k, v) in vec![("protocol_type", protocol_type),
                           ("service", service),
                           ("flag", flag),
                           ("land", land),
                           ("logged_in", logged_in),
                           ("is_host_login", is_host_login),
                           ("is_guest_login", is_guest_login)]
            .into_iter() {
            d.add_string(k, v);
        }

        for (k, v) in vec![("duration", duration.parse::<f64>().unwrap()),
                           ("src_bytes", src_bytes.parse::<f64>().unwrap()),
                           ("dst_bytes", dst_bytes.parse::<f64>().unwrap()),
                           ("wrong_fragment", wrong_fragment.parse::<f64>().unwrap()),
                           ("urgent", urgent.parse::<f64>().unwrap()),
                           ("hot", hot.parse::<f64>().unwrap()),
                           ("num_failed_logins", num_failed_logins.parse::<f64>().unwrap()),
                           ("num_compromised", num_compromised.parse::<f64>().unwrap()),
                           ("root_shell", root_shell.parse::<f64>().unwrap()),
                           ("su_attempted", su_attempted.parse::<f64>().unwrap()),
                           ("num_root", num_root.parse::<f64>().unwrap()),
                           ("num_file_creations", num_file_creations.parse::<f64>().unwrap()),
                           ("num_shells", num_shells.parse::<f64>().unwrap()),
                           ("num_access_files", num_access_files.parse::<f64>().unwrap()),
                           ("num_outbound_cmds", num_outbound_cmds.parse::<f64>().unwrap()),
                           ("count", count.parse::<f64>().unwrap()),
                           ("srv_count", srv_count.parse::<f64>().unwrap()),
                           ("serror_rate", serror_rate.parse::<f64>().unwrap()),
                           ("srv_serror_rate", srv_serror_rate.parse::<f64>().unwrap()),
                           ("rerror_rate", rerror_rate.parse::<f64>().unwrap()),
                           ("srv_rerror_rate", srv_rerror_rate.parse::<f64>().unwrap()),
                           ("same_srv_rate", same_srv_rate.parse::<f64>().unwrap()),
                           ("diff_srv_rate", diff_srv_rate.parse::<f64>().unwrap()),
                           ("srv_diff_host_rate", srv_diff_host_rate.parse::<f64>().unwrap()),
                           ("dst_host_count", dst_host_count.parse::<f64>().unwrap()),
                           ("dst_host_srv_count", dst_host_srv_count.parse::<f64>().unwrap()),
                           ("dst_host_same_srv_rate", dst_host_same_srv_rate.parse::<f64>().unwrap()),
                           ("dst_host_same_src_port_rate", dst_host_same_src_port_rate.parse::<f64>().unwrap()),
                           ("dst_host_diff_srv_rate", dst_host_diff_srv_rate.parse::<f64>().unwrap()),
                           ("dst_host_srv_diff_host_rate", dst_host_srv_diff_host_rate.parse::<f64>().unwrap()),
                           ("dst_host_serror_rate", dst_host_serror_rate.parse::<f64>().unwrap()),
                           ("dst_host_srv_serror_rate", dst_host_srv_serror_rate.parse::<f64>().unwrap()),
                           ("dst_host_rerror_rate", dst_host_rerror_rate.parse::<f64>().unwrap()),
                           ("dst_host_srv_rerror_rate", dst_host_srv_rerror_rate.parse::<f64>().unwrap())]
            .into_iter() {
            d.add_number(k, v);
        }

        let ret = anom.add(d);

        if !ret.score.is_infinite() && ret.score != 1.0 {
            println!("{:?} {}", ret, label);
        }
    }
}
