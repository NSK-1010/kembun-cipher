use structopt::StructOpt;

mod encode;
mod decode;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "e", long = "encode", default_value = "")]
    text: String,
    #[structopt(short = "d", long = "decode", default_value = "")]
    file: String,
}

fn main() {
    // 平文はǷind is with you.
    let opt = Opt::from_args();
    let text: &str = &opt.text;
    let file: &str = &opt.file;
    if text != "" {
        if let Err(err) = encode::encode(text) {
            println!("{:?}", err);
        }
    } else if file != "" {
        decode::decode(file);
    }
}