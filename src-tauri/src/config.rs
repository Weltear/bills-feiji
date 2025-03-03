use std::{error::Error, fs, path::Path};

// 设置模块
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub expect_out: f64,
    pub kind: Vec<InTag>,
    pub in_tag: Vec<InTag>,
    pub out_tag: Vec<OutTag>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            expect_out: 0f64,
            kind: Vec::new(),
            in_tag: Vec::new(),
            out_tag: Vec::new(),
        }
    }
}

/// IO 模块
impl Config {
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let file_path = "config.json";
        let file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)?;

        // 写入config
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let file_path = "config.json";
        let res: Self;

        // 若设置文件存在，直接读取
        if Path::new(file_path).exists() {
            let file = fs::File::open(file_path)?;
            res = serde_json::from_reader(file)?;
        } else {
            // 否则，创建一个默认设置并保存
            let init_config = Self {
                expect_out: 0f64,
                kind: vec![
                    InTag{text: "收入".to_string(), value: "收入".to_string()},
                    InTag{text: "支出".to_string(), value: "支出".to_string()}
                ],
                in_tag: vec![
                    InTag{text: "未注明".to_string(), value: "收入：未注明".to_string()},
                    InTag{text: "工资".to_string(), value: "收入：工资".to_string()}
                ],
                out_tag: vec![
                    OutTag{text: "未注明".to_string(), value: "支出：未注明".to_string()},
                    OutTag{text: "餐费".to_string(), value: "支出：餐费".to_string()}
                ],
            };
            init_config.save()?;
            res = init_config;
        }

        Ok(res)
    }
}

/// 收入tag结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct InTag {
    pub text: String,
    pub value: String,
}

// 支出tag结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct OutTag {
    pub text: String,
    pub value: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_config() {
        let config = Config::load().unwrap();

        println!("{}", serde_json::to_string_pretty(&config).unwrap());

        config.save().unwrap();
    }
}