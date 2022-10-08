pub type ContainerId = Vec<u8>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ContainerKind {
    Data,
    Link,
    Bundle,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Container {
    pub id: ContainerId,
    pub kind: ContainerKind,
    pub data: Vec<u8>,
}

#[allow(clippy::wrong_self_convention)]
pub trait Containering: std::fmt::Debug {
    fn purpose() -> &'static str {
        "None"
    }

    fn id(&self) -> ContainerId;

    fn valid_container(&self, container: &Container) -> eyre::Result<bool>
    where
        Self: Sized;

    fn from_container(container: &Container) -> eyre::Result<Self>
    where
        Self: Sized;
    fn to_container(&self) -> eyre::Result<Container>
    where
        Self: Sized;
}
