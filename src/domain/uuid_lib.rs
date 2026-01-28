use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct Id(pub Uuid);

impl Id {
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    pub fn from_str(input: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(input).map(Id)
    }

    // pub fn uuid_from_str(input: &str) -> Result<Uuid, uuid::Error> {
    //     Uuid::parse_str(input)
    // }

    // pub fn new(id: String) -> Result<Self, uuid::Error> {
    //     Ok(Self(Uuid::parse_str(&id)?))
    // }

    // pub fn as_string(&self) -> String {
    //     self.0.to_string()
    // }
}
