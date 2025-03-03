use crate::{account::{Account, Kind}, time::get_month_days};

use serde_json;
use std::{fs, io::{Read, Write}, path::Path};
use chrono::{Datelike, Local};

type AnyError = Box<dyn std::error::Error>;

/// 账单结构体，自然单位为一个月份
pub struct Bill {
    pub accounts: Vec<Account>,
    year: i32,
    month: u32,
}

impl Bill {
    pub fn new(year: Option<i32>, month: Option<u32>) -> Self {
        let accounts: Vec<Account> = Vec::new();

        let year = match year {
            Some(year) => year,
            None => Local::now().year(),
        };

        let month = match month {
            Some(month) => month,
            None => Local::now().month(),
        };

        Self{accounts, year, month}
    }
}

impl Bill {
    pub fn push(&mut self, acc: Account) {
        self.accounts.push(acc);
    }

    pub fn extend(&mut self, accs: Vec<Account>) {
        self.accounts.extend(accs);
    }

    pub fn cover(&mut self, accs: Vec<Account>) {
        self.accounts = accs;
    }
}

/// json文件io操作
impl Bill {
    /// 获得文件存储路径
    pub fn get_path(&self, file_path: &Option<String>) -> String {
        match file_path {
            Some(string) => string.clone(),
            None => format!("data/{}_{}.json", self.year, self.month)
        }
    }

    /// 数据存储方法，默认为覆写方法
    pub fn save(&self, file_path: &Option<String>) -> Result<(), AnyError> {
        let file_path = self.get_path(file_path);
        let is_init = !is_exists(&file_path);
        let data: Vec<Account>;

        // 当第一次创建文件时，直接写入，否则读取再并入
        if !is_init {
            // data = load_json(&file_path)?;
            // data.extend(self.accounts.clone());
            data = self.accounts.clone();
        } else {
            data = self.accounts.clone();
        }

        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&file_path)?;

        serde_json::to_writer_pretty(file, &data)?;
        Ok(())
    }

    pub fn load(&mut self, file_path: &Option<String>) -> Result<(), AnyError> {
        let file_path = self.get_path(file_path);

        // 若文件存在，则直接读取，否则直接返回
        if !Path::new(&file_path).exists() {
            Ok(())
        } else {
            self.accounts = load_json(&file_path)?;
            Ok(())
        }
    }
}

/// 输出模块
impl Bill {
    pub fn get_accs(&self) -> Vec<Account> {
        self.accounts.clone()
    }

    /// 获取指定日期的所有账单
    pub fn get_day_accs(&self, day: u32) -> Self {
        let mut res = Vec::new();

        for acc in self.accounts.iter() {
            if acc.date.day == day {
                res.push(acc.clone());
            }
        }

        let mut day_bill = Self::new(Some(self.year), Some(self.month));
        day_bill.cover(res);
        day_bill
    }

    /// 降级为日级账单
    pub fn to_day_bill(self) -> Vec<Self> {
        let days = get_month_days(self.year, self.month);
        let mut res = Vec::with_capacity(days);

        for _ in 0..days {
            res.push(Self::new(Some(self.year), Some(self.month)));
        }

        for acc in self.accounts {
            res[(acc.date.day - 1) as usize].push(acc);
        }

        res
    }
}

/// 统计模块
impl Bill {
    /// 计算账单内所有支出项总和
    pub fn sum_expense(&self) -> f64 {
        let mut res = 0.0;
        for acc in self.accounts.iter() {
            if let Kind::Out(_) = acc.kind {
                res += acc.value;
            }
        }
        res
    }

    /// 计算账单内所有收入项总和
    pub fn sum_in(&self) -> f64 {
        let mut res = 0.0;
        for acc in self.accounts.iter() {
            if let Kind::In(_) = acc.kind {
                res += acc.value;
            }
        }
        res
    }
}

/// ========================================================================================================
/// 函数集
/// ========================================================================================================
/// 判断指定文件路径是否存在
pub fn is_exists(file_path: &str) -> bool {
    // 将路径分割，并将最后的文件名pop出
    let mut paths: Vec<&str> = file_path.split("/").collect();
    let end = paths.pop().unwrap();
    let mut res = true;
    let mut file_path = String::new();

    for path in paths {
        file_path += path;
        if !Path::new(&file_path).exists() {
            fs::create_dir(&file_path).unwrap();
            res = false;
        }
        file_path += "/";
    }

    file_path += end;
    if !Path::new(&file_path).exists() {
        let mut file = fs::OpenOptions::new().create(true).write(true).open(&file_path).unwrap();
        file.write_all("[]".as_bytes()).unwrap();
        res = false;
    }

    res
}

/// 读取json文件并输出Vec<Account>
pub fn load_json(file_path: &str) -> Result<Vec<Account>, AnyError> {
    let init = !is_exists(file_path);
    let res: Vec<Account> = Vec::new();

    // 若为第一次打开，直接返回空数组
    if init {
        Ok(res)
    } else {
        let mut file = fs::File::open(file_path)?;
    
        // 将文件内容输出为string
        let mut str = String::new();
        file.read_to_string(&mut str)?;
    
        // 将string反序列化为Vec<Acc>
        let res: Vec<Account> = serde_json::from_str(&str)?;
        Ok(res)
    }
}

// 存储json文件
pub fn save_json(data: Vec<Account>, file_path: &str) -> Result<(), AnyError> {
    let _ = !is_exists(&file_path);
    let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&file_path)?;

    serde_json::to_writer_pretty(file, &data)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::account::Kind;
    use crate::time::{Date, get_month_days};

    use super::*;
    use rand::{self, Rng};

    #[test]
    fn save() {
        let acc = Account::new(0, 1f64, Kind::In("".to_string()), Date::now(), None);

        let mut bill = Bill::new(None, None);
        bill.push(acc);

        bill.save(&None).expect("?");
    }

    #[test]
    fn load() {
        let mut bill = Bill::new(None, None);
        bill.load(&None).unwrap();
        println!("{:?}", bill.accounts);
    }

    #[test]
    fn generate_months_data() {
        // 随机生成两个月的数据吧
        let mut bills = vec![Bill::new(None, Some(1)), Bill::new(None, Some(2))];
        let mut rng = rand::rng();
        
        for bill in bills.iter_mut() {
            let mut accs = Vec::new();
            for day in 1..=get_month_days(bill.year, bill.month) {
                let value = rng.random_range(-1.0..=1.0);
                accs.push(Account::new(
                    0,
                    value,
                    Kind::In("".to_string()),
                    Date::from(Some(bill.year), Some(bill.month), Some(day as u32)),
                    None
                ))
            }
            bill.extend(accs);
            bill.save(&None).unwrap();
        }
    }
}