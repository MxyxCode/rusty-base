use crate::container::Containering;

use super::container::{Container, ContainerId};

macro_rules! storage_reflect {
    ($self:expr,$reflect:expr, $index:expr, $func:tt, $arg:expr) => {{
        let endpoint = get_endpoint!($self);

        endpoint.$func($arg.clone()).await?;

        if $reflect != true {
            return Ok(());
        };

        for tuple in $self.endpoints.iter().enumerate() {
            let (i, endpoint): (usize, &Box<dyn Storaging>) = tuple;

            if i == $index {
                continue;
            };

            endpoint.$func($arg.clone()).await?;
        }

        Ok(())
    }};
}

macro_rules! get_endpoint {
    ($self:expr) => {
        $self
            .endpoints
            .get($self.default_endpoint)
            .ok_or(eyre::Report::msg("Could not get the endpoint"))?
    };
}

pub type ContainerCollection = Vec<Container>;
pub type ContainerIdCollection = Vec<ContainerId>;

pub type OptionalContainer = Option<Container>;
pub type OptionalContainerCollection = Option<Vec<Container>>;

#[derive(Debug)]
pub struct Storage {
    pub endpoints: Vec<Box<dyn Storaging>>,
    pub default_endpoint: usize,
}

impl Storage {
    pub fn names(&self) -> Vec<&str> {
        self.endpoints
            .iter()
            .map(|ep| ep.name())
            .collect::<Vec<&str>>()
    }

    pub async fn write_container(
        &self,
        container: &impl Containering,
        endpoint_index: Option<usize>,
        reflect_on_all: bool,
    ) -> eyre::Result<()> {
        let container = container.to_container()?;
        let index = endpoint_index.unwrap_or(self.default_endpoint);

        storage_reflect!(self, reflect_on_all, index, write_container, container)
    }

    pub async fn write_containers(
        &self,
        containers: Vec<&impl Containering>,
        endpoint_index: Option<usize>,
        reflect_on_all: bool,
    ) -> eyre::Result<()> {
        let index = endpoint_index.unwrap_or(self.default_endpoint);
        let mut new_containers = Vec::new();

        for container in containers {
            let c = container.to_container()?;

            new_containers.push(c);
        }

        storage_reflect!(
            self,
            reflect_on_all,
            index,
            write_containers,
            new_containers
        )
    }

    async fn get_container_ids(&self) -> eyre::Result<ContainerIdCollection> {
        let endpoint = get_endpoint!(self);

        endpoint.get_container_ids().await
    }

    async fn exists_container(&self, id: &ContainerId) -> eyre::Result<bool> {
        let endpoint = get_endpoint!(self);

        endpoint.exists_container(&id).await
    }

    async fn read_container(&self, id: &ContainerId) -> eyre::Result<Container> {
        let endpoint = get_endpoint!(self);

        endpoint.read_container(&id).await
    }

    async fn read_containers(&self, ids: Vec<&ContainerId>) -> eyre::Result<ContainerCollection> {
        let endpoint = get_endpoint!(self);

        endpoint.read_containers(ids).await
    }

    async fn read_all_containers(&self) -> eyre::Result<ContainerCollection> {
        let endpoint = get_endpoint!(self);

        endpoint.read_all_containers().await
    }

    async fn delete_container(
        &self,
        id: &ContainerId,
        return_container: bool,
        reflect_on_all: bool,
    ) -> eyre::Result<OptionalContainer> {
        let endpoint = get_endpoint!(self);
        let container = endpoint.delete_container(&id, return_container).await?;

        if reflect_on_all == false {
            return Ok(container);
        };

        let default_index = self.default_endpoint;

        for (index, endpoint) in self.endpoints.iter().enumerate() {
            if index == default_index {
                continue;
            };

            endpoint.delete_container(&id, false).await?;
        }

        Ok(container)
    }

    async fn delete_containers(
        self,
        ids: Vec<&ContainerId>,
        return_containers: bool,
    ) -> eyre::Result<OptionalContainerCollection> {
        let endpoint = get_endpoint!(self);
        let containers = endpoint
            .delete_containers(ids.clone(), return_containers)
            .await?;

        if return_containers == false {
            return Ok(containers);
        }

        let default_index = self.default_endpoint;

        for (index, endpoint) in self.endpoints.iter().enumerate() {
            if index == default_index {
                continue;
            }

            endpoint.delete_containers(ids.clone(), false).await?;
        }

        Ok(containers)
    }
}

#[allow(clippy::wrong_self_convention)]
#[async_trait::async_trait]
pub trait Storaging: std::fmt::Debug {
    fn name(&self) -> &'static str;

    async fn write_container(&self, container: Container) -> eyre::Result<()>;
    async fn write_containers(&self, containers: ContainerCollection) -> eyre::Result<()>;

    async fn get_container_ids(&self) -> eyre::Result<ContainerIdCollection>;
    async fn exists_container(&self, id: &ContainerId) -> eyre::Result<bool>;

    async fn read_container(&self, id: &ContainerId) -> eyre::Result<Container>;
    async fn read_containers(&self, ids: Vec<&ContainerId>) -> eyre::Result<ContainerCollection>;
    async fn read_all_containers(&self) -> eyre::Result<ContainerCollection>;

    async fn delete_container(
        &self,
        id: &ContainerId,
        return_container: bool,
    ) -> eyre::Result<OptionalContainer>;
    async fn delete_containers(
        &self,
        ids: Vec<&ContainerId>,
        return_containers: bool,
    ) -> eyre::Result<OptionalContainerCollection>;
}
