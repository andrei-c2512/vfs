

pub fn split_path(path : &str) -> Vec<String>{
    let list : Vec<&str> = path.split('/').collect();
    let mut result = Vec::with_capacity(list.len());
    for view in list.iter() {
        result.push(view.to_string());
    }
    result
}

