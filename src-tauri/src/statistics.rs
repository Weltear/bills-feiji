// 数据统计模块
use serde::{Serialize, Deserialize};

use crate::{account::{Account, Kind}, bill::Bill, config::Config};

/// 统计数据结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct StatData {
    name: String,
    #[serde(rename="type")]
    kind: String,
    #[serde(skip_serializing_if="Option::is_none")]
    stack: Option<String>,
    emphasis: Emphasis,
    data: Vec<f64>,
}

/// 数据统计模块
impl StatData {
    /// 获得Series统计数据
    pub fn get_series(bills: Vec<Bill>) -> Vec<Self> {
        let config = Config::load().unwrap();
        
        // 根据intag创建收入StatData
        let mut in_stat = Vec::new();
        for in_tag in &config.in_tag {
            let stat_data = Self {
                name: in_tag.value.clone(),
                kind: "bar".to_string(),
                stack: Some("收入".to_string()),
                emphasis: Emphasis{focus: "series".to_string()},
                data: vec![0f64; bills.len()],
            };
            in_stat.push(stat_data);
        }

        // 根据outtag创建支出StatData
        let mut out_stat = Vec::new();
        for out_tag in &config.out_tag {
            let stat_data = Self {
                name: out_tag.value.clone(),
                kind: "bar".to_string(),
                stack: Some("支出".to_string()),
                emphasis: Emphasis{focus: "series".to_string()},
                data: vec![0f64; bills.len()],
            };
            out_stat.push(stat_data);
        }

        // 对每单元数据遍历添加，month可为月或日
        for (month, bill) in bills.iter().enumerate() {
            for acc in bill.accounts.iter() {
                Self::acc_deal(acc, &mut in_stat, &mut out_stat, month);
            }
        }

        // 将两者和并返回
        in_stat.extend(out_stat);
        in_stat
    }

    // 账目处理
    pub fn acc_deal(acc: &Account, in_stat: &mut Vec<Self>, out_stat: &mut Vec<Self>, month: usize) {
        let deal = |stats: &mut Vec<Self>, name: &String| {
            for stat in stats.iter_mut() {
                if &stat.name == name {
                    stat.data[month] += acc.value.abs();
                    break;
                }
            }
        };

        match &acc.kind {
            Kind::In(str) => deal(in_stat, str),
            Kind::Out(str) => deal(out_stat, str),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Emphasis {
    focus: String,
}

/// 饼图结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct PieSeries {
    name: String,
    #[serde(rename="type")]
    kind: String,
    radius: String,
    rose_type: String,
    item_style: ItemStyle,
    data: Vec<PieData>
}

impl PieSeries {
    /// 根据StatData处理，目前只需要一个饼图
    pub fn from_bar_series(bar_series: Vec<StatData>, index: usize) -> Vec<Self> {
        let mut data = Vec::new();
        for stat_data in bar_series {
            // 若为支出，纳入统计
            if let Some(string) = stat_data.stack {
                if string == "支出" {
                    let pie_data = PieData {
                        value: stat_data.data[index],
                        name: stat_data.name,
                    };
                    data.push(pie_data);
                }
            }
        }

        let res = Self {
            name: "支出统计饼图".to_string(),
            kind: "pie".to_string(),
            radius: "60%".to_string(),
            rose_type: "area".to_string(),
            item_style: ItemStyle {
                shadow_blur: 10,
                shadow_offset_x: 0,
                shadow_color: "rgba(0, 0, 0, 0.5)".to_string(),
            },
            data,
        };

        vec![res]
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemStyle {
    shadow_blur: i32,
    shadow_offset_x: i32,
    shadow_color: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PieData {
    value: f64,
    name: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn get_json() {
        let data = vec![0f64; 12];
        let stat_data = StatData {
            name: "收入".to_string(),
            kind: "bar".to_string(),
            stack: Some("get大泽".to_string()),
            emphasis: Emphasis{focus: "series".to_string()},
            data
        };
        let str = serde_json::to_string_pretty(&stat_data).unwrap();
        println!("{}", str);
    }
}