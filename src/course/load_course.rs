use super::*;
use std::fs;
use ron::de::from_str;

use std::path::PathBuf;

fn exe_dir() -> PathBuf {
    let exe_path = std::env::current_exe().expect("failed to get exe path");
    exe_path.parent().unwrap().to_path_buf()
}

fn courses_dir() -> PathBuf {
    exe_dir().join("courses_ron")
}

fn load_course_list() -> CourseList {
    let path = courses_dir().join("index.ron");

    let text = fs::read_to_string(path)
        .expect("failed to read index.ron");

    from_str(&text).expect("failed to parse index.ron")
}

pub fn load_course(path: &str) -> Course {

    let path = courses_dir().join(path);

    let text = fs::read_to_string(path)
        .expect("failed to read course file");

    ron::de::from_str(&text)
        .expect("failed to parse course file")
}

pub fn load_all_courses() -> Vec<(CourseEntry, Course)> {
    let list = load_course_list();

    list.0.into_iter().map(|entry| {
        let course = load_course(&entry.path);
        (entry, course)
    }).collect()
}

pub fn init_courses_list_resource(mut course_list_resource: ResMut<CourseListResource>) {
    let mut course_list = load_all_courses();
    course_list.sort_by(|a,b|a.0.id.cmp(&b.0.id));
    course_list_resource.0 = course_list;
}