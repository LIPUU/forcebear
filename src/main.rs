mod generator;

use generator::*;
#[derive(Debug)]
pub struct LeadingChars(String);
static INSTANCE: OnceCell<LeadingChars> = OnceCell::new();
impl LeadingChars {
    pub fn global() -> &'static LeadingChars {
        INSTANCE.get().expect("LeadingChars is not initialized")
    }

    pub fn from(s: String) -> LeadingChars {
        LeadingChars(s)
    }
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    // leading chars of address you want
    #[clap(short='s', long, default_value_t = String::from("0000"))]
    leading_chars: String,

    // mnemonic count, 12 or 24
    #[clap(short = 'l', long, default_value_t = 12)]
    words: i32,

    // if
    #[clap(short = 'p', long)]
    password: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let cpu_nums = num_cpus::get_physical();
    let leading_chars = LeadingChars::from(args.leading_chars);
    INSTANCE.set(leading_chars).unwrap();
    let (sender, mut receiver) = mpsc::unbounded_channel::<(Mnemonic, String)>();
    let sender = Arc::new(sender);
    let word = {
        if args.words == 12 {
            Count::Words12
        } else {
            Count::Words24
        }
    };
    let senders: Vec<_> = (0..cpu_nums).map(|_| Arc::clone(&sender)).collect();

    let _works = (0..cpu_nums)
        .map(|i| {
            let sender = senders[i].clone();
            tokio::task::spawn_blocking(move || {
                generate_address(word, sender, LeadingChars::global())
            })
        })
        .collect::<Vec<_>>();

    while let Some((mnemonic, addr)) = receiver.recv().await {
        let addr = format!("0x{}", addr);
        println!("💍BIP39: {}", &mnemonic.phrase().to_string());
        println!("😀addr: {}\n", addr);
    }
}
