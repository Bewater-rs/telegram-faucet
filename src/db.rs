

#[derive(Debug, Clone)]
pub struct UserFaucetInfo {
    pub times: u32,
    pub amount: u32,
    pub already_issued: bool,
    pub timestamp: u32,
}

fn open_db() {
    let tree = sled::open("").expect("failed to open sled db");
    if let Ok(Some((key, val))) = tree.last() {
        tree.insert(b"", b"");
    } else {
        tree.insert(b"", b"");
    }
}
