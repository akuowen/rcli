use crate::opt::GenPassOps;
use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>?/";

pub fn process_passgen(opts: &GenPassOps) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut passwd: Vec<u8> = Vec::new();
    let mut chars = Vec::new();
    if opts.uppercase {
        chars.extend_from_slice(UPPER);
        passwd.push(*UPPER.choose(&mut rng).expect("empty char"));
    }
    if opts.lowercase {
        chars.extend_from_slice(LOWER);
        passwd.push(*LOWER.choose(&mut rng).expect("empty char"));
    }

    if opts.number {
        chars.extend_from_slice(NUMBER);
        passwd.push(*NUMBER.choose(&mut rng).expect("empty char"));
    }

    if opts.symbol {
        chars.extend_from_slice(SYMBOL);
        passwd.push(*SYMBOL.choose(&mut rng).expect("empty char·"));
    }
    for _ in 0..opts.length - passwd.len() as u8 {
        passwd.push(*chars.choose(&mut rng).expect("empty char"));
    }
    passwd.shuffle(&mut rng);

    let passwdstr = String::from_utf8(passwd)?;
    println!("{}", passwdstr);
    let zxcvbn = zxcvbn(&passwdstr, &[])?;
    eprintln!("密码强度:{}", zxcvbn.score());
    Ok(())
}
