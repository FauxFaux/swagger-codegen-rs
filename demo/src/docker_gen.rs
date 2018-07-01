struct Address {
    addr: String,
    prefix_len: i64,
}
struct AuthConfig {
    username: String,
    password: String,
    email: String,
    serveraddress: String,
}
struct BuildInfo {
    id: String,
    stream: String,
    error: String,
    error_detail: ErrorDetail,
    status: String,
    progress: String,
    progress_detail: ProgressDetail,
    aux: ImageID,
}
struct BuildPrune {
    space_reclaimed: i64,
}
struct ClusterInfo {
    id: String,
    version: ObjectVersion,
    created_at: ::chrono::DateTime,
    updated_at: ::chrono::DateTime,
    spec: SwarmSpec,
    tls_info: TLSInfo,
    root_rotation_in_progress: bool,
}
struct Commit {
    id: String,
    expected: String,
}
struct Config {
    id: String,
    version: ObjectVersion,
    created_at: ::chrono::DateTime,
    updated_at: ::chrono::DateTime,
    spec: ConfigSpec,
}
struct ConfigSpec {
    name: String,
    labels: (),
    data: String,
    templating: Driver,
}
struct ContainerArchiveInfo {
    message: String,
    message: String,
}
struct ContainerChanges {
    path: String,
    kind: u8,
}
struct ContainerConfig {
    hostname: String,
    domainname: String,
    user: String,
    attach_stdin: bool,
    attach_stdout: bool,
    attach_stderr: bool,
    exposed_ports: (),
    tty: bool,
    open_stdin: bool,
    stdin_once: bool,
    env: Vec<String>,
    cmd: Vec<String>,
    healthcheck: HealthConfig,
    args_escaped: bool,
    image: String,
    volumes: (),
    working_dir: String,
    entrypoint: Vec<String>,
    network_disabled: bool,
    mac_address: String,
    on_build: Vec<String>,
    labels: (),
    stop_signal: String,
    stop_timeout: i64,
    shell: Vec<String>,
}
struct ContainerCreateBody {
    hostname: String,
    domainname: String,
    user: String,
    attach_stdin: bool,
    attach_stdout: bool,
    attach_stderr: bool,
    exposed_ports: (),
    tty: bool,
    open_stdin: bool,
    stdin_once: bool,
    env: Vec<String>,
    cmd: Vec<String>,
    healthcheck: HealthConfig,
    args_escaped: bool,
    image: String,
    volumes: (),
    working_dir: String,
    entrypoint: Vec<String>,
    network_disabled: bool,
    mac_address: String,
    on_build: Vec<String>,
    labels: (),
    stop_signal: String,
    stop_timeout: i64,
    shell: Vec<String>,
    host_config: HostConfig,
    networking_config: ContainerCreateNetworkingConfig,
}
struct ContainerCreateCreated {
    id: String,
    warnings: Vec<String>,
}
struct ContainerCreateHostConfigBlkioWeightDevice {
    path: String,
    weight: u64,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ContainerCreateHostConfigIsolation {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "process")]
    Process,
    #[serde(rename = "hyperv")]
    Hyperv,
}
struct ContainerCreateHostConfigLogConfig {
    type_: ContainerCreateHostConfigLogConfigType,
    config: (),
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ContainerCreateHostConfigLogConfigType {
    #[serde(rename = "json-file")]
    Jsonfile,
    #[serde(rename = "syslog")]
    Syslog,
    #[serde(rename = "journald")]
    Journald,
    #[serde(rename = "gelf")]
    Gelf,
    #[serde(rename = "fluentd")]
    Fluentd,
    #[serde(rename = "awslogs")]
    Awslogs,
    #[serde(rename = "splunk")]
    Splunk,
    #[serde(rename = "etwlogs")]
    Etwlogs,
    #[serde(rename = "none")]
    None,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ContainerCreateHostConfigRestartPolicyName {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "unless-stopped")]
    Unlessstopped,
    #[serde(rename = "on-failure")]
    Onfailure,
}
struct ContainerCreateHostConfigUlimits {
    name: String,
    soft: i64,
    hard: i64,
}
struct ContainerCreateNetworkingConfig {
    endpoints_config: (),
}
struct ContainerExec {
    attach_stdin: bool,
    attach_stdout: bool,
    attach_stderr: bool,
    detach_keys: String,
    tty: bool,
    env: Vec<String>,
    cmd: Vec<String>,
    privileged: bool,
    user: String,
    working_dir: String,
}
struct ContainerInspect {
    id: String,
    created: String,
    path: String,
    args: Vec<String>,
    state: ContainerInspectState,
    image: String,
    resolv_conf_path: String,
    hostname_path: String,
    hosts_path: String,
    log_path: String,
    node: (),
    name: String,
    restart_count: i64,
    driver: String,
    mount_label: String,
    process_label: String,
    app_armor_profile: String,
    exec_i_ds: Vec<String>,
    host_config: HostConfig,
    graph_driver: GraphDriverData,
    size_rw: i64,
    size_root_fs: i64,
    mounts: Vec<MountPoint>,
    config: ContainerConfig,
    network_settings: NetworkSettings,
}
struct ContainerInspectState {
    status: ContainerInspectStateStatus,
    running: bool,
    paused: bool,
    restarting: bool,
    oom_killed: bool,
    dead: bool,
    pid: i64,
    exit_code: i64,
    error: String,
    started_at: String,
    finished_at: String,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ContainerInspectStateStatus {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "restarting")]
    Restarting,
    #[serde(rename = "removing")]
    Removing,
    #[serde(rename = "exited")]
    Exited,
    #[serde(rename = "dead")]
    Dead,
}
struct ContainerList {
    id: String,
    names: Vec<String>,
    image: String,
    image_id: String,
    command: String,
    created: i64,
    ports: Vec<Port>,
    size_rw: i64,
    size_root_fs: i64,
    labels: (),
    state: String,
    status: String,
    host_config: ContainerListHostConfig,
    network_settings: ContainerListNetworkSettings,
    mounts: Vec<Mount>,
}
struct ContainerListHostConfig {
    network_mode: String,
}
struct ContainerListMountsBindOptions {
    propagation: ContainerListMountsBindOptionsPropagation,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ContainerListMountsBindOptionsPropagation {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "rprivate")]
    Rprivate,
    #[serde(rename = "shared")]
    Shared,
    #[serde(rename = "rshared")]
    Rshared,
    #[serde(rename = "slave")]
    Slave,
    #[serde(rename = "rslave")]
    Rslave,
}
struct ContainerListMountsTmpfsOptions {
    size_bytes: i64,
    mode: i64,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ContainerListMountsType {
    #[serde(rename = "bind")]
    Bind,
    #[serde(rename = "volume")]
    Volume,
    #[serde(rename = "tmpfs")]
    Tmpfs,
}
struct ContainerListMountsVolumeOptions {
    no_copy: bool,
    labels: (),
    driver_config: ContainerListMountsVolumeOptionsDriverConfig,
}
struct ContainerListMountsVolumeOptionsDriverConfig {
    name: String,
    options: (),
}
struct ContainerListNetworkSettings {
    networks: (),
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ContainerListPortsType {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
    #[serde(rename = "sctp")]
    Sctp,
}
struct ContainerPrune {
    containers_deleted: Vec<String>,
    space_reclaimed: i64,
}
struct ContainerTop {
    titles: Vec<String>,
    processes: Vec<Vec<String>>,
}
struct ContainerUpdateOk {
    warnings: Vec<String>,
}
struct ContainerUpdateUpdate {
    cpu_shares: i64,
    memory: i64,
    cgroup_parent: String,
    blkio_weight: u64,
    blkio_weight_device: Vec<ContainerCreateHostConfigBlkioWeightDevice>,
    blkio_device_read_bps: Vec<ThrottleDevice>,
    blkio_device_write_bps: Vec<ThrottleDevice>,
    blkio_device_read_i_ops: Vec<ThrottleDevice>,
    blkio_device_write_i_ops: Vec<ThrottleDevice>,
    cpu_period: i64,
    cpu_quota: i64,
    cpu_realtime_period: i64,
    cpu_realtime_runtime: i64,
    cpuset_cpus: String,
    cpuset_mems: String,
    devices: Vec<DeviceMapping>,
    device_cgroup_rules: Vec<String>,
    disk_quota: i64,
    kernel_memory: i64,
    memory_reservation: i64,
    memory_swap: i64,
    memory_swappiness: u64,
    nano_cp_us: i64,
    oom_kill_disable: bool,
    init: bool,
    pids_limit: i64,
    ulimits: Vec<ContainerCreateHostConfigUlimits>,
    cpu_count: i64,
    cpu_percent: i64,
    io_maximum_i_ops: i64,
    io_maximum_bandwidth: i64,
    restart_policy: RestartPolicy,
}
struct ContainerWait {
    status_code: i64,
    error: ContainerWaitError,
}
struct ContainerWaitError {
    message: String,
}
struct CreateImageInfo {
    id: String,
    error: String,
    status: String,
    progress: String,
    progress_detail: ProgressDetail,
}
struct DeviceMapping {
    path_on_host: String,
    path_in_container: String,
    cgroup_permissions: String,
}
struct DistributionInspect {
    descriptor: DistributionInspectDescriptor,
    platforms: Vec<DistributionInspectPlatforms>,
}
struct DistributionInspectDescriptor {
    media_type: String,
    size: i64,
    digest: String,
    ur_ls: Vec<String>,
}
struct DistributionInspectPlatforms {
    architecture: String,
    os: String,
    os_version: String,
    os_features: Vec<String>,
    variant: String,
    features: Vec<String>,
}
struct Driver {
    name: String,
    options: (),
}
struct EndpointIPAMConfig {
    i_pv4_address: String,
    i_pv6_address: String,
    link_local_i_ps: Vec<String>,
}
struct EndpointPortConfig {
    name: String,
    protocol: ContainerListPortsType,
    target_port: i64,
    published_port: i64,
    publish_mode: ServiceListSpecEndpointSpecPortsPublishMode,
}
struct EndpointSettings {
    ipam_config: EndpointIPAMConfig,
    links: Vec<String>,
    aliases: Vec<String>,
    network_id: String,
    endpoint_id: String,
    gateway: String,
    ip_address: String,
    ip_prefix_len: i64,
    i_pv6_gateway: String,
    global_i_pv6_address: String,
    global_i_pv6_prefix_len: i64,
    mac_address: String,
    driver_opts: (),
}
struct EndpointSpec {
    mode: ServiceListSpecEndpointSpecMode,
    ports: Vec<EndpointPortConfig>,
}
struct EngineDescription {
    engine_version: String,
    labels: (),
    plugins: Vec<NodeListDescriptionEnginePlugins>,
}
struct ErrorDetail {
    code: i64,
    message: String,
}
struct ErrorResponse {
    message: String,
}
struct ExecInspect {
    can_remove: bool,
    detach_keys: String,
    id: String,
    running: bool,
    exit_code: i64,
    process_config: ProcessConfig,
    open_stdin: bool,
    open_stderr: bool,
    open_stdout: bool,
    container_id: String,
    pid: i64,
}
struct ExecStart {
    detach: bool,
    tty: bool,
}
struct GetPluginPrivileges {
    name: String,
    description: String,
    value: Vec<String>,
}
struct GraphDriverData {
    name: String,
    data: (),
}
struct HealthConfig {
    test: Vec<String>,
    interval: i64,
    timeout: i64,
    retries: i64,
    start_period: i64,
}
struct HostConfig {
    cpu_shares: i64,
    memory: i64,
    cgroup_parent: String,
    blkio_weight: u64,
    blkio_weight_device: Vec<ContainerCreateHostConfigBlkioWeightDevice>,
    blkio_device_read_bps: Vec<ThrottleDevice>,
    blkio_device_write_bps: Vec<ThrottleDevice>,
    blkio_device_read_i_ops: Vec<ThrottleDevice>,
    blkio_device_write_i_ops: Vec<ThrottleDevice>,
    cpu_period: i64,
    cpu_quota: i64,
    cpu_realtime_period: i64,
    cpu_realtime_runtime: i64,
    cpuset_cpus: String,
    cpuset_mems: String,
    devices: Vec<DeviceMapping>,
    device_cgroup_rules: Vec<String>,
    disk_quota: i64,
    kernel_memory: i64,
    memory_reservation: i64,
    memory_swap: i64,
    memory_swappiness: u64,
    nano_cp_us: i64,
    oom_kill_disable: bool,
    init: bool,
    pids_limit: i64,
    ulimits: Vec<ContainerCreateHostConfigUlimits>,
    cpu_count: i64,
    cpu_percent: i64,
    io_maximum_i_ops: i64,
    io_maximum_bandwidth: i64,
    binds: Vec<String>,
    container_id_file: String,
    log_config: ContainerCreateHostConfigLogConfig,
    network_mode: String,
    port_bindings: (),
    restart_policy: RestartPolicy,
    auto_remove: bool,
    volume_driver: String,
    volumes_from: Vec<String>,
    mounts: Vec<Mount>,
    cap_add: Vec<String>,
    cap_drop: Vec<String>,
    dns: Vec<String>,
    dns_options: Vec<String>,
    dns_search: Vec<String>,
    extra_hosts: Vec<String>,
    group_add: Vec<String>,
    ipc_mode: String,
    cgroup: String,
    links: Vec<String>,
    oom_score_adj: i64,
    pid_mode: String,
    privileged: bool,
    publish_all_ports: bool,
    readonly_rootfs: bool,
    security_opt: Vec<String>,
    storage_opt: (),
    tmpfs: (),
    uts_mode: String,
    userns_mode: String,
    shm_size: u64,
    sysctls: (),
    runtime: String,
    console_size: Vec<u64>,
    isolation: ContainerCreateHostConfigIsolation,
}
struct IPAM {
    driver: String,
    config: Vec<()>,
    options: Vec<()>,
}
struct IdResponse {
    id: String,
}
struct Image {
    id: String,
    repo_tags: Vec<String>,
    repo_digests: Vec<String>,
    parent: String,
    comment: String,
    created: String,
    container: String,
    container_config: ContainerConfig,
    docker_version: String,
    author: String,
    config: ContainerConfig,
    architecture: String,
    os: String,
    os_version: String,
    size: i64,
    virtual_size: i64,
    graph_driver: GraphDriverData,
    root_fs: ImageInspectRootFS,
    metadata: ImageInspectMetadata,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ImageBuild {
    #[serde(rename = "application/x-tar")]
    Applicationxtar,
}

impl Default for ImageBuild {
    fn default() -> Self {
        Self::Applicationxtar
    }
}
struct ImageDeleteResponseItem {
    untagged: String,
    deleted: String,
}
struct ImageHistory {
    id: String,
    created: i64,
    created_by: String,
    tags: Vec<String>,
    size: i64,
    comment: String,
}
struct ImageID {
    id: String,
}
struct ImageInspectMetadata {
    last_tag_time: ::chrono::DateTime,
}
struct ImageInspectRootFS {
    type_: String,
    layers: Vec<String>,
    base_layer: String,
}
struct ImagePrune {
    images_deleted: Vec<ImageDeleteResponseItem>,
    space_reclaimed: i64,
}
struct ImageSearch {
    description: String,
    is_official: bool,
    is_automated: bool,
    name: String,
    star_count: i64,
}
struct ImageSummary {
    id: String,
    parent_id: String,
    repo_tags: Vec<String>,
    repo_digests: Vec<String>,
    created: i64,
    size: i64,
    shared_size: i64,
    virtual_size: i64,
    labels: (),
    containers: i64,
}
struct IndexInfo {
    name: String,
    mirrors: Vec<String>,
    secure: bool,
    official: bool,
}
struct JoinTokens {
    worker: String,
    manager: String,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum LocalNodeState {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "locked")]
    Locked,
}

impl Default for LocalNodeState {
    fn default() -> Self {
        Self::Empty
    }
}
struct ManagerStatus {
    leader: bool,
    reachability: Reachability,
    addr: String,
}
struct Mount {
    target: String,
    source: String,
    type_: ContainerListMountsType,
    read_only: bool,
    consistency: String,
    bind_options: ContainerListMountsBindOptions,
    volume_options: ContainerListMountsVolumeOptions,
    tmpfs_options: ContainerListMountsTmpfsOptions,
}
struct MountPoint {
    type_: String,
    name: String,
    source: String,
    destination: String,
    driver: String,
    mode: String,
    rw: bool,
    propagation: String,
}
struct Network {
    name: String,
    id: String,
    created: ::chrono::DateTime,
    scope: String,
    driver: String,
    enable_i_pv6: bool,
    ipam: IPAM,
    internal: bool,
    attachable: bool,
    ingress: bool,
    containers: (),
    options: (),
    labels: (),
}
struct NetworkConnect {
    container: String,
    endpoint_config: EndpointSettings,
}
struct NetworkContainer {
    name: String,
    endpoint_id: String,
    mac_address: String,
    i_pv4_address: String,
    i_pv6_address: String,
}
struct NetworkCreateCreated {
    id: String,
    warning: String,
}
struct NetworkCreateNetworkConfig {
    name: String,
    check_duplicate: bool,
    driver: String,
    internal: bool,
    attachable: bool,
    ingress: bool,
    ipam: IPAM,
    enable_i_pv6: bool,
    options: (),
    labels: (),
}
struct NetworkDisconnect {
    container: String,
    force: bool,
}
struct NetworkPrune {
    networks_deleted: Vec<String>,
}
struct NetworkSettings {
    bridge: String,
    sandbox_id: String,
    hairpin_mode: bool,
    link_local_i_pv6_address: String,
    link_local_i_pv6_prefix_len: i64,
    ports: (),
    sandbox_key: String,
    secondary_ip_addresses: Vec<Address>,
    secondary_i_pv6_addresses: Vec<Address>,
    endpoint_id: String,
    gateway: String,
    global_i_pv6_address: String,
    global_i_pv6_prefix_len: i64,
    ip_address: String,
    ip_prefix_len: i64,
    i_pv6_gateway: String,
    mac_address: String,
    networks: (),
}
struct Node {
    id: String,
    version: ObjectVersion,
    created_at: ::chrono::DateTime,
    updated_at: ::chrono::DateTime,
    spec: NodeSpec,
    description: NodeDescription,
    status: NodeStatus,
    manager_status: ManagerStatus,
}
struct NodeDescription {
    hostname: String,
    platform: Platform,
    resources: ResourceObject,
    engine: EngineDescription,
    tls_info: TLSInfo,
}
struct NodeListDescriptionEnginePlugins {
    type_: String,
    name: String,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum NodeListSpecAvailability {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "drain")]
    Drain,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum NodeListSpecRole {
    #[serde(rename = "worker")]
    Worker,
    #[serde(rename = "manager")]
    Manager,
}
struct NodeSpec {
    name: String,
    labels: (),
    role: NodeListSpecRole,
    availability: NodeListSpecAvailability,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum NodeState {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "disconnected")]
    Disconnected,
}
struct NodeStatus {
    state: NodeState,
    message: String,
    addr: String,
}
struct ObjectVersion {
    inde: u64,
}
struct PeerNode {
    node_id: String,
    addr: String,
}
struct Platform {
    architecture: String,
    os: String,
}
struct Plugin {
    id: String,
    name: String,
    enabled: bool,
    settings: PluginListSettings,
    plugin_reference: String,
    config: PluginListConfig,
}
struct PluginDevice {
    name: String,
    description: String,
    settable: Vec<String>,
    path: String,
}
struct PluginEnv {
    name: String,
    description: String,
    settable: Vec<String>,
    value: String,
}
struct PluginInterfaceType {
    prefix: String,
    capability: String,
    version: String,
}
struct PluginListConfig {
    docker_version: String,
    description: String,
    documentation: String,
    interface: PluginListConfigInterface,
    entrypoint: Vec<String>,
    work_dir: String,
    user: PluginListConfigUser,
    network: PluginListConfigNetwork,
    linux: PluginListConfigLinux,
    propagated_mount: String,
    ipc_host: bool,
    pid_host: bool,
    mounts: Vec<PluginMount>,
    env: Vec<PluginEnv>,
    args: PluginListConfigArgs,
    rootfs: PluginListConfigrootfs,
}
struct PluginListConfigArgs {
    name: String,
    description: String,
    settable: Vec<String>,
    value: Vec<String>,
}
struct PluginListConfigInterface {
    types: Vec<PluginInterfaceType>,
    socket: String,
    protocol_scheme: PluginListConfigInterfaceProtocolScheme,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum PluginListConfigInterfaceProtocolScheme {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "moby.plugins.http/v1")]
    Mobypluginshttpv1,
}
struct PluginListConfigLinux {
    capabilities: Vec<String>,
    allow_all_devices: bool,
    devices: Vec<PluginDevice>,
}
struct PluginListConfigNetwork {
    type_: String,
}
struct PluginListConfigUser {
    uid: u32,
    gid: u32,
}
struct PluginListConfigrootfs {
    type_: String,
    diff_ids: Vec<String>,
}
struct PluginListSettings {
    mounts: Vec<PluginMount>,
    env: Vec<String>,
    args: Vec<String>,
    devices: Vec<PluginDevice>,
}
struct PluginMount {
    name: String,
    description: String,
    settable: Vec<String>,
    source: String,
    destination: String,
    type_: String,
    options: Vec<String>,
}
struct PluginsInfo {
    volume: Vec<String>,
    network: Vec<String>,
    authorization: Vec<String>,
    log: Vec<String>,
}
struct Port {
    ip: ::std::net::IpAddr,
    private_port: u16,
    public_port: u16,
    type_: ContainerListPortsType,
}
struct PortBinding {
    host_ip: String,
    host_port: String,
}
struct ProcessConfig {
    privileged: bool,
    user: String,
    tty: bool,
    entrypoint: String,
    arguments: Vec<String>,
}
struct ProgressDetail {
    current: i64,
    total: i64,
}
struct PushImageInfo {
    error: String,
    status: String,
    progress: String,
    progress_detail: ProgressDetail,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum Reachability {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "unreachable")]
    Unreachable,
    #[serde(rename = "reachable")]
    Reachable,
}
struct RegistryServiceConfig {
    allow_nondistributable_artifacts_cid_rs: Vec<String>,
    allow_nondistributable_artifacts_hostnames: Vec<String>,
    insecure_registry_cid_rs: Vec<String>,
    index_configs: (),
    mirrors: Vec<String>,
}
struct ResourceObject {
    nano_cp_us: i64,
    memory_bytes: i64,
    generic_resources: Vec<SystemInfoGenericResources>,
}
struct Resources {
    cpu_shares: i64,
    memory: i64,
    cgroup_parent: String,
    blkio_weight: u64,
    blkio_weight_device: Vec<ContainerCreateHostConfigBlkioWeightDevice>,
    blkio_device_read_bps: Vec<ThrottleDevice>,
    blkio_device_write_bps: Vec<ThrottleDevice>,
    blkio_device_read_i_ops: Vec<ThrottleDevice>,
    blkio_device_write_i_ops: Vec<ThrottleDevice>,
    cpu_period: i64,
    cpu_quota: i64,
    cpu_realtime_period: i64,
    cpu_realtime_runtime: i64,
    cpuset_cpus: String,
    cpuset_mems: String,
    devices: Vec<DeviceMapping>,
    device_cgroup_rules: Vec<String>,
    disk_quota: i64,
    kernel_memory: i64,
    memory_reservation: i64,
    memory_swap: i64,
    memory_swappiness: u64,
    nano_cp_us: i64,
    oom_kill_disable: bool,
    init: bool,
    pids_limit: i64,
    ulimits: Vec<ContainerCreateHostConfigUlimits>,
    cpu_count: i64,
    cpu_percent: i64,
    io_maximum_i_ops: i64,
    io_maximum_bandwidth: i64,
}
struct RestartPolicy {
    name: ContainerCreateHostConfigRestartPolicyName,
    maximum_retry_count: i64,
}
struct Runtime {
    path: String,
    runtime_args: Vec<String>,
}
struct Secret {
    id: String,
    version: ObjectVersion,
    created_at: ::chrono::DateTime,
    updated_at: ::chrono::DateTime,
    spec: SecretSpec,
}
struct SecretSpec {
    name: String,
    labels: (),
    data: String,
    driver: Driver,
    templating: Driver,
}
struct Service {
    id: String,
    version: ObjectVersion,
    created_at: ::chrono::DateTime,
    updated_at: ::chrono::DateTime,
    spec: ServiceSpec,
    endpoint: ServiceListEndpoint,
    update_status: ServiceListUpdateStatus,
}
struct ServiceCreate {
    id: String,
    warning: String,
}
struct ServiceListEndpoint {
    spec: EndpointSpec,
    ports: Vec<EndpointPortConfig>,
    virtual_i_ps: Vec<ServiceListEndpointVirtualIPs>,
}
struct ServiceListEndpointVirtualIPs {
    network_id: String,
    addr: String,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ServiceListSpecEndpointSpecMode {
    #[serde(rename = "vip")]
    Vip,
    #[serde(rename = "dnsrr")]
    Dnsrr,
}

impl Default for ServiceListSpecEndpointSpecMode {
    fn default() -> Self {
        Self::Vip
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ServiceListSpecEndpointSpecPortsPublishMode {
    #[serde(rename = "ingress")]
    Ingress,
    #[serde(rename = "host")]
    Host,
}

impl Default for ServiceListSpecEndpointSpecPortsPublishMode {
    fn default() -> Self {
        Self::Ingress
    }
}
struct ServiceListSpecMode {
    replicated: ServiceListSpecModeReplicated,
    global: (),
}
struct ServiceListSpecModeReplicated {
    replicas: i64,
}
struct ServiceListSpecRollbackConfig {
    parallelism: i64,
    delay: i64,
    failure_action: ServiceListSpecRollbackConfigFailureAction,
    monitor: i64,
    max_failure_ratio: f64,
    order: ServiceListSpecUpdateConfigOrder,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ServiceListSpecRollbackConfigFailureAction {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "pause")]
    Pause,
}
struct ServiceListSpecTaskTemplateContainerSpec {
    image: String,
    labels: (),
    command: Vec<String>,
    args: Vec<String>,
    hostname: String,
    env: Vec<String>,
    dir: String,
    user: String,
    groups: Vec<String>,
    privileges: ServiceListSpecTaskTemplateContainerSpecPrivileges,
    tty: bool,
    open_stdin: bool,
    read_only: bool,
    mounts: Vec<Mount>,
    stop_signal: String,
    stop_grace_period: i64,
    health_check: HealthConfig,
    hosts: Vec<String>,
    dns_config: ServiceListSpecTaskTemplateContainerSpecDNSConfig,
    secrets: Vec<ServiceListSpecTaskTemplateContainerSpecSecrets>,
    configs: Vec<ServiceListSpecTaskTemplateContainerSpecConfigs>,
    isolation: ContainerCreateHostConfigIsolation,
}
struct ServiceListSpecTaskTemplateContainerSpecConfigs {
    file: ServiceListSpecTaskTemplateContainerSpecSecretsFile,
    config_id: String,
    config_name: String,
}
struct ServiceListSpecTaskTemplateContainerSpecDNSConfig {
    nameservers: Vec<String>,
    search: Vec<String>,
    options: Vec<String>,
}
struct ServiceListSpecTaskTemplateContainerSpecPrivileges {
    credential_spec: ServiceListSpecTaskTemplateContainerSpecPrivilegesCredentialSpec,
    se_linux_context: ServiceListSpecTaskTemplateContainerSpecPrivilegesSELinuxContext,
}
struct ServiceListSpecTaskTemplateContainerSpecPrivilegesCredentialSpec {
    file: String,
    registry: String,
}
struct ServiceListSpecTaskTemplateContainerSpecPrivilegesSELinuxContext {
    disable: bool,
    user: String,
    role: String,
    type_: String,
    level: String,
}
struct ServiceListSpecTaskTemplateContainerSpecSecrets {
    file: ServiceListSpecTaskTemplateContainerSpecSecretsFile,
    secret_id: String,
    secret_name: String,
}
struct ServiceListSpecTaskTemplateContainerSpecSecretsFile {
    name: String,
    uid: String,
    gid: String,
    mode: u32,
}
struct ServiceListSpecTaskTemplateLogDriver {
    name: String,
    options: (),
}
struct ServiceListSpecTaskTemplateNetworkAttachmentSpec {
    container_id: String,
}
struct ServiceListSpecTaskTemplateNetworks {
    target: String,
    aliases: Vec<String>,
}
struct ServiceListSpecTaskTemplatePlacement {
    constraints: Vec<String>,
    preferences: Vec<ServiceListSpecTaskTemplatePlacementPreferences>,
    platforms: Vec<Platform>,
}
struct ServiceListSpecTaskTemplatePlacementPreferences {
    spread: ServiceListSpecTaskTemplatePlacementPreferencesSpread,
}
struct ServiceListSpecTaskTemplatePlacementPreferencesSpread {
    spread_descriptor: String,
}
struct ServiceListSpecTaskTemplatePluginSpec {
    name: String,
    remote: String,
    disabled: bool,
    plugin_privilege: Vec<GetPluginPrivileges>,
}
struct ServiceListSpecTaskTemplateResources {
    limits: ResourceObject,
    reservation: ResourceObject,
}
struct ServiceListSpecTaskTemplateRestartPolicy {
    condition: ServiceListSpecTaskTemplateRestartPolicyCondition,
    delay: i64,
    max_attempts: i64,
    window: i64,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ServiceListSpecTaskTemplateRestartPolicyCondition {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "on-failure")]
    Onfailure,
    #[serde(rename = "any")]
    Any,
}
struct ServiceListSpecUpdateConfig {
    parallelism: i64,
    delay: i64,
    failure_action: ServiceListSpecUpdateConfigFailureAction,
    monitor: i64,
    max_failure_ratio: f64,
    order: ServiceListSpecUpdateConfigOrder,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ServiceListSpecUpdateConfigFailureAction {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "rollback")]
    Rollback,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ServiceListSpecUpdateConfigOrder {
    #[serde(rename = "stop-first")]
    Stopfirst,
    #[serde(rename = "start-first")]
    Startfirst,
}
struct ServiceListUpdateStatus {
    state: ServiceListUpdateStatusState,
    started_at: ::chrono::DateTime,
    completed_at: ::chrono::DateTime,
    message: String,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ServiceListUpdateStatusState {
    #[serde(rename = "updating")]
    Updating,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "completed")]
    Completed,
}
struct ServiceSpec {
    name: String,
    labels: (),
    task_template: TaskSpec,
    mode: ServiceListSpecMode,
    update_config: ServiceListSpecUpdateConfig,
    rollback_config: ServiceListSpecRollbackConfig,
    networks: Vec<ServiceListSpecTaskTemplateNetworks>,
    endpoint_spec: EndpointSpec,
}
struct ServiceUpdateResponse {
    warnings: Vec<String>,
}
struct Swarm {
    id: String,
    version: ObjectVersion,
    created_at: ::chrono::DateTime,
    updated_at: ::chrono::DateTime,
    spec: SwarmSpec,
    tls_info: TLSInfo,
    root_rotation_in_progress: bool,
    join_tokens: JoinTokens,
}
struct SwarmInfo {
    node_id: String,
    node_addr: String,
    local_node_state: LocalNodeState,
    control_available: bool,
    error: String,
    remote_managers: Vec<PeerNode>,
    nodes: i64,
    managers: i64,
    cluster: ClusterInfo,
}
struct SwarmInit {
    listen_addr: String,
    advertise_addr: String,
    data_path_addr: String,
    force_new_cluster: bool,
    spec: SwarmSpec,
}
struct SwarmJoin {
    listen_addr: String,
    advertise_addr: String,
    data_path_addr: String,
    remote_addrs: String,
    join_token: String,
}
struct SwarmSpec {
    name: String,
    labels: (),
    orchestration: SystemInfoSwarmClusterSpecOrchestration,
    raft: SystemInfoSwarmClusterSpecRaft,
    dispatcher: SystemInfoSwarmClusterSpecDispatcher,
    ca_config: SystemInfoSwarmClusterSpecCAConfig,
    encryption_config: SystemInfoSwarmClusterSpecEncryptionConfig,
    task_defaults: SystemInfoSwarmClusterSpecTaskDefaults,
}
struct SwarmUnlockkey {
    unlock_key: String,
}
struct SystemAuth {
    status: String,
    identity_token: String,
}
struct SystemDataUsage {
    layers_size: i64,
    images: Vec<ImageSummary>,
    containers: Vec<Vec<ContainerList>>,
    volumes: Vec<Volume>,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum SystemDataUsageVolumesScope {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "global")]
    Global,
}

