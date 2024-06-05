use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::{
    env::args,
    fs::{create_dir_all, write},
    path::PathBuf,
};

fn main() {
    let args: Vec<String> = args().collect();
    let index_name = args.get(2).expect("Output directory not specified!");
    let index_page = get(args.get(1).expect("SLM url not specified!"));
    let index_page = Html::parse_document(&index_page);
    let semesters_selector = Selector::parse("div.row:nth-child(6) > div:nth-child(1) > div:nth-child(2) > div > div > h4 > a:nth-child(1)").unwrap();
    let semesters = index_page.select(&semesters_selector);

    for semester in semesters {
        let semester_name = semester.text().next().unwrap().trim();
        let semester_path = semester.attr("href").unwrap();
        println!("Semester: {semester_name}: {semester_path}");
        let semester_page = get(format!("{}{}", "https://egyankosh.ac.in", semester_path).as_str());
        let semester_page = Html::parse_document(&semester_page);
        let courses_selector = Selector::parse("div.row:nth-child(6) > div:nth-child(1) > div:nth-child(2) > div > div > h4 > a:nth-child(1)").unwrap();
        let courses = semester_page.select(&courses_selector);
        for course in courses {
            let course_name = course.text().next().unwrap().trim();
            let course_path = course.attr("href").unwrap();
            println!("Course: {course_name}: {course_path}");
            let course_page = get(format!("{}{}", "https://egyankosh.ac.in", course_path).as_str());
            let course_page = Html::parse_document(&course_page);
            let blocks_selector = Selector::parse("div.row:nth-child(6) > div:nth-child(1) > div:nth-child(2) > div > div > h4 > a:nth-child(1)").unwrap();
            let blocks = course_page.select(&blocks_selector);
            for block in blocks {
                let block_name = block.text().next().unwrap().trim();
                let block_path = block.attr("href").unwrap();
                println!("Block: {block_name}: {block_path}");
                let block_page =
                    get(format!("{}{}", "https://egyankosh.ac.in", block_path).as_str());
                let block_page = Html::parse_document(&block_page);
                let units_selector = Selector::parse(".table > tbody:nth-child(2) > tr > td:nth-child(2) > strong:nth-child(1) > a:nth-child(1)").unwrap();
                let units = block_page.select(&units_selector);
                for unit in units {
                    let unit_name = unit.text().next().unwrap().trim();
                    let unit_path = unit.attr("href").unwrap();
                    println!("Unit: {unit_name}: {unit_path}");
                    let unit_page =
                        get(format!("{}{}", "https://egyankosh.ac.in", unit_path).as_str());
                    let unit_page = Html::parse_document(&unit_page);
                    let pdf_selector =
                        Selector::parse("td.standard:nth-child(1) > a:nth-child(1)").unwrap();
                    let pdf = unit_page.select(&pdf_selector).next().unwrap();
                    let pdf_path = pdf.attr("href").unwrap();
                    let full_dir_path = PathBuf::new()
                        .join(index_name)
                        .join(semester_name)
                        .join(course_name)
                        .join(block_name);
                    create_dir_all(&full_dir_path).unwrap();
                    let full_file_path = full_dir_path
                        .join(unit_name.replace('/', "-"))
                        .with_extension("pdf");
                    if let Err(_) = std::fs::metadata(&full_file_path) {
                        let pdf_page = Client::builder()
                            .danger_accept_invalid_certs(true)
                            .build()
                            .unwrap()
                            .get(format!("{}{}", "https://egyankosh.ac.in", pdf_path).as_str())
                            .send()
                            .unwrap()
                            .bytes()
                            .unwrap();
                        write(&full_file_path, pdf_page).unwrap();
                    }
                    println!("Saved {}", full_file_path.display());
                }
            }
        }
    }
}

fn get(url: &str) -> String {
    Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get(url)
        .send()
        .unwrap()
        .text()
        .unwrap()
}
