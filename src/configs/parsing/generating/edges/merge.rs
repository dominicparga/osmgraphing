use crate::configs::SimpleId;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum Category {
    Id(SimpleId),
    Ignored,
}

impl From<ProtoCategory> for Category {
    fn from(proto_category: ProtoCategory) -> Category {
        match proto_category {
            ProtoCategory::Id(id) => Category::Id(id),
            ProtoCategory::Ignored => Category::Ignored,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum ProtoCategory {
    Id(SimpleId),
    Ignored,
}

impl From<RawCategory> for ProtoCategory {
    fn from(raw_category: RawCategory) -> ProtoCategory {
        match raw_category {
            RawCategory::Id(id) => ProtoCategory::Id(id),
            RawCategory::Ignored => ProtoCategory::Ignored,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RawCategory {
    Id(SimpleId),
    Ignored,
}
