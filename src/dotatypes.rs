use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizableEntity {
    pub name: String,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl LocalizableEntity {
    pub fn to_dict(&self) -> serde_json::Value {
        let mut m = serde_json::Map::new();
        m.insert("name".into(), self.name.clone().into());
        m.insert("key".into(), self.key.clone().into());
        if let Some(ref u) = self.username {
            m.insert("username".into(), u.clone().into());
        }
        serde_json::Value::Object(m)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    #[serde(flatten)]
    pub entity: LocalizableEntity,
}

impl Skill {
    pub fn to_dict(&self) -> serde_json::Value {
        self.entity.to_dict()
    }
    pub fn to_key_pair(&self) -> (String, String) {
        let v = self.entity.username.clone().unwrap_or_else(|| self.entity.name.clone());
        (self.entity.key.clone(), v)
    }
}

fn default_gender() -> String {
    "m".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hero {
    #[serde(flatten)]
    pub base: LocalizableEntity,
    #[serde(default = "default_gender")]
    pub gender: String,
    pub skills: Vec<Skill>,
}

impl Hero {
    pub fn to_dict(&self) -> serde_json::Value {
        let mut m = serde_json::Map::new();
        m.insert("name".into(), self.base.name.clone().into());
        m.insert("key".into(), self.base.key.clone().into());
        if let Some(ref u) = self.base.username {
            m.insert("username".into(), u.clone().into());
        }
        if self.gender == "f" {
            m.insert("gender".into(), "f".into());
        }
        let skills: Vec<serde_json::Value> = self.skills.iter().map(|s| s.to_dict()).collect();
        m.insert("skills".into(), skills.into());
        serde_json::Value::Object(m)
    }

    pub fn to_key_pairs(&self) -> HashMap<String, String> {
        let mut r = HashMap::new();
        let nv = self.base.username.clone().unwrap_or_else(|| self.base.name.clone());
        r.insert(self.base.key.clone(), format!("#|{}|#{}", self.gender, nv));
        for s in &self.skills {
            let (k, v) = s.to_key_pair();
            r.insert(k, v);
        }
        r
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    #[serde(flatten)]
    pub entity: LocalizableEntity,
}

impl Item {
    pub fn to_dict(&self) -> serde_json::Value {
        self.entity.to_dict()
    }
    pub fn to_key_pair(&self) -> (String, String) {
        let v = self.entity.username.clone().unwrap_or_else(|| self.entity.name.clone());
        (self.entity.key.clone(), v)
    }
}
