extern crate jubatus;
extern crate rand;

use jubatus::classifier::client::ClassifierClient;
use jubatus::classifier::types::LabeledDatum;
use jubatus::common::datum::Datum;
use rand::{thread_rng, Rng};


fn train(client: &mut ClassifierClient) {
    let mut train_data = vec![];
    for n in vec!["家康", "秀忠", "家光", "家綱", "綱吉", "家宣", "家継", "吉宗", "家重", "家治",
                  "家斉", "家慶", "家定", "家茂"] {
        let mut d = Datum::new();
        d.add_string("name", n);
        train_data.push(LabeledDatum {
            label: "徳川".to_string(),
            data: d,
        });
    }
    // (u'徳川', Datum({'name': u'慶喜'})),

    for n in vec!["尊氏", "義詮", "義満", "義持", "義量", "義教", "義勝", "義政", "義尚", "義稙",
                  "義澄", "義稙", "義晴", "義輝", "義栄"] {
        let mut d = Datum::new();
        d.add_string("name", n);
        train_data.push(LabeledDatum {
            label: "足利".to_string(),
            data: d,
        });
    }
    // (u'足利', Datum({'name': u'義昭'})),

    for n in vec!["時政", "義時", "泰時", "経時", "時頼", "長時", "政村", "時宗", "貞時", "師時",
                  "宗宣", "煕時", "基時", "高時", "貞顕"] {
        let mut d = Datum::new();
        d.add_string("name", n);
        train_data.push(LabeledDatum {
            label: "北条".to_string(),
            data: d,
        });
    }
    // (u'北条', Datum({'name': u'守時'})),

    // training data must be shuffled on online learning!
    let mut rng = thread_rng();
    rng.shuffle(&mut train_data);

    // run train
    client.train(train_data);
}

fn predict(client: &mut ClassifierClient) {
    // predict the last shogun
    for n in vec!["慶喜", "義昭", "守時"] {
        let mut d = Datum::new();
        d.add_string("name", n);
        let mut res = client.classify(vec![d]);
        // get the predicted shogun name
        let ref shogun_name = res[0].iter_mut().max_by_key(|x| (x.score * 100.) as i64).unwrap().label;
        println!("{} {}", shogun_name, n);
    }
}

fn main() {
    let host = "127.0.0.1:9199";
    let name = "test";

    let mut client = ClassifierClient::new(host, name);
    train(&mut client);
    predict(&mut client);
}
