# Word Counter

## Overview
Simple word counter CLI tool to count the number of words in a specified file and provide additional statistics like word frequency and unique word count. Provides additional options for case sensitivity, including non-alphabetic characters, and stopword exclusion.

## Usage
```bash
wc <FILENAME> [OPTIONS]
```

### Arguments:
- `FILENAME`: Path to the file to count words in.

### Options:
- `-n, --top <N>`: Display the top N most frequent words.
- `-I, --ignore <WORD>`: Ignore specific word or words for frequency counts.
- `-c, --cased`: Enable case-sensitive word counting for frequency.
- `-a, --include-all`: Include non-ASCII characters as tokens.
- `-s, --include-stopwords`: Include stopwords in the word count.
- `-u, --include-unique`: Display the count of unique words.

## Example:
To simply count the number of words in a given file, in this case the current `README.MD`:

```text
$ wc "./README.MD"
119 words on 20 lines
```

To display the most commonly occurring words or tokens in the file use the `--top <N>` or `-n <N>` option frequencies, and to ignore specific words or tokens when counting them, we can use the `--ignore <WORD>` or `-i <WORD>` option as well:

```text
$ wc "./README.MD" --top 5 --ignore "word,words"
┌───┬─────────┬────┐
│ # ┆ word    ┆ ct │
╞═══╪═════════╪════╡
│ 1 ┆ top     ┆  8 │
│ 2 ┆ count   ┆  7 │
│ 3 ┆ wc      ┆  5 │
│ 4 ┆ display ┆  5 │
│ 5 ┆ cased   ┆  4 │
└───┴─────────┴────┘
176 words on 37 lines
```

The `--include-stopwords` or `-s` option can be provided to toggle the inclusion of common stopwords when counting word frequencies. The full list of stopwords is ~184 words in total but some examples of stopwords are "the", "of" and "a". Such words are not counted by default when calculating word frequencies.

## Note
Created for fun with no real intention of revisiting, thanks!

Respectfully,
Ante