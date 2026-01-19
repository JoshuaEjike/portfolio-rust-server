use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    pub fn from_str(input: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(input).map(UserId)
    }

    pub fn uuid_from_str(input: &str) -> Result<Uuid, uuid::Error> {
        Uuid::parse_str(input)
    }

    pub fn as_string(&self) -> String {
        self.0.to_string()
    }
}
