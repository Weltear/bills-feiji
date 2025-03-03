use crate::bill::{load_json, save_json, Bill};
use crate::account::Account;
use crate::config::Config;
use crate::statistics::{PieSeries, StatData};
use crate::time::Date;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// 获取指定日期的数据，mode012分别为本日本月本年
#[tauri::command]
pub fn get_appointed_data(data_ptr: i32) -> (String, String) {
    // 创建日期实例，并通过数据指针获得指定月份
    let mut date = Date::now();
    date.month_addsub(data_ptr);

    // 读取相应数据
    let mut bill = Bill::new(Some(date.year), Some(date.month));
    bill.load(&None).unwrap();

    // 输出
    let result = bill.get_accs();
    let data = serde_json::to_string_pretty(&result).unwrap();
    (data, date.to_month_picker())
}

/// 保存指定的数据，前端返回json数组对象字符串与月份数据指针
#[tauri::command]
pub fn save_appointed_data(data: String, data_ptr: i32) {
    // 创建日期实例并调整至数据日期
    let mut date = Date::now();
    date.month_addsub(data_ptr);

    // 创建账单实例，并将data读入
    let accs: Vec<Account> = serde_json::from_str(&data).unwrap();
    let mut bill = Bill::new(Some(date.year), Some(date.month));
    bill.cover(accs);
    bill.save(&None).unwrap();
}

/// 概览界面数据发送函数
#[tauri::command]
pub fn get_hello_data() -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    // 获取当前日期
    let now = Date::now();
    // 上月日期
    let mut last_month = Date::now();
    last_month.month_addsub(-1);
    // 昨日日期
    let mut last_day = Date::now();
    last_day.day_sub();

    // 本月账单
    let mut this_month_bill = Bill::new(Some(now.year), Some(now.month));
    this_month_bill.load(&None).unwrap();
    // 上月账单
    let mut last_month_bill = Bill::new(Some(last_month.year), Some(last_month.month));
    last_month_bill.load(&None).unwrap();
    // 当日账单
    let this_day_bill = this_month_bill.get_day_accs(now.day);
    // 昨日账单
    let last_day_bill;
    // 若昨日所属的月份与当前相同，否则月份改变采用上月日期
    if last_day.month == now.month {
        last_day_bill = this_month_bill.get_day_accs(last_day.day);
    } else {
        last_day_bill = last_month_bill.get_day_accs(last_day.day);
    }

    // 本日消费金额
    let today_expense = this_day_bill.sum_expense();
    // 昨日消费金额
    let last_expense = last_day_bill.sum_expense();
    // 本月消费金额
    let this_month_expense = this_month_bill.sum_expense();
    // 昨月消费金额
    let last_month_expense = last_month_bill.sum_expense();
    // 本月平均消费金额
    let this_month_mean = this_month_expense / (now.day as f64);
    // 上月平均消费金额
    let last_month_mean = last_month_expense / (last_month.sum_days() as f64);
    // 本日收入
    let today_get = this_day_bill.sum_in();
    // 本月收入
    let this_month_get = this_month_bill.sum_in();

    // 返回
    (
        -today_expense,
        -last_expense,
        -this_month_expense,
        -last_month_expense,
        -this_month_mean,
        -last_month_mean,
        today_get,
        this_month_get,
    )
}

/// 大额账单界面数据交互函数
#[tauri::command]
pub fn big_acc_data_load() -> String {
    let file_path = "data/big_acc.json";
    let data = load_json(file_path).unwrap();

    serde_json::to_string_pretty(&data).unwrap()
}

/// 大额账单数据保存
#[tauri::command]
pub fn big_acc_data_save(data: String) {
    let file_path = "data/big_acc.json";
    let data = serde_json::from_str(&data).unwrap();

    save_json(data, file_path).unwrap();
}

/// 设置读取
#[tauri::command]
pub fn get_config() -> String {
    let config = Config::load().unwrap();
    serde_json::to_string_pretty(&config).unwrap()
}

/// 设置保存
#[tauri::command]
pub fn save_config(config: String) {
    let config: Config = serde_json::from_str(&config).unwrap();
    config.save().unwrap();
}

/// 获取统计报表数据
#[tauri::command]
pub fn get_stat_data(year: i32) -> String {
    let mut bills = Vec::new();

    for month in 1..=12 {
        let mut bill = Bill::new(Some(year), Some(month));
        bill.load(&None).unwrap();
        bills.push(bill);
    }

    let stat_data = StatData::get_series(bills);

    serde_json::to_string_pretty(&stat_data).unwrap()
}

/// 获取每月统计报表数据
#[tauri::command]
pub fn get_day_stat(str: String) -> String {
    // 分解str
    let str: Vec<&str> = str.split("-").collect();
    // 获取年月
    let year = str[0].parse::<i32>().unwrap();
    let month = str[1].parse::<u32>().unwrap();

    // 读取相应数据
    let mut bill = Bill::new(Some(year), Some(month));
    bill.load(&None).unwrap();
    
    let day_bills = bill.to_day_bill();
    let stat_data = StatData::get_series(day_bills);
    serde_json::to_string_pretty(&stat_data).unwrap()
}

/// 获取饼图数据
#[tauri::command]
pub fn get_pie_data(bar_series: String, index: usize) -> String {
    let bar_series: Vec<StatData> = serde_json::from_str(&bar_series).unwrap();
    let pie_series = PieSeries::from_bar_series(bar_series, index);
    serde_json::to_string_pretty(&pie_series).unwrap()
}