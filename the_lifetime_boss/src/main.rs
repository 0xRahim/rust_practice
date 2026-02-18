/*

### 3. A Basic Log Parser (The "Lifetime" Boss)

If you want to truly test your understanding of **Lifetimes and Strings**, this is the one.

* **The Goal:** Read a multi-line string (simulating a log file) and extract "Warnings" and "Errors" into a structured format without copying the data unnecessarily.
* **Concepts to Practice:**
* **Lifetimes:** Create a struct `LogEntry<'a>` that holds a reference (`&'a str`) to a slice of the original log string. This prevents heap allocations and keeps it fast.
* **Slices:** Use string slicing to find keywords like `[ERROR]` or `[INFO]`.
* **Vectors:** Store your `LogEntry` structs in a `Vec`.
* **Iterators:** (Sneak peek for Chapter 13) Use `.lines()` to loop through the input.


*/
#[derive(Debug)]
enum LogType {
    Warning,
    Error,
    Info,
    Debug,
}

#[derive(Debug)]
struct LogEntry<'a> {
    date: &'a str,
    message: &'a str,
    tag: &'a str,
    log_type: LogType,
}

fn main() {
    let mut log_entries: Vec<LogEntry> = Vec::new();

    // read test.log file
    let mut file_data = std::fs::read_to_string("test.log").unwrap();
    file_data.push('\n');

    for lines in file_data.lines() {
        let parts: Vec<&str> = lines.split_whitespace().collect();

        if parts.len() < 4 {
            continue; // skip malformed lines
        }
        let LogEntry {
            date,
            message,
            tag,
            log_type,
        } = LogEntry {
            date: parts[0],
            message: parts[1],
            tag: parts[2],
            log_type: match parts[3] {
                "[WARNING]" => LogType::Warning,
                "[ERROR]" => LogType::Error,
                "[INFO]" => LogType::Info,
                "[DEBUG]" => LogType::Debug,
                _ => LogType::Info,
            },
        };
        log_entries.push(LogEntry {
            date,
            message,
            tag,
            log_type,
        });
    }

    for entry in log_entries {
        println!("{:?}", entry);
    }
}
