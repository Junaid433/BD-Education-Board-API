use crate::models::{Grade, ResultData};
use scraper::{Html, Selector};
use std::sync::OnceLock;

fn table_selector() -> &'static Selector {
    static TABLE_SELECTOR: OnceLock<Selector> = OnceLock::new();
    TABLE_SELECTOR.get_or_init(|| Selector::parse("table.black12").expect("valid table selector"))
}

fn td_selector() -> &'static Selector {
    static TD_SELECTOR: OnceLock<Selector> = OnceLock::new();
    TD_SELECTOR.get_or_init(|| Selector::parse("td").expect("valid td selector"))
}

fn tr_selector() -> &'static Selector {
    static TR_SELECTOR: OnceLock<Selector> = OnceLock::new();
    TR_SELECTOR.get_or_init(|| Selector::parse("tr").expect("valid tr selector"))
}

fn cell_text(cell: scraper::ElementRef<'_>) -> String {
    cell.text().map(str::trim).collect::<String>()
}

fn normalize_label(label: &str) -> String {
    label
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .to_ascii_lowercase()
}

fn set_if_present(slot: &mut Option<String>, value: String) {
    if !value.is_empty() {
        *slot = Some(value);
    }
}

pub fn parse_result(html: &str) -> crate::exceptions::AppResult<ResultData> {
    let document = Html::parse_document(html);

    let mut result = ResultData::default();

    if let Some(info_table) = document.select(table_selector()).next() {
        let mut info_iter = info_table.select(td_selector());
        while let (Some(key_td), Some(value_td)) = (info_iter.next(), info_iter.next()) {
            let key = cell_text(key_td);
            let value = cell_text(value_td);

            match normalize_label(&key).as_str() {
                "roll no" | "roll" => set_if_present(&mut result.roll, value),
                "registration no" | "reg no" | "registration" | "reg" => {
                    set_if_present(&mut result.reg, value)
                }
                "name" => set_if_present(&mut result.name, value),
                "board" => set_if_present(&mut result.board, value),
                "fathers name" => set_if_present(&mut result.father_name, value),
                "mothers name" => set_if_present(&mut result.mother_name, value),
                "group" => set_if_present(&mut result.group, value),
                "type" | "exam type" => set_if_present(&mut result.exam_type, value),
                "date of birth" | "dob" => set_if_present(&mut result.dob, value),
                "result" => set_if_present(&mut result.result, value),
                "institute" | "institution" => set_if_present(&mut result.institute, value),
                "gpa" => set_if_present(&mut result.gpa, value),
                _ => {}
            }
        }
    }

    let mut tables = document.select(table_selector());
    tables.next();
    if let Some(grade_table) = tables.next() {
        for row in grade_table.select(tr_selector()).skip(1) {
            let mut tds = row.select(td_selector());
            if let (Some(code_td), Some(subject_td), Some(grade_td), None) =
                (tds.next(), tds.next(), tds.next(), tds.next())
            {
                let code = cell_text(code_td);
                let subject = cell_text(subject_td);
                let grade = cell_text(grade_td);
                result.grades.push(Grade {
                    code,
                    subject,
                    grade,
                });
            }
        }
    }

    if !result.has_identity() {
        return Err(crate::exceptions::AppError::Parse(
            "Result page did not contain a valid student record".to_string(),
        ));
    }

    Ok(result)
}
