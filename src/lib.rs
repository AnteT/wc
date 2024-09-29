use std::fmt::Display;

/// Returns the number of digits in the provided value using a more performant log based approach.
pub fn _count_digits_log(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    // Calculate the number of digits
    ((n as f64).log(10.0).floor() as usize) + 1
}

/// Pad a given input (either number or string) to fit within a specified width.
pub fn pad_input<T: Display>(input: T, max_length: usize, align_right: bool) -> String {
    let input = input.to_string();
    let padding = " ".repeat(max_length.saturating_sub(input.len()));
    
    if align_right {
        concat_str!(padding, input)
    } else {
        concat_str!(input, padding)
    }
}

/// Experimental version using the mapping iterable directly to do all the heavy lifting of creating and returning a tabular representation of the top N frequency mappings.
pub fn generate_frequency_table(word_map: &std::collections::HashMap<String, usize>, top_n: usize, ignore_words: &[&str]) -> String {
    if top_n < 1 {
        return String::from("")
    }
    const WID_LABEL: &'static str = "#";
    const WORD_LABEL: &'static str = "word";
    const FREQ_LABEL: &'static str = "ct";
    const VERT_CHAR: &'static str = "│";
    const TOP_LH_CORNER: &'static str = "┌";
    const TOP_VERT_SEP: &'static str = "┬";
    const TOP_RH_CORNER: &'static str = "┐";
    const MID_LH_BORDER: &'static str = "├";
    const MID_VERT_SEP: &'static str = "┼";
    const MID_RH_BORDER: &'static str = "┤";
    const BOTTOM_LH_CORNER: &'static str = "└";
    const BOTTOM_VERT_SEP: &'static str = "┴";
    const BOTTOM_RH_CORNER: &'static str = "┘";
    const HORIZ_CHAR: &'static str = "─";
    const NEWLINE_CHAR: &'static str = "\n";
    const PAD_CHAR: &'static str = " ";
    const PAD_WIDTH: usize = 1;

    let mut table_repr = String::new();

    // Collects sorted word frequency mappings after filtering out stop words, if any, provided through `ignore_words`
    let mut sorted_freqs: Vec<(&String, &usize)> = word_map.iter().filter(|(word, _)| !ignore_words.contains(&word.to_ascii_lowercase().as_str())).collect();
    sorted_freqs.sort_by(|a, b| b.1.cmp(a.1)); // Sort in descending order by frequency

    // Get the top N
    let top_words: &[(&String, &usize)] = &sorted_freqs[..top_n.min(sorted_freqs.len())]; // Get top N or all if fewer than N
    
    // Find the maximum length of the words and the width of top N for padding result set
    let max_wid_length = WID_LABEL.len().max(top_n.min(top_words.len()).to_string().len());
    let max_freq_length = FREQ_LABEL.len().max(sorted_freqs.first().map_or_else(|| 0_usize, |(_, &v)| v.to_string().len()));
    let max_word_length = WORD_LABEL.len().max(top_words.iter().map(|(word, _)| word.len()).max().unwrap_or(0)); // Returns 0 if there are no words
    
    let top_lh_corner = concat_str!(TOP_LH_CORNER, HORIZ_CHAR.repeat(PAD_WIDTH));
    let lh_border = concat_str!(VERT_CHAR, PAD_CHAR.repeat(PAD_WIDTH));
    let top_sep = concat_str!(HORIZ_CHAR.repeat(PAD_WIDTH), TOP_VERT_SEP, HORIZ_CHAR.repeat(PAD_WIDTH));
    let mid_lh_border = concat_str!(MID_LH_BORDER, HORIZ_CHAR.repeat(PAD_WIDTH));
    let mid_vert_sep = concat_str!(HORIZ_CHAR.repeat(PAD_WIDTH), MID_VERT_SEP, HORIZ_CHAR.repeat(PAD_WIDTH));
    let mid_rh_border = concat_str!(HORIZ_CHAR.repeat(PAD_WIDTH), MID_RH_BORDER);
    let vertical_sep = concat_str!(PAD_CHAR.repeat(PAD_WIDTH), VERT_CHAR, PAD_CHAR.repeat(PAD_WIDTH));
    let top_rh_corner = concat_str!(HORIZ_CHAR.repeat(PAD_WIDTH), TOP_RH_CORNER);
    let rh_border = concat_str!(PAD_CHAR.repeat(PAD_WIDTH), VERT_CHAR);
    let bottom_lh_corner = concat_str!(BOTTOM_LH_CORNER, HORIZ_CHAR.repeat(PAD_WIDTH));
    let bottom_rh_corner = concat_str!(HORIZ_CHAR.repeat(PAD_WIDTH), BOTTOM_RH_CORNER);
    let bottom_vert_sep = concat_str!(HORIZ_CHAR.repeat(PAD_WIDTH), BOTTOM_VERT_SEP, HORIZ_CHAR.repeat(PAD_WIDTH));

    let mut wid = 0_usize;
    // Construct top border of the table
    let top = concat_str!(top_lh_corner, HORIZ_CHAR.repeat(max_wid_length), top_sep, HORIZ_CHAR.repeat(max_word_length), top_sep, HORIZ_CHAR.repeat(max_freq_length), top_rh_corner, NEWLINE_CHAR);
    table_repr.push_str(&top);
    
    // Construct table header
    let fmt_wid_label = self::pad_input(WID_LABEL, max_wid_length, true);
    let fmt_word_label = self::pad_input(WORD_LABEL, max_word_length, false);
    let fmt_freq_label = self::pad_input(FREQ_LABEL, max_freq_length, true);
    let header = concat_str!(lh_border, fmt_wid_label, vertical_sep, fmt_word_label, vertical_sep, fmt_freq_label, rh_border, NEWLINE_CHAR);
    table_repr.push_str(&header);
    
    // Construct the header body separator
    let mid = concat_str!(mid_lh_border, HORIZ_CHAR.repeat(max_wid_length), mid_vert_sep, HORIZ_CHAR.repeat(max_word_length), mid_vert_sep, HORIZ_CHAR.repeat(max_freq_length), mid_rh_border, NEWLINE_CHAR);
    table_repr.push_str(&mid);

    // Construct the table rows
    for (word, freq) in top_words {
        wid += 1;
        let fmt_wid = self::pad_input(wid, max_wid_length, true);
        let fmt_word = self::pad_input(word, max_word_length, false);
        let fmt_freq = self::pad_input(freq, max_freq_length, true);
        let fmt_row = concat_str!(lh_border, fmt_wid, vertical_sep, fmt_word, vertical_sep, fmt_freq, rh_border, NEWLINE_CHAR);
        table_repr.push_str(&fmt_row);
    }

    // Construct the bottom border of the table
    let bottom = concat_str!(bottom_lh_corner, HORIZ_CHAR.repeat(max_wid_length), bottom_vert_sep, HORIZ_CHAR.repeat(max_word_length), bottom_vert_sep, HORIZ_CHAR.repeat(max_freq_length), bottom_rh_corner, NEWLINE_CHAR);
    table_repr.push_str(&bottom);

    table_repr
}