impl Default for SystemDataUsageVolumesScope {
    fn default() -> Self {
        Self::Local
    }
}
struct SystemDataUsageVolumesUsageData {
    size: i64,
    ref_count: i64,
}
struct SystemEvents {
    type_: String,
    action: String,
    actor: SystemEventsActor,
    time: i64,
    time_nano: i64,
}
struct SystemEventsActor {
    id: String,
    attributes: (),
}
struct SystemInfo {
    id: String,
    containers: i64,
    containers_running: i64,
    containers_paused: i64,
    containers_stopped: i64,
    images: i64,
    driver: String,
    driver_status: Vec<Vec<String>>,
    docker_root_dir: String,
    system_status: Vec<Vec<String>>,
    plugins: PluginsInfo,
    memory_limit: bool,
    swap_limit: bool,
    kernel_memory: bool,
    cpu_cfs_period: bool,
    cpu_cfs_quota: bool,
    cpu_shares: bool,
    cpu_set: bool,
    oom_kill_disable: bool,
    i_pv4_forwarding: bool,
    bridge_nf_iptables: bool,
    bridge_nf_ip6tables: bool,
    debug: bool,
    n_fd: i64,
    n_goroutines: i64,
    system_time: String,
    logging_driver: String,
    cgroup_driver: SystemInfoCgroupDriver,
    n_events_listener: i64,
    kernel_version: String,
    operating_system: String,
    os_type: String,
    architecture: String,
    ncpu: i64,
    mem_total: i64,
    index_server_address: String,
    registry_config: RegistryServiceConfig,
    generic_resources: Vec<SystemInfoGenericResources>,
    http_proxy: String,
    https_proxy: String,
    no_proxy: String,
    name: String,
    labels: Vec<String>,
    experimental_build: bool,
    server_version: String,
    cluster_store: String,
    cluster_advertise: String,
    runtimes: (),
    default_runtime: String,
    swarm: SwarmInfo,
    live_restore_enabled: bool,
    isolation: SystemInfoIsolation,
    init_binary: String,
    containerd_commit: Commit,
    runc_commit: Commit,
    init_commit: Commit,
    security_options: Vec<String>,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum SystemInfoCgroupDriver {
    #[serde(rename = "cgroupfs")]
    Cgroupfs,
    #[serde(rename = "systemd")]
    Systemd,
}

impl Default for SystemInfoCgroupDriver {
    fn default() -> Self {
        Self::Cgroupfs
    }
}
struct SystemInfoGenericResources {
    named_resource_spec: SystemInfoGenericResourcesNamedResourceSpec,
    discrete_resource_spec: SystemInfoGenericResourcesDiscreteResourceSpec,
}
struct SystemInfoGenericResourcesDiscreteResourceSpec {
    kind: String,
    value: i64,
}
struct SystemInfoGenericResourcesNamedResourceSpec {
    kind: String,
    value: String,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum SystemInfoIsolation {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "hyperv")]
    Hyperv,
    #[serde(rename = "process")]
    Process,
}

