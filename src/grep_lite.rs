#[allow(unused, dead_code)]
#[derive(Debug)]
pub struct GrepInfo {
    pub line_number: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub upper_bound: usize,
    pub lower_bound: usize,
}

pub fn grep_lite(haystack: &str, needle: &str) {
    let mut grep_with_ctx = Vec::new();
    let mut temp = Vec::new();

    haystack.lines().for_each(|s| temp.push(s));

    for line in haystack.lines().enumerate() {
        if line.1.contains(needle) {
            let line_number = line.0;
            let mut column_start = 0usize;
            let mut column_end = 0usize;

            if let Some(idx_start) = line.1.find(needle) {
                column_start = idx_start;
                column_end = idx_start + (needle.len() - 1);
            }

            let mut upper_bound = line.0.saturating_add(2);
            let lower_bound = line.0.saturating_sub(2);

            if upper_bound > temp.len() {
                upper_bound = temp.len() - 1;
            }

            grep_with_ctx.push(GrepInfo {
                column_end,
                column_start,
                line_number,
                lower_bound,
                upper_bound,
            });
        }
    }

    if grep_with_ctx.is_empty() {
        println!("===========\n{needle} not found in haystack\n===========");
        return;
    }

    for info in grep_with_ctx {
        let mut precontext = String::new();
        let mut postcontext = String::new();
        let mut context = String::new();
        for line in haystack.lines().enumerate() {
            if
                info.line_number != info.upper_bound &&
                line.0 > info.line_number &&
                line.0 <= info.upper_bound
            {
                postcontext += format!("{}\n", line.1).as_str();
            }
            if
                info.line_number != info.lower_bound &&
                line.0 >= info.lower_bound &&
                line.0 < info.line_number
            {
                precontext += format!("{}\n", line.1).as_str();
            }
            if line.0 == info.line_number {
                context = format!("\tfound here [{}] => {}\n", info.line_number + 1, line.1);
            }
        }

        println!(
            "===========\nFound {needle} \nLine: {}\nColumn: {} - {}\nContext: {}{}{}===========",
            info.line_number + 1,
            info.column_start,
            info.column_end,
            precontext,
            context,
            postcontext
        );
    }
}
