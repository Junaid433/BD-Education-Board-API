use crate::models::{Grade, ResultData};
use scraper::{Html, Selector};

pub fn parse_result(html: &str) -> ResultData {
    let document = Html::parse_document(html);
    let table_selector = Selector::parse("table.black12").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();

    let mut result = ResultData::default();

    if let Some(info_table) = document.select(&table_selector).next() {
        let mut info_iter = info_table.select(&td_selector);
        while let (Some(key_td), Some(value_td)) = (info_iter.next(), info_iter.next()) {
            let key = key_td.text().collect::<String>().trim().to_string();
            let value = value_td.text().collect::<String>().trim().to_string();

            match key.to_lowercase().as_str() {
                "roll no" => result.roll = value,
                "name" => result.name = value,
                "board" => result.board = value,
                "father's name" => result.father_name = value,
                "mother's name" => result.mother_name = value,
                "group" => result.group = value,
                "type" => result.exam_type = value,
                "date of birth" => result.dob = value,
                "result" => result.result = value,
                "institute" => result.institute = value,
                "gpa" => result.gpa = value,
                _ => {}
            }
        }
    }

    let mut tables = document.select(&table_selector);
    tables.next();
    if let Some(grade_table) = tables.next() {
        for row in grade_table.select(&tr_selector).skip(1) {
            let tds: Vec<_> = row.select(&td_selector).collect();
            if tds.len() == 3 {
                let code = tds[0].text().collect::<String>().trim().to_string();
                let subject = tds[1].text().collect::<String>().trim().to_string();
                let grade = tds[2].text().collect::<String>().trim().to_string();
                result.grades.push(Grade {
                    code,
                    subject,
                    grade,
                });
            }
        }
    }

    result
}