#[macro_export]
/// Concatenates provided strings using push_str method to avoid overhead of format macro with explicit capacity bounds.
macro_rules! concat_str {
    ($($item:expr),*) => {{
        let total_length = 0 $( + $item.len() )*;
        let mut result = String::with_capacity(total_length);
        $( result.push_str($item.as_ref()); )*
        result
    }};
}

#[derive(Debug, Clone)]
/// Mentally treat this as a Textual Table instead of a DataFrame, maybe that'll help with constructing the structure.
pub struct Table<T, U> {
    pub headers: Vec<T>,
    pub data: Vec<Vec<U>>,
    pub style: TableStyle,
    pub alignment: Option<String>
}

/* DataFrame methods go here as they dont require that T and U implement Display or String related methods */
impl<T, U> Table<T, U> {
    /// Create a new `Table` using the default styling for now
    pub fn new(headers: Vec<T>, data: Vec<Vec<U>>) -> Self {
        Table { headers, data, style: TableStyle::Normal, alignment: None }
    }
    /// Create a new `Table` with a specified formatting style and column alignment pattern.
    pub fn new_with_style(headers: Vec<T>, data: Vec<Vec<U>>, style: TableStyle, alignment: impl Into<String>) -> Self {
        let alignment: String = alignment.into();
        let alignment = if !alignment.is_empty() { Some (alignment) } else { None };
        Table { headers, data, style, alignment }
    }
    /// Get shape from data dimensions as a tuple of `(rows, columns)`
    pub fn get_shape(&self) -> (usize, usize) {
        (self.data.len(), self.headers.len())
    }
    /// Change the table style using one of the preset `TableStyle` variants like `ASCII` or `Polars` styles
    pub fn set_table_style(&mut self, style: TableStyle) {
        self.style = style;
    }
    /// Set the table column alignment using a String of sequential `<` or `>` characters to represent left or right column alignment, respectively.
    /// For example, providing a value of `"><>>"` would set the first column to right-aligned, the second to left aligned and the third and fourth columns to right aligned.
    /// To reset alignment to default, which is to left-align all columns, provide an empty string or invalid character count.
    pub fn set_table_alignment(&mut self, alignment: impl Into<String>) {
        let alignment: String = alignment.into();
        let alignment = if !alignment.is_empty() { Some (alignment) } else { None };        
        self.alignment = alignment;
    }    
    /// Create a tabular data grid of specified dimensions.
    pub fn generate_grid(n_rows: usize, n_cols: usize) -> Table<String, String> {
        let headers = (0..n_cols).map(|c| c.to_string()).collect::<Vec<String>>();
        let data = (0..n_rows)
            .map(|r| {
                (0..n_cols)
                    .map(|c| concat_str!(r.to_string(), ",", c.to_string()))
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();
        Table::new(headers, data)
    }
}

/* Textual printing related methods go here as T and U are bound by the `Display` trait and their lengths can be determined ahead of time */
impl<T, U> Table<T, U> where T: Display, U: Display {
    /// Returns a table of type Table<String, String> to allow terminal pretty printing.
    pub fn to_stringy(self) -> Table<String, String> {
        let headers: Vec<String> = self.headers.into_iter().map(|h| h.to_string()).collect();
        let data: Vec<Vec<String>> = self.data.into_iter().map(|row| row.into_iter().map(|item| item.to_string()).collect()).collect();
        Table { headers, data, style: self.style, alignment: self.alignment }
    }
    /// Converts the header field into a 1D array of Strings to prep for rendering.
    pub fn get_headers_to_string(&self) -> Vec<String> {
        self.headers.iter().map(|h| h.to_string()).collect()
    }
    /// Converts the data field into a 2D array of Strings to prep for rendering.
    pub fn get_data_to_string(&self) -> Vec<Vec<String>> {
        let data: Vec<Vec<String>> = self.data.iter()
        .map(|row| row.iter().map(|item| item.to_string()).collect())
        .collect();
        data
    }
}

impl<T, U> Display for Table<T, U> where T: Display, U: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Get the display format characters
        let fmt = self.style.get_style_format();

        // Convert headers from T to String:
        let headers = self.get_headers_to_string();

        // Convert data from U to String:
        let data = self.get_data_to_string();
        
        // Get shape of data
        let (_n_rows, n_cols) = self.get_shape();
        
        // Get alignment or use default left-aligned columns if None or invalid input
        let alignment = self.alignment.as_ref().map_or_else(|| vec![false; n_cols], |s| {
            if s.chars().count() == n_cols { s.chars().map(|c| c == '>').collect() } else { vec![false; n_cols] }
        });

        // Initialize the max column widths array using the header widths as a starting point
        let mut max_widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
        
        for cid in 0..n_cols {
            // Calculate max column widths by iterating over the column dimension for each row and keeping largest value
            max_widths[cid] = max_widths[cid].max(data.iter().filter_map(|row| row.get(cid).map(|item| item.len())).max().unwrap_or(cid));
        }
        
        // Big things have small beginnings... 
        let mut fmt_table = String::new();

        // Create the top horizontal bar
        let fmt_top_bar: Vec<String> = max_widths.iter().map(|w| fmt.top_bar.repeat(*w)).collect();
        let fmt_top_bar = concat_str!(fmt.top_lh, fmt_top_bar.join(&fmt.top_sep), fmt.top_rh, fmt.crlf);
        fmt_table.push_str(&fmt_top_bar);
        
        // Create the header row
        let headers_padded: Vec<String> = headers.iter().enumerate().map(|(hid, header)| pad_or_truncate_input(header, max_widths[hid], alignment[hid])).collect();
        let fmt_headers = concat_str!(fmt.row_lh, headers_padded.join(&fmt.row_sep), fmt.row_rh, fmt.crlf);
        fmt_table.push_str(&fmt_headers);

        // Create the middle horizontal separator bar
        let fmt_mid_bar: Vec<String> = max_widths.iter().map(|w| fmt.mid_bar.repeat(*w)).collect();
        let fmt_mid_bar = concat_str!(fmt.mid_lh, fmt_mid_bar.join(&fmt.mid_sep), fmt.mid_rh, fmt.crlf);
        fmt_table.push_str(&fmt_mid_bar);

        // Create the body of the table from the rows of data
        for row in data.into_iter() {
            let row_padded: Vec<String> = row.iter().enumerate().map(|(cid, cell)| pad_or_truncate_input(cell, max_widths[cid], alignment[cid])).collect();
            let fmt_row = concat_str!(fmt.row_lh, row_padded.join(&fmt.row_sep), fmt.row_rh, fmt.crlf);
            fmt_table.push_str(&fmt_row);
        }

        // Create the bottom horizontal bar
        let fmt_end_bar: Vec<String> = max_widths.iter().map(|w| fmt.end_bar.repeat(*w)).collect();
        let fmt_end_bar = concat_str!(fmt.end_lh, fmt_end_bar.join(&fmt.end_sep), fmt.end_rh, fmt.crlf);
        fmt_table.push_str(&fmt_end_bar);        

        // Create the caption
        // let fmt_caption = concat_str!("[", _n_rows.to_string(), " rows, ", n_cols.to_string(), " columns]");
        // fmt_table.push_str(&fmt_caption);
        
        // Write the finished table to the buffer
        write!(f, "{fmt_table}")?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
/// Defines styling for `Table` struct.
pub enum TableStyle {
    Normal,
    ASCII,
    Polars
}

#[derive(Debug, Clone)]
/// Holds the actual formatting strings based on the `TableStyle` chosen.
pub struct TableStyleFormat {
    pub top_lh: &'static str,
    pub top_bar: &'static str,
    pub top_sep: &'static str,
    pub top_rh: &'static str,
    pub mid_lh: &'static str,
    pub mid_bar: &'static str,
    pub mid_sep: &'static str,
    pub mid_rh: &'static str,
    pub row_lh: &'static str,
    pub row_sep: &'static str,
    pub row_rh: &'static str,
    pub end_lh: &'static str,
    pub end_bar: &'static str,
    pub end_sep: &'static str,
    pub end_rh: &'static str,
    pub crlf: &'static str,
}

impl TableStyleFormat {
    /// Create a new format based on positions of formatting characters.
    pub fn new(top_lh: &'static str,top_bar: &'static str,top_sep: &'static str,top_rh: &'static str,mid_lh: &'static str,mid_bar: &'static str,mid_sep: &'static str,mid_rh: &'static str,row_lh: &'static str,row_sep: &'static str,row_rh: &'static str,end_lh: &'static str,end_bar: &'static str,end_sep: &'static str,end_rh: &'static str) -> Self {
        TableStyleFormat {top_lh,top_bar,top_sep,top_rh,mid_lh,mid_bar,mid_sep,mid_rh,row_lh,row_sep,row_rh,end_lh,end_bar,end_sep,end_rh,crlf:"\n"} 
    }
}
impl TableStyle {
    /// Returns the styling components required based on variant.
    pub fn get_style_format(&self) -> TableStyleFormat {
        match self {
            Self::Normal => TableStyleFormat::new(
            "┌─","─","─┬─","─┐",
            "├─","─","─┼─","─┤",
            "│ ",            " │ "," │",
            "└─","─","─┴─","─┘",
            ),
            Self::ASCII => TableStyleFormat::new(
            "+-","-","-+-","-+", 
            "+-","-","-+-","-+", 
            "| ",            " | "," |", 
            "+-","-","-+-","-+",
            ),
            Self::Polars => TableStyleFormat::new(
            "┌─","─","─┬─","─┐",
            "╞═","═","═╪═","═╡",
            "│ ",            " ┆ "," │",
            "└─","─","─┴─","─┘",
            ),
        }
    }
}

/// Pad a given input (either number or string) to fit within a specified width.
pub fn pad_or_truncate_input<T: Display>(input: T, max_length: usize, align_right: bool) -> String {
    let input = input.to_string();
    let input_length = input.len();
    if input_length > max_length {
        concat_str!(input[..max_length.saturating_sub(2)], "..")
    } else {
        let padding = " ".repeat(max_length.saturating_sub(input.len()));
        
        if align_right {
            concat_str!(padding, input)
        } else {
            concat_str!(input, padding)
        }
    }
}