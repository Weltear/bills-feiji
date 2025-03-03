use serde::{Serialize, Deserialize};

use chrono::{Datelike, Local};

/// 日期结构体
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Date {
    pub fn now() -> Self {
        let now = Local::now();
        Self {
            year: now.year(),
            month: now.month(),
            day: now.day(),
        }
    }

    pub fn from(year: Option<i32>, month: Option<u32>, day: Option<u32>) -> Self {
        let now = Local::now();

        let year = match year {
            Some(year) => year,
            None => now.year(),
        };

        let month = match month {
            Some(month) => month,
            None => now.month(),
        };

        let day = match day {
            Some(day) => day,
            None => now.day(),
        };

        Self{year, month, day}
    }

    pub fn from_str(date: &str) -> Result<Self, String> {
        let parts: Vec<&str> = date.split("-").collect();

        let year = parts[0].parse::<i32>().map_err(|_| "年份解析失败".to_string())?;
        let month = parts[1].parse::<u32>().map_err(|_| "月份解析失败".to_string())?;
        let day = parts[2].parse::<u32>().map_err(|_| "日期解析失败".to_string())?;

        let res = Self{year, month, day};
        Ok(res)
    }
}

/// 日期加减操作模块
impl Date {
    pub fn month_addsub(&mut self, n: i32) {
        // 首先，年份增减整除值
        self.year += n / 12;
        // 通过取余值获得月份增减
        let month_change = n % 12;
        // 将月份变为i32，计算差
        let res = self.month as i32 + month_change;
        // 若差值小于等于0，年份减一
        if res <= 0 {
            self.year -= 1;
            self.month = (12 + res) as u32;
        } else if res > 12 {  // 若差值大于12，年份增益
            self.year += 1;
            self.month = (res - 12) as u32;
        } else {  // 否则，月份就是res
            self.month = res as u32;
        }
    }

    /// 日期步进
    pub fn day_add(&mut self) {
        let this_month_max_day = get_month_days(self.year, self.month) as u32;

        // 若为最后一天
        if self.day == this_month_max_day {
            // 月份加一
            self.month_addsub(1);
            // 日期置1
            self.day = 1;
        } else {
            self.day += 1;
        }
    }

    /// 日期步减
    pub fn day_sub(&mut self) {
        if self.day == 1 {
            self.month_addsub(-1);
            self.day = get_month_days(self.year, self.month) as u32;
        } else {
            self.day -= 1;
        }
    }
}

impl Date {
    pub fn to_string(&self) -> String {
        if self.month < 10 && self.day < 10 {
            format!("{}-0{}-0{}", self.year, self.month, self.day)
        } else if self.month < 10 {
            format!("{}-0{}-{}", self.year, self.month, self.day)
        } else if self.day < 10 {
            format!("{}-{}-0{}", self.year, self.month, self.day)
        } else {
            format!("{}-{}-{}", self.year, self.month, self.day)
        }
    }

    pub fn to_month_picker(&self) -> String {
        let month = match self.month {
            1 => "Jan".to_string(),
            2 => "Feb".to_string(),
            3 => "Mar".to_string(),
            4 => "Apr".to_string(),
            5 => "May".to_string(),
            6 => "Jun".to_string(),
            7 => "Jul".to_string(),
            8 => "Aug".to_string(),
            9 => "Sep".to_string(),
            10 => "Oct".to_string(),
            11 => "Nov".to_string(),
            _ => "Dec".to_string(),
        };
        format!("{} {}", month, self.year)
    }

    pub fn sum_days(&self) -> usize {
        get_month_days(self.year, self.month)
    }
}

/// ========================
/// 函数集
/// ========================
/// 当前是否为闰年
pub fn is_leap_year(year: i32) -> bool {
    // 判断是否为世纪年
    if year % 100 == 0 {
        year / 100 % 4 == 0
    } else {
        year % 4 == 0
    }
}

/// 指定月份含多少天
pub fn get_month_days(year: i32, month: u32) -> usize {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        _ => 28,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn debug() {
        let mut date = Date::now();
        for _ in 0..24    {
            date.month_addsub(1);
            println!("{:?}", date);
        }
    }
}