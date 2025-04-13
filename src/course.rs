use std::fmt::Debug;
use std::fs::{create_dir, File};
use std::io::{copy, BufWriter};
use std::{error::Error, path::Path};

use crate::Client;
use reqwest::Url;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Programme {
    name: String,
    path: String,
    semesters: Vec<Semester>,
}
impl Select for Programme {
    const SELECTOR: &str = ".col-md-8 > h2:nth-child(1)";
    fn new(name: String, path: String, client: &Client) -> Result<Self, Box<dyn Error>> {
        let page = client.text(&path)?;
        let page = Html::parse_document(&page);
        Ok(Self {
            name,
            path,
            semesters: Select::select(client, &page)?,
        })
    }
}
impl Create for Programme {
    fn create(&self, path: &Path, client: &Client) -> Result<(), Box<dyn Error>> {
        println!("creating programme: {}", self.name);
        let this_path = path.join(&self.name);
        create_dir(&this_path).ok();
        for semester in &self.semesters {
            semester.create(&this_path, client)?
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Semester {
    name: String,
    path: String,
    courses: Vec<Course>,
}
impl Select for Semester {
    const SELECTOR: &str = "div.row:nth-child(6) > div:nth-child(1) > div:nth-child(2) > div > div > h4 > a:nth-child(1)";
    fn new(name: String, path: String, client: &Client) -> Result<Self, Box<dyn Error>> {
        let page = client.text(format!("https://egyankosh.ac.in{}", &path))?;
        let page = Html::parse_document(&page);
        let mut courses = Course::select(client, &page)?;
        let mut lab_courses = LabCourse::select(client, &page)?
            .into_iter()
            .map(|lc| lc.into())
            .collect();
        courses.append(&mut lab_courses);
        Ok(Self {
            name,
            path,
            courses,
        })
    }
}
impl Create for Semester {
    fn create(&self, path: &Path, client: &Client) -> Result<(), Box<dyn Error>> {
        println!("creating semester: {}", self.name);
        let this_path = path.join(&self.name);
        create_dir(&this_path).ok();
        for course in &self.courses {
            course.create(&this_path, client)?
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Course {
    name: String,
    path: String,
    blocks: Vec<Block>,
}
impl Select for Course {
    const SELECTOR: &str = "div.row:nth-child(6) > div:nth-child(1) > div:nth-child(2) > div > div > h4 > a:nth-child(1)";
    fn new(name: String, path: String, client: &Client) -> Result<Self, Box<dyn Error>> {
        let page = client.text(format!("https://egyankosh.ac.in{}", &path))?;
        let page = Html::parse_document(&page);
        Ok(Self {
            name,
            path,
            blocks: Select::select(client, &page)?,
        })
    }
}
impl From<LabCourse> for Course {
    fn from(lab: LabCourse) -> Self {
        Self {
            name: lab.name,
            path: lab.path,
            blocks: vec![Block {
                name: "Block 1".into(),
                path: "".into(),
                units: lab.units,
            }],
        }
    }
}
impl Create for Course {
    fn create(&self, path: &Path, client: &Client) -> Result<(), Box<dyn Error>> {
        println!("creating course: {}", self.name);
        let this_path = path.join(&self.name);
        create_dir(&this_path).ok();
        for block in &self.blocks {
            block.create(&this_path, client)?
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct LabCourse {
    name: String,
    path: String,
    units: Vec<Unit>,
}
impl Select for LabCourse {
    const SELECTOR: &str = "div.row:nth-child(6) > div:nth-child(2) > div:nth-child(2) > div > div:nth-child(1) > h4:nth-child(1) > a:nth-child(1)";
    fn new(name: String, path: String, client: &Client) -> Result<Self, Box<dyn Error>> {
        let page = client.text(format!("https://egyankosh.ac.in{}", &path))?;
        let page = Html::parse_document(&page);
        Ok(Self {
            name,
            path,
            units: Select::select(client, &page)?,
        })
    }
}

#[derive(Debug)]
pub struct Block {
    name: String,
    path: String,
    units: Vec<Unit>,
}
impl Select for Block {
    const SELECTOR: &str = "div.row:nth-child(6) > div:nth-child(1) > div:nth-child(2) > div > div > h4 > a:nth-child(1)";
    fn new(name: String, path: String, client: &Client) -> Result<Self, Box<dyn Error>> {
        let page = client.text(format!("https://egyankosh.ac.in{}", &path))?;
        let page = Html::parse_document(&page);
        Ok(Self {
            name,
            path,
            units: Select::select(client, &page)?,
        })
    }
}
impl Create for Block {
    fn create(&self, path: &Path, client: &Client) -> Result<(), Box<dyn Error>> {
        println!("creating block: {}", self.name);
        let this_path = path.join(&self.name);
        create_dir(&this_path).ok();
        for unit in &self.units {
            unit.create(&this_path, client)?
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Unit {
    name: String,
    path: String,
    pdf: Pdf,
}
impl Select for Unit {
    const SELECTOR: &str =
        ".table > tbody:nth-child(2) > tr > td:nth-child(2) > strong:nth-child(1) > a:nth-child(1)";

    fn new(name: String, path: String, client: &Client) -> Result<Self, Box<dyn Error>> {
        let page = client.text(format!("https://egyankosh.ac.in{}", &path))?;
        let page = Html::parse_document(&page);
        Ok(Self {
            name,
            path,
            pdf: Select::select(client, &page)?
                .into_iter()
                .next()
                .ok_or("no pdf found")?,
        })
    }
}
impl Create for Unit {
    fn create(&self, path: &Path, client: &Client) -> Result<(), Box<dyn Error>> {
        println!("creating unit: {}", self.name);
        let this_path = path.join(&self.name);
        self.pdf.create(&this_path, client)
    }
}

#[derive(Debug)]
pub struct Pdf {
    path: String,
}
impl Select for Pdf {
    const SELECTOR: &str = "td.standard:nth-child(1) > a:nth-child(1)";

    fn new(_name: String, path: String, _client: &Client) -> Result<Self, Box<dyn Error>> {
        Ok(Self { path })
    }
}
impl Create for Pdf {
    fn create(&self, path: &Path, client: &Client) -> Result<(), Box<dyn Error>> {
        println!("creating pdf");
        let url = Url::parse(&format!("https://egyankosh.ac.in{}", &self.path))?;
        let mut pdf = client.bytes(&url)?;
        let mut file = BufWriter::new(File::create(path.with_extension("pdf"))?);
        copy(&mut pdf, &mut file)?;
        Ok(())
    }
}

pub trait Select: Sized + Debug {
    const SELECTOR: &str;
    fn new(name: String, path: String, client: &Client) -> Result<Self, Box<dyn Error>>;

    fn select(client: &Client, page: &Html) -> Result<Vec<Self>, Box<dyn Error>> {
        let selector = Selector::parse(Self::SELECTOR)?;
        let selected = page.select(&selector);
        let mut selections = vec![];
        for selection in selected {
            let name = selection
                .text()
                .next()
                .ok_or("name not found")?
                .trim()
                .replace('/', "|");
            let path = selection.attr("href").ok_or("path not found")?;
            println!("name: {name}; path: {path}");
            let item = Self::new(name.into(), path.into(), client)?;
            selections.push(item)
        }
        Ok(selections)
    }
}

pub trait Create {
    fn create(&self, path: &Path, client: &Client) -> Result<(), Box<dyn Error>>;
}
