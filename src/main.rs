use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main()
{
    write!(io::stdout(), "Input the target you want to hack: ").unwrap();
    io::stdout().flush().unwrap();

    let mut target = String::new();
    io::stdin().read_line(&mut target).unwrap();
    target = target.trim().to_string();

    write!(io::stdout(), "Start to hack {target}:\n").unwrap();
    io::stdout().flush().unwrap();

    for i in 1..11
    {
        sleep(Duration::from_secs(1));
        write!(io::stdout(), "Hacking {target} {i}0%...\n").unwrap();
        io::stdout().flush().unwrap();
    }

    write!(io::stdout(), "{target} has been hacked.\n").unwrap();
}