impl Default for SystemInfoIsolation {
    fn default() -> Self {
        Self::Default
    }
}
struct SystemInfoSwarmClusterSpecCAConfig {
    node_cert_expiry: i64,
    external_c_as: Vec<SystemInfoSwarmClusterSpecCAConfigExternalCAs>,
    signing_ca_cert: String,
    signing_ca_key: String,
    force_rotate: u64,
}
struct SystemInfoSwarmClusterSpecCAConfigExternalCAs {
    protocol: SystemInfoSwarmClusterSpecCAConfigExternalCAsProtocol,
    url: String,
    options: (),
    ca_cert: String,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum SystemInfoSwarmClusterSpecCAConfigExternalCAsProtocol {
    #[serde(rename = "cfssl")]
    Cfssl,
}

impl Default for SystemInfoSwarmClusterSpecCAConfigExternalCAsProtocol {
    fn default() -> Self {
        Self::Cfssl
    }
}
struct SystemInfoSwarmClusterSpecDispatcher {
    heartbeat_period: i64,
}
struct SystemInfoSwarmClusterSpecEncryptionConfig {
    auto_lock_managers: bool,
}
struct SystemInfoSwarmClusterSpecOrchestration {
    task_history_retention_limit: i64,
}
struct SystemInfoSwarmClusterSpecRaft {
    snapshot_interval: u64,
    keep_old_snapshots: u64,
    log_entries_for_slow_followers: u64,
    election_tick: i64,
    heartbeat_tick: i64,
}
struct SystemInfoSwarmClusterSpecTaskDefaults {
    log_driver: SystemInfoSwarmClusterSpecTaskDefaultsLogDriver,
}
struct SystemInfoSwarmClusterSpecTaskDefaultsLogDriver {
    name: String,
    options: (),
}
struct SystemVersion {
    platform: SystemVersionPlatform,
    components: Vec<SystemVersionComponents>,
    version: String,
    api_version: String,
    min_api_version: String,
    git_commit: String,
    go_version: String,
    os: String,
    arch: String,
    kernel_version: String,
    experimental: bool,
    build_time: String,
}
struct SystemVersionComponents {
    name: String,
    version: String,
    details: (),
}
struct SystemVersionPlatform {
    name: String,
}
struct TLSInfo {
    trust_root: String,
    cert_issuer_subject: String,
    cert_issuer_public_key: String,
}
struct Task {
    id: String,
    version: ObjectVersion,
    created_at: ::chrono::DateTime,
    updated_at: ::chrono::DateTime,
    name: String,
    labels: (),
    spec: TaskSpec,
    service_id: String,
    slot: i64,
    node_id: String,
    assigned_generic_resources: Vec<SystemInfoGenericResources>,
    status: TaskListStatus,
    desired_state: TaskState,
}
struct TaskListStatus {
    timestamp: ::chrono::DateTime,
    state: TaskState,
    message: String,
    err: String,
    container_status: TaskListStatusContainerStatus,
}
struct TaskListStatusContainerStatus {
    container_id: String,
    pid: i64,
    exit_code: i64,
}
struct TaskSpec {
    plugin_spec: ServiceListSpecTaskTemplatePluginSpec,
    container_spec: ServiceListSpecTaskTemplateContainerSpec,
    network_attachment_spec: ServiceListSpecTaskTemplateNetworkAttachmentSpec,
    resources: ServiceListSpecTaskTemplateResources,
    restart_policy: ServiceListSpecTaskTemplateRestartPolicy,
    placement: ServiceListSpecTaskTemplatePlacement,
    force_update: i64,
    runtime: String,
    networks: Vec<ServiceListSpecTaskTemplateNetworks>,
    log_driver: ServiceListSpecTaskTemplateLogDriver,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum TaskState {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "allocated")]
    Allocated,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "assigned")]
    Assigned,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "preparing")]
    Preparing,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "shutdown")]
    Shutdown,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "orphaned")]
    Orphaned,
}
struct ThrottleDevice {
    path: String,
    rate: u64,
}
struct Volume {
    name: String,
    driver: String,
    mountpoint: String,
    created_at: ::chrono::DateTime,
    status: (),
    labels: (),
    scope: SystemDataUsageVolumesScope,
    options: (),
    usage_data: SystemDataUsageVolumesUsageData,
}
struct VolumeCreate {
    name: String,
    driver: String,
    driver_opts: (),
    labels: (),
}
struct VolumeList {
    volumes: Vec<Volume>,
    warnings: Vec<String>,
}
struct VolumePrune {
    volumes_deleted: Vec<String>,
    space_reclaimed: i64,
}
// GET /containers/json
//   GET /containers/json
//   GET /containers/json
//   GET /containers/json
//   GET /containers/json
// POST /containers/create
//   POST /containers/create
//   POST /containers/create
// GET /containers/{id}/json
//   GET /containers/{id}/json
//   GET /containers/{id}/json
// GET /containers/{id}/top
//   GET /containers/{id}/top
//   GET /containers/{id}/top
// GET /containers/{id}/logs
//   GET /containers/{id}/logs
//   GET /containers/{id}/logs
//   GET /containers/{id}/logs
//   GET /containers/{id}/logs
//   GET /containers/{id}/logs
//   GET /containers/{id}/logs
//   GET /containers/{id}/logs
//   GET /containers/{id}/logs
// GET /containers/{id}/changes
//   GET /containers/{id}/changes
// GET /containers/{id}/export
//   GET /containers/{id}/export
// GET /containers/{id}/stats
//   GET /containers/{id}/stats
//   GET /containers/{id}/stats
// POST /containers/{id}/resize
//   POST /containers/{id}/resize
//   POST /containers/{id}/resize
//   POST /containers/{id}/resize
// POST /containers/{id}/start
//   POST /containers/{id}/start
//   POST /containers/{id}/start
// POST /containers/{id}/stop
//   POST /containers/{id}/stop
//   POST /containers/{id}/stop
// POST /containers/{id}/restart
//   POST /containers/{id}/restart
//   POST /containers/{id}/restart
// POST /containers/{id}/kill
//   POST /containers/{id}/kill
//   POST /containers/{id}/kill
// POST /containers/{id}/update
//   POST /containers/{id}/update
//   POST /containers/{id}/update
// POST /containers/{id}/rename
//   POST /containers/{id}/rename
//   POST /containers/{id}/rename
// POST /containers/{id}/pause
//   POST /containers/{id}/pause
// POST /containers/{id}/unpause
//   POST /containers/{id}/unpause
// POST /containers/{id}/attach
//   POST /containers/{id}/attach
//   POST /containers/{id}/attach
//   POST /containers/{id}/attach
//   POST /containers/{id}/attach
//   POST /containers/{id}/attach
//   POST /containers/{id}/attach
//   POST /containers/{id}/attach
// GET /containers/{id}/attach/ws
//   GET /containers/{id}/attach/ws
//   GET /containers/{id}/attach/ws
//   GET /containers/{id}/attach/ws
//   GET /containers/{id}/attach/ws
//   GET /containers/{id}/attach/ws
//   GET /containers/{id}/attach/ws
//   GET /containers/{id}/attach/ws
// POST /containers/{id}/wait
//   POST /containers/{id}/wait
//   POST /containers/{id}/wait
// DELETE /containers/{id}
//   DELETE /containers/{id}
//   DELETE /containers/{id}
//   DELETE /containers/{id}
//   DELETE /containers/{id}
// GET /containers/{id}/archive
//   GET /containers/{id}/archive
//   GET /containers/{id}/archive
// PUT /containers/{id}/archive
//   PUT /containers/{id}/archive
//   PUT /containers/{id}/archive
//   PUT /containers/{id}/archive
//   PUT /containers/{id}/archive
// HEAD /containers/{id}/archive
//   HEAD /containers/{id}/archive
//   HEAD /containers/{id}/archive
// POST /containers/prune
//   POST /containers/prune
// GET /images/json
//   GET /images/json
//   GET /images/json
//   GET /images/json
// POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
//   POST /build
// POST /build/prune
// POST /images/create
//   POST /images/create
//   POST /images/create
//   POST /images/create
//   POST /images/create
//   POST /images/create
//   POST /images/create
//   POST /images/create
// GET /images/{name}/json
//   GET /images/{name}/json
// GET /images/{name}/history
//   GET /images/{name}/history
// POST /images/{name}/push
//   POST /images/{name}/push
//   POST /images/{name}/push
//   POST /images/{name}/push
// POST /images/{name}/tag
//   POST /images/{name}/tag
//   POST /images/{name}/tag
//   POST /images/{name}/tag
// DELETE /images/{name}
//   DELETE /images/{name}
//   DELETE /images/{name}
//   DELETE /images/{name}
// GET /images/search
//   GET /images/search
//   GET /images/search
//   GET /images/search
// POST /images/prune
//   POST /images/prune
// POST /auth
//   POST /auth
// GET /info
// GET /version
// GET /_ping
// POST /commit
//   POST /commit
//   POST /commit
//   POST /commit
//   POST /commit
//   POST /commit
//   POST /commit
//   POST /commit
//   POST /commit
// GET /events
//   GET /events
//   GET /events
//   GET /events
// GET /system/df
// GET /images/{name}/get
//   GET /images/{name}/get
// GET /images/get
//   GET /images/get
// POST /images/load
//   POST /images/load
//   POST /images/load
// POST /containers/{id}/exec
//   POST /containers/{id}/exec
//   POST /containers/{id}/exec
// POST /exec/{id}/start
//   POST /exec/{id}/start
//   POST /exec/{id}/start
// POST /exec/{id}/resize
//   POST /exec/{id}/resize
//   POST /exec/{id}/resize
//   POST /exec/{id}/resize
// GET /exec/{id}/json
//   GET /exec/{id}/json
// GET /volumes
//   GET /volumes
// POST /volumes/create
//   POST /volumes/create
// GET /volumes/{name}
//   GET /volumes/{name}
// DELETE /volumes/{name}
//   DELETE /volumes/{name}
//   DELETE /volumes/{name}
// POST /volumes/prune
//   POST /volumes/prune
// GET /networks
//   GET /networks
// DELETE /networks/{id}
//   DELETE /networks/{id}
// GET /networks/{id}
//   GET /networks/{id}
//   GET /networks/{id}
//   GET /networks/{id}
// POST /networks/create
//   POST /networks/create
// POST /networks/{id}/connect
//   POST /networks/{id}/connect
//   POST /networks/{id}/connect
// POST /networks/{id}/disconnect
//   POST /networks/{id}/disconnect
//   POST /networks/{id}/disconnect
// POST /networks/prune
//   POST /networks/prune
// GET /plugins
//   GET /plugins
// GET /plugins/privileges
//   GET /plugins/privileges
// POST /plugins/pull
//   POST /plugins/pull
//   POST /plugins/pull
//   POST /plugins/pull
//   POST /plugins/pull
// GET /plugins/{name}/json
//   GET /plugins/{name}/json
// DELETE /plugins/{name}
//   DELETE /plugins/{name}
//   DELETE /plugins/{name}
// POST /plugins/{name}/enable
//   POST /plugins/{name}/enable
//   POST /plugins/{name}/enable
// POST /plugins/{name}/disable
//   POST /plugins/{name}/disable
// POST /plugins/{name}/upgrade
//   POST /plugins/{name}/upgrade
//   POST /plugins/{name}/upgrade
//   POST /plugins/{name}/upgrade
//   POST /plugins/{name}/upgrade
// POST /plugins/create
//   POST /plugins/create
//   POST /plugins/create
// POST /plugins/{name}/push
//   POST /plugins/{name}/push
// POST /plugins/{name}/set
//   POST /plugins/{name}/set
//   POST /plugins/{name}/set
// GET /nodes
//   GET /nodes
// DELETE /nodes/{id}
//   DELETE /nodes/{id}
//   DELETE /nodes/{id}
// GET /nodes/{id}
//   GET /nodes/{id}
// POST /nodes/{id}/update
//   POST /nodes/{id}/update
//   POST /nodes/{id}/update
//   POST /nodes/{id}/update
// GET /swarm
// POST /swarm/init
//   POST /swarm/init
// POST /swarm/join
//   POST /swarm/join
// POST /swarm/leave
//   POST /swarm/leave
// POST /swarm/update
//   POST /swarm/update
//   POST /swarm/update
//   POST /swarm/update
//   POST /swarm/update
//   POST /swarm/update
// GET /swarm/unlockkey
// POST /swarm/unlock
//   POST /swarm/unlock
// GET /services
//   GET /services
// POST /services/create
//   POST /services/create
//   POST /services/create
// DELETE /services/{id}
//   DELETE /services/{id}
// GET /services/{id}
//   GET /services/{id}
//   GET /services/{id}
// POST /services/{id}/update
//   POST /services/{id}/update
//   POST /services/{id}/update
//   POST /services/{id}/update
//   POST /services/{id}/update
//   POST /services/{id}/update
//   POST /services/{id}/update
// GET /services/{id}/logs
//   GET /services/{id}/logs
//   GET /services/{id}/logs
//   GET /services/{id}/logs
//   GET /services/{id}/logs
//   GET /services/{id}/logs
//   GET /services/{id}/logs
//   GET /services/{id}/logs
//   GET /services/{id}/logs
// GET /tasks
//   GET /tasks
// GET /tasks/{id}
//   GET /tasks/{id}
// GET /tasks/{id}/logs
//   GET /tasks/{id}/logs
//   GET /tasks/{id}/logs
//   GET /tasks/{id}/logs
//   GET /tasks/{id}/logs
//   GET /tasks/{id}/logs
//   GET /tasks/{id}/logs
//   GET /tasks/{id}/logs
//   GET /tasks/{id}/logs
// GET /secrets
//   GET /secrets
// POST /secrets/create
//   POST /secrets/create
// GET /secrets/{id}
//   GET /secrets/{id}
// DELETE /secrets/{id}
//   DELETE /secrets/{id}
// POST /secrets/{id}/update
//   POST /secrets/{id}/update
//   POST /secrets/{id}/update
//   POST /secrets/{id}/update
// GET /configs
//   GET /configs
// POST /configs/create
//   POST /configs/create
// DELETE /configs/{id}
//   DELETE /configs/{id}
// GET /configs/{id}
//   GET /configs/{id}
// POST /configs/{id}/update
//   POST /configs/{id}/update
//   POST /configs/{id}/update
//   POST /configs/{id}/update
// GET /distribution/{name}/json
//   GET /distribution/{name}/json
// POST /session
