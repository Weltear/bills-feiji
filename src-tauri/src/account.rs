use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer, de};

use crate::time::Date;

/// 账目结构体
#[derive(Clone, Debug)]
pub struct Account {
    /// 唯一识别编号，无符号整数，从零计数
    pub id: u32,
    /// 账目值，有符号整数，正为收益，负为亏损
    pub value: f64,
    /// 账目类型
    pub kind: Kind,
    /// 账目日期
    pub date: Date,
    /// 备注
    pub note: Option<String>,
}

impl Account {
    pub fn new(id: u32, value: f64, kind: Kind, date: Date, note: Option<String>) -> Self {
        Self{id, value, kind, date, note}
    }
}

impl Serialize for Account {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        // 创建一个新的序列化会话，Account自定义6个字段
        let mut state = serializer.serialize_struct("Account", 6)?;

        // 序列化id字段
        state.serialize_field("id", &self.id)?;

        // 序列化value字段
        state.serialize_field("value", &self.value)?;

        // 序列化kind字段，拆分为两个
        match &self.kind {
            Kind::In(str) => {
                state.serialize_field("attribute", "收入")?;
                state.serialize_field("kind", str)?;
            },
            Kind::Out(str) => {
                state.serialize_field("attribute", "支出")?;
                state.serialize_field("kind", str)?;
            }
        }

        // 序列化日期字段
        state.serialize_field("date", &self.date.to_string())?;

        // 序列化note字段
        match &self.note {
            Some(str) => state.serialize_field("note", str)?,
            None => state.serialize_field("note", "")?,
        }

        state.end()
    }
}

impl<'de> Deserialize<'de> for Account {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
        // 定义Visitor用以访问序列化数据
        struct AccountVisitor;

        impl<'de> serde::de::Visitor<'de> for AccountVisitor {
            type Value = Account;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Account")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>, {
                let mut id: Option<u32> = None;
                let mut value: Option<f64> = None;
                let mut attribute: Option<&str> = None;
                let mut kind: Option<String> = None;
                let mut date: Option<&str> = None;
                let mut note: Option<String> = None;

                // 遍历map中的每个键值对
                while let Some(key) = map.next_key()? {
                    match key {
                        "id" => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        "value" => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                        "attribute" => {
                            if attribute.is_some() {
                                return Err(de::Error::duplicate_field("attribute"));
                            }
                            attribute = Some(map.next_value()?);
                        }
                        "kind" => {
                            if kind.is_some() {
                                return Err(de::Error::duplicate_field("kind"));
                            }
                            kind = Some(map.next_value()?);
                        }
                        "date" => {
                            if date.is_some() {
                                return Err(de::Error::duplicate_field("date"));
                            }
                            date = Some(map.next_value()?);
                        }
                        "note" => {
                            if note.is_some() {
                                return Err(de::Error::duplicate_field("note"));
                            }
                            note = Some(map.next_value()?);
                        }
                        _ => {
                            return Err(de::Error::unknown_field(key, &["id", "value", "attribute", "kind", "date", "note"]));
                        }
                    }
                }

                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                let attribute = attribute.ok_or_else(|| de::Error::missing_field("attribute"))?;
                let kind = kind.ok_or_else(|| de::Error::missing_field("kind"))?;
                let date = date.ok_or_else(|| de::Error::missing_field("date"))?;

                let kind = match attribute {
                    "收入" => Kind::In(kind),
                    "支出" => Kind::Out(kind),
                    _ => return Err(de::Error::invalid_value(de::Unexpected::Str(attribute), &"收入 or 支出")),
                };

                let date = Date::from_str(date).unwrap();

                let note = match note {
                    Some(str) if !str.is_empty() => Some(str),
                    _ => None,
                };

                let res = Account {
                    id,
                    value,
                    kind,
                    date,
                    note,
                };
                Ok(res)
            }
        }

        const FIELDS: &'static [&'static str] = &["id", "value", "attribute", "kind", "date", "note"];
        deserializer.deserialize_struct("Account", FIELDS, AccountVisitor)
    }
}

/// 账单基础种类枚举，分为支出或收入
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Kind {
    In(String),
    Out(String),
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn accout_serde() {
        let acc = Account {
            id: 0,
            value: 32f64,
            kind: Kind::In(String::from("工资")),
            date: Date::now(),
            note: Some("2月工资".to_string()),
        };
        let str = serde_json::to_string_pretty(&acc).unwrap();
        println!("{}", str);

        let res: Account = serde_json::from_str(&str).unwrap();
        println!("{:?}", res);
    }
}