pub struct Settings {
    pub headless: bool,
    pub count: usize,
    pub target: String,
    pub output_filename: String,
}

const FLAGS: [&str;3] = [
    "--headless",
    "--frames=20",
    "--output=gif.gif"
];

impl Settings {
    pub fn from_args() -> Settings {
        let mut iter = std::env::args();

        let target = iter.nth(1).expect("provide url");
        if &target == "-h" {
            for arg in FLAGS {
                println!("{}", arg);
            }
            std::process::exit(0);
        }
        let mut headless = false;
        let mut count = 20usize;
        let mut output_filename = "gif.gif".to_string();

        for arg in iter {
            if &arg == "--headless" {
                headless = true;
            } else if arg.contains("--frames") {
                let s = arg.split('=');
                count = s.last().unwrap().parse::<usize>().expect("couldnt read frames input");
            } else if arg.contains("--output") {
                let s = arg.split('=');
                output_filename = s.last().unwrap().to_string();
            } else {
                panic!("unknown argument: {}", arg);
            }
        }

        Settings {
            headless,
            count,
            target,
            output_filename,
        }
    }
}
