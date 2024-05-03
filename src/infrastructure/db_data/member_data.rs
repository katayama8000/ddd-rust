use crate::domain::aggregate::{
    member::Member,
    value_object::{grade::Grade, major::Major, member_id::MemberId},
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct MemberData {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub grade: i32,
    pub major: String,
}

impl std::convert::From<Member> for MemberData {
    fn from(value: Member) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            age: value.age,
            grade: value.grade.into(),
            major: value.major.into(),
        }
    }
}

impl std::convert::TryFrom<MemberData> for Member {
    type Error = anyhow::Error;

    fn try_from(value: MemberData) -> Result<Self, Self::Error> {
        Ok(Member::reconstruct(
            MemberId::from(value.id),
            value.name,
            value.age,
            Grade::try_from(value.grade)?,
            Major::from(value.major.as_str()),
        ))
    }
}