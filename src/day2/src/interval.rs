pub struct Interval {
    pub start: i64,
    pub end: i64,
}

pub enum Id {
    Valid(i64),
    Invalid(i64),
}

impl Id {
    pub(crate) fn is_valid(&self) -> bool {
        matches!(self, Id::Valid(_))
    }
    pub fn value(&self) -> i64 {
        match self {
            Id::Valid(v) => *v,
            Id::Invalid(v) => *v,
        }
    }
}

pub fn determine_id_status(id: i64) -> Id {
    let id_str = id.to_string();
    let mid = id_str.len() / 2;
    let (left, right) = id_str.split_at(mid);

    if left == right {
        Id::Invalid(id)
    } else {
        Id::Valid(id)
    }
}
