use find_folder::Search;
use find_folder::Error;
use std::path::PathBuf;

pub fn find_assets_folder() -> PathBuf {
   let p = Search::KidsThenParents(3, 3).for_folder("assets");
   match p {
      Result::Ok(path) => { path }
      Result::Err(E) => { panic!("Assets folder not found.") }
   }
}

pub fn find_asset(filename: &str) -> PathBuf {
   find_assets_folder().join(filename)
}