use std::fs::{read_to_string, metadata};
use std::io;
use std::collections::HashMap;
use wclib::{self};

mod args;

/// A list of 184 common stopwords to ignore when doing word frequency mapping.
const STOPWORDS: [&str; 184] = ["a","about","above","actually","after","again","against","all","almost","also","although","always","am","an","and","any","are","as","at","be","became","become","because","been","before","being","below","between","both","but","by","can","could","did","do","does","doing","down","during","each","either","else","few","for","from","further","had","has","have","having","he","he'd","he'll","hence","he's","her","here","here's","hers","herself","him","himself","his","how","how's","I","I'd","I'll","I'm","I've","if","in","into","is","it","it's","its","itself","just","let's","may","maybe","me","might","mine","more","most","must","my","myself","neither","nor","not","of","oh","on","once","only","ok","or","other","ought","our","ours","ourselves","out","over","own","same","she","she'd","she'll","she's","should","so","some","such","than","that","that's","the","their","theirs","them","themselves","then","there","there's","these","they","they'd","they'll","they're","they've","this","those","through","to","too","under","until","up","very","was","we","we'd","we'll","we're","we've","were","what","what's","when","whenever","when's","where","whereas","wherever","where's","whether","which","while","who","whoever","who's","whose","whom","why","why's","will","with","within","would","yes","yet","you","you'd","you'll","you're","you've","your","yours","yourself","yourselves"];

fn main() -> io::Result<()> {
    let args = args::parse_args();

    // Check if the provided path is a directory
    if let Ok(meta) = metadata(&args.filename) {
        if meta.is_dir() {
            eprintln!("Provided filename {:?} appears to be a directory and not a file.", args.filename);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Input is a directory, not a file."));
        }
    } else {
        // Handle the case where metadata retrieval failed
        eprintln!("Error retrieving metadata for {:?}: file may not exist.", args.filename);
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found."));
    }

    // Attempt to read the file and handle errors gracefully
    let raw_contents = match read_to_string(&args.filename) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file {:?}: {}", args.filename, e);
            return Err(e); // Return the error to the caller
        }
    };

    // Count the number of lines in the raw content that are not empty
    let line_count = raw_contents.lines().filter(|l| !l.is_empty()).count();

    let mut word_map: HashMap<String, usize> = HashMap::new();

    // Count the top N word frequencies and display them along with the counts
    for word in raw_contents.split_whitespace() {
        let word = if !args.is_cased { word.to_ascii_lowercase() } else { word.into() };
        let word = if args.is_include_all { word } else { word.chars().filter(|c| c.is_alphanumeric() || *c == '\'').collect() };
        if word.is_empty() {
            continue;
        }
        *word_map.entry(word).or_insert_with(|| 0) += 1;
    }

    // Get the top N
    let top_n = args.top_words.unwrap_or_default(); // Safe to unwrap since it's checked above

    let unique_count = word_map.len();
    let mut word_count = 0_usize;
    word_map.iter().for_each(|(_, c)| word_count += *c);

    // Generate and print the frequency table
    let ignore_words = if !args.is_include_stopwords { STOPWORDS.as_ref() } else { &[""] };
    // let fmt_freq_table = wclib::generate_frequency_table(&word_map, top_n, ignore_words);

    // Revised approach using tabulars module
    let mut sorted_freqs: Vec<(&String, &usize)> = word_map.iter()
        .filter(|(word, _)| 
            !ignore_words.contains(&word.to_ascii_lowercase().as_str()) // Passes stopwords check
                && args.ignore_words.as_ref().map_or(true, |iwords| !iwords.contains(&word)) // Passes additional ignore words check
        )
        .collect();
    sorted_freqs.sort_by(|a, b| b.1.cmp(a.1)); // Sort in descending order by frequency
    
    // Get the headers and top N rows of data
    let headers = vec!["#", "word", "ct"];
    // Get data from word frequency map and convert to homogenous vec for display
    let data: Vec<Vec<_>> = sorted_freqs[..top_n.min(sorted_freqs.len())].iter().enumerate().map(|(i, (s, n))| vec![(i+1).to_string(), s.to_string(), n.to_string()]).collect(); // Get top N or all if fewer than N
    // Set table styling, current options are ASCII, Polars, Normal etc.
    let style = wclib::TableStyle::Polars;
    // Right align numeric columns and left align words
    let alignment = "><>"; 
    // Create the table using the headers and data along with specified style formatting if top_n was provided and greater than 0:
    let fmt_freq_table = if top_n > 0 {
        wclib::Table::new_with_style(headers, data, style, alignment).to_string()
    } else {
        "".to_string()
    };

    // let fmt_filename = args.filename.to_string_lossy().replace("\\", "/");
    // let fmt_filename = concat_str!(fmt_filename, ": ");

    let fmt_unique = if args.is_include_unique {
        format!("{unique_count} unique, ")
    } else {
        "".to_string()
    };
    let fmt_summary = format!("{} words, {}{} lines", word_count, fmt_unique, line_count);

    println!("{fmt_freq_table}{fmt_summary}");

    // Print the word count
    Ok(())
}
