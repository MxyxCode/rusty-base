Author: Mycodee

----

# Storge Api
```
  Storage Api is designed to storage the data in modular way. 
  So you can swap the direct access of the file system with storage servers.
```

----

# Types
```
  VecIndex -> usize
  
  ContainerId -> Bytes

  ContainerCollection -> Vec<Container>
  OptionalContainer -> Option<Container>
  OptionalContainerCollection -> Vec<Option<Vec<Container>>>
```

# Enums 

ContainerKind 
```
  Data // The Data of the database, table entrys or schemas or required data
  Link // The Linking of Data Containers with the purpose, Indexing, Schema, Entry Containers
  Bundle // The Bundeling of Linking Containers with a Id
```

# Structs 

Container 
```
  id: ContainerId
  kind: ContainerKind
  data: Bytes
```

Storage
```
  endpoint: Vec<Storaging>
  default_endpoint: VecIndex
  
 async fn write_container(&self, container: impl Containering) -> Result<()>
 async fn write_containers(&self, containers: Vec<impl Containering>) -> Result<()>

 async fn get_container_ids(&self) -> Result<Vec<String>>
 async fn exist_container(&self, id: &ContainerId) -> Result<bool>

 async fn read_container(&self, id: ContainerId) -> Result<Container>
 async fn read_containers(&self, ids: Vec<&ContainerId>) -> Result<ContainerCollection>
 async fn read_all_containers(&self) -> Result<ContainerCollection>

 async fn delete_container(&self, id: &ContainerId, return_container: bool) -> Result<OptionalContainer>
 async fn delete_containers(&self, ids: Vec<&ContainerId>, return_containers: bool) -> Result<OptionalContainerCollection>
 async fn delete_all_containers(&self, return_containers: bool) -> Result<OptionalContainerCollection>
```

# Traits 

Containering
```
  fn purpose() -> &'static str

  fn id(&self) -> ContainerId

  fn valid_container(&self, &Container) -> Result<Boolean>

  fn from_container(&self, container: &Container) -> Result<Self>
  fn to_container(&self) -> Result<Container>
```

Storaging
```
  fn name() -> &'static str
  
  async fn write_container(&self, container: Container) -> Result<()>
  async fn write_containers(&self, containers: Vec<Container>) -> Result<()>
  
  async fn get_container_ids(&self) -> Result<Vec<String>>
  async fn exist_container(&self, id: &ContainerId) -> Result<bool>
  
  async fn read_container(&self, id: &ContainerId) -> Result<Container>
  async fn read_containers(&self, ids: Vec<&ContainerId>) -> Result<ContainerCollection>
  async fn read_all_containers(&self) -> Result<ContainerCollection>
  
  async fn delete_container(&self, id: &ContainerId, return_container: bool) -> Result<OptionalContainer>
  async fn delete_containers(&self, ids: Vec<&ContainerId>, return_containers: bool) -> Result<OptionalContainerCollection>
  async fn delete_all_containers(&self, return_containers: bool) -> Result<OptionalContainerCollection>
```
