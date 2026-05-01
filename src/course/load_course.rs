use super::*;
use ron::de::from_str;
use std::fs;

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

    let text = fs::read_to_string(path).expect("failed to read index.ron");

    from_str(&text).expect("failed to parse index.ron")
}

pub fn load_course(path: &str) -> Course {
    let path = courses_dir().join(path);

    let text = fs::read_to_string(path).expect("failed to read course file");

    ron::de::from_str(&text).expect("failed to parse course file")
}

pub fn load_all_courses() -> Vec<(CourseEntry, Course)> {
    let list = load_course_list();

    list.0
        .into_iter()
        .map(|entry| {
            let course = load_course(&entry.path);
            (entry, course)
        })
        .collect()
}

pub fn init_courses_list_resource(mut course_list_resource: ResMut<CourseListResource>) {
    let mut course_list = load_all_courses();
    course_list.sort_by(|a, b| a.0.id.cmp(&b.0.id));
    course_list_resource.0 = course_list;
}

pub mod asset_load {
    use super::*;
    use bevy::asset::Asset;
    use bevy::reflect::TypePath;

    #[derive(Asset, TypePath, Clone)]
    pub struct RonText(pub String);
    use anyhow::Error;
    use bevy::asset::{AssetLoader, LoadContext, io::Reader};

    #[derive(Default, TypePath)]
    pub struct RonTextLoader;

    impl AssetLoader for RonTextLoader {
        type Asset = RonText;
        type Settings = ();
        type Error = Error;

        async fn load(
            &self,
            reader: &mut dyn Reader,
            _settings: &Self::Settings,
            _ctx: &mut LoadContext<'_>,
        ) -> Result<Self::Asset, Self::Error> {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let text = String::from_utf8(bytes)?;
            Ok(RonText(text))
        }

        fn extensions(&self) -> &[&str] {
            &["ron"]
        }
    }

    #[derive(Resource, Default)]
    pub struct CourseLoadState {
        pub index: Option<Handle<RonText>>,
        pub courses: Vec<(CourseEntry, Handle<RonText>)>,
        pub loaded: bool,
    }
    pub fn start_load_courses(mut state: ResMut<CourseLoadState>, asset_server: Res<AssetServer>) {
        state.index = Some(asset_server.load("courses_ron/index.ron"));
        state.courses.clear();
        state.loaded = false;
    }
    pub fn resolve_courses(
        mut state: ResMut<CourseLoadState>,
        mut course_list_resource: ResMut<CourseListResource>,
        texts: Res<Assets<RonText>>,
        asset_server: Res<AssetServer>,
    ) {
        if state.loaded {
            return;
        }
        let Some(index_handle) = &state.index else {
            return;
        };

        // indexがまだロードされてなければ待つ
        let Some(index_text) = texts.get(index_handle) else {
            return;
        };

        let list: CourseList = ron::de::from_str(&index_text.0).expect("parse index.ron failed");

        // courseのロード開始（1回だけ）
        if state.courses.is_empty() {
            state.courses = list
                .0
                .iter()
                .map(|entry| {
                    let handle = asset_server.load(format!("courses_ron/{}", entry.path));
                    (entry.clone(), handle)
                })
                .collect::<Vec<(CourseEntry, Handle<RonText>)>>();
        }

        // 全部ロード完了してるかチェック
        let mut result = vec![];

        for (entry, handle) in &state.courses {
            let Some(text) = texts.get(handle) else {
                return;
            };

            let course: Course = ron::de::from_str(&text.0).expect("parse course failed");

            result.push((entry.clone(), course));
        }

        // ソートして反映
        result.sort_by(|a, b| a.0.id.cmp(&b.0.id));
        course_list_resource.0 = result;
        state.loaded = true;
    }
}
