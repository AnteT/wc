use std::path::PathBuf;

use clap::{value_parser, Arg, ArgAction, Command};

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct WCArgs {
    pub filename: PathBuf,
    pub top_words: Option<usize>, // Optional field for top N word frequencies
    pub ignore_words: Option<Vec<String>>, // Optional words to ignore for frequency mapping
    pub is_cased: bool, // Do not ignore character casing when checking token equality for word frequency
    pub is_include_all: bool, // Include punctuation characters or other non [A-Z] characters in word mapping
    pub is_include_stopwords: bool, // Ignore stopwords like 'if', 'the', 'let', 'a', 'you', etc when counting word frequencies
    pub is_include_unique: bool, // Include unique word count in output summary
}

pub fn parse_args() -> WCArgs {
    let matches = Command::new("Word Counter Program")
        .version(VERSION.unwrap_or("Unknown"))
        .author("Ante Tonkovic-Capin")
        .about("Counts the number of space-separated tokens in the provided file, returning the total count with additional metrics like word frequencies.")
        .after_help("For example, run `wc file.txt` to count the number of words in `file.txt`.")
        /* Positional arguments */
        .arg(Arg::new("file")
            .help("The target file to open and count")
            .value_name("FILENAME")
            .required(true)
            .value_parser(value_parser!(PathBuf))
            .action(ArgAction::Set)
            .index(1))        
        /* Argument for top N word frequencies */
        .arg(Arg::new("top")
            .long("top")
            .short('n')
            .short_alias('N')
            .alias("freq")
            .value_name("N")
            .required(false)
            .value_parser(value_parser!(usize))
            .display_order(0)
            .help("Displays the top N most frequent words")
            .action(ArgAction::Set))
        /* Argument for ignoring specific words */
        .arg(Arg::new("ignore")
             .short('I')
             .short_alias('i')
             .long("ignore")
             .value_name("WORD, ..., WORD")
             .value_delimiter(',')
             .display_order(1)
             .action(ArgAction::Append)
             .help("Ignore specific words or tokens for frequency counts"))               
        .arg(Arg::new("cased")
            .long("cased")
            .short('c')
            .short_alias('C')
            .alias("case-sensitive")
            .help("Do not ignore character cases for word frequency")
            .action(ArgAction::SetTrue))
        .arg(Arg::new("include-all")
            .long("include-all")
            .short('a')
            .short_alias('A')
            .alias("all")
            .help("Includes all non-ASCII alphabet characters in word tokens")
            .action(ArgAction::SetTrue))
        .arg(Arg::new("include-stopwords")
            .long("include-stopwords")
            .short('s')
            .short_alias('S')
            .alias("stopwords")
            .help("Includes common stopwords for word frequency")
            .action(ArgAction::SetTrue))   
        .arg(Arg::new("include-unique")
            .long("include-unique")
            .short('u')
            .short_alias('U')
            .alias("unique")
            .help("Includes unique word count in output summary")
            .action(ArgAction::SetTrue))                        
        .get_matches();

    let filename = matches.get_one::<PathBuf>("file").map(|p| p.to_path_buf()).unwrap();
    let top_words = matches.get_one::<usize>("top").copied(); // Get the top N value, if provided
    let is_cased = matches.get_flag("cased");
    let ignore_words: Option<Vec<String>> = matches.get_many::<String>("ignore").map_or_else(|| None, |v| Some(v
        .filter(|s| !s.is_empty())
        .map(|w| {
                if is_cased { w.trim().to_owned() } else { w.trim().to_ascii_lowercase().to_owned() }
            }).collect::<Vec<_>>()));
    let is_include_all = matches.get_flag("include-all");
    let is_include_stopwords = matches.get_flag("include-stopwords");
    let is_include_unique = matches.get_flag("include-unique");
    
    // Return the primary struct
    WCArgs { filename, top_words, ignore_words, is_cased, is_include_all, is_include_stopwords, is_include_unique }
}