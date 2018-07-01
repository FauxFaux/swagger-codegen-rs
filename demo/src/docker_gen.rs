#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Address {
    #[serde(rename = "Addr")]
    addr: String,
    #[serde(rename = "PrefixLen")]
    prefix_len: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct AuthConfig {
    #[serde(rename = "username")]
    username: String,
    #[serde(rename = "password")]
    password: String,
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "serveraddress")]
    serveraddress: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct BuildInfo {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "stream")]
    stream: String,
    #[serde(rename = "error")]
    error: String,
    #[serde(rename = "errorDetail")]
    error_detail: ErrorDetail,
    #[serde(rename = "status")]
    status: String,
    #[serde(rename = "progress")]
    progress: String,
    #[serde(rename = "progressDetail")]
    progress_detail: ProgressDetail,
    #[serde(rename = "aux")]
    aux: ImageID,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct BuildPrune {
    #[serde(rename = "SpaceReclaimed")]
    space_reclaimed: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ClusterInfo {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Version")]
    version: ObjectVersion,
    #[serde(rename = "CreatedAt")]
    created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "UpdatedAt")]
    updated_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "Spec")]
    spec: SwarmSpec,
    #[serde(rename = "TLSInfo")]
    tls_info: TLSInfo,
    #[serde(rename = "RootRotationInProgress")]
    root_rotation_in_progress: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Commit {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Expected")]
    expected: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Config {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Version")]
    version: ObjectVersion,
    #[serde(rename = "CreatedAt")]
    created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "UpdatedAt")]
    updated_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "Spec")]
    spec: ConfigSpec,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ConfigSpec {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Labels")]
    labels: (),
    #[serde(rename = "Data")]
    data: String,
    #[serde(rename = "Templating")]
    templating: Driver,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerChanges {
    #[serde(rename = "Path")]
    path: String,
    #[serde(rename = "Kind")]
    kind: u8,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerConfig {
    #[serde(rename = "Hostname")]
    hostname: String,
    #[serde(rename = "Domainname")]
    domainname: String,
    #[serde(rename = "User")]
    user: String,
    #[serde(rename = "AttachStdin")]
    attach_stdin: bool,
    #[serde(rename = "AttachStdout")]
    attach_stdout: bool,
    #[serde(rename = "AttachStderr")]
    attach_stderr: bool,
    #[serde(rename = "ExposedPorts")]
    exposed_ports: (),
    #[serde(rename = "Tty")]
    tty: bool,
    #[serde(rename = "OpenStdin")]
    open_stdin: bool,
    #[serde(rename = "StdinOnce")]
    stdin_once: bool,
    #[serde(rename = "Env")]
    env: Vec<String>,
    #[serde(rename = "Cmd")]
    cmd: Vec<String>,
    #[serde(rename = "Healthcheck")]
    healthcheck: HealthConfig,
    #[serde(rename = "ArgsEscaped")]
    args_escaped: bool,
    #[serde(rename = "Image")]
    image: String,
    #[serde(rename = "Volumes")]
    volumes: (),
    #[serde(rename = "WorkingDir")]
    working_dir: String,
    #[serde(rename = "Entrypoint")]
    entrypoint: Vec<String>,
    #[serde(rename = "NetworkDisabled")]
    network_disabled: bool,
    #[serde(rename = "MacAddress")]
    mac_address: String,
    #[serde(rename = "OnBuild")]
    on_build: Vec<String>,
    #[serde(rename = "Labels")]
    labels: (),
    #[serde(rename = "StopSignal")]
    stop_signal: String,
    #[serde(rename = "StopTimeout")]
    stop_timeout: i64,
    #[serde(rename = "Shell")]
    shell: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerCreateBody {
    #[serde(rename = "Hostname")]
    hostname: String,
    #[serde(rename = "Domainname")]
    domainname: String,
    #[serde(rename = "User")]
    user: String,
    #[serde(rename = "AttachStdin")]
    attach_stdin: bool,
    #[serde(rename = "AttachStdout")]
    attach_stdout: bool,
    #[serde(rename = "AttachStderr")]
    attach_stderr: bool,
    #[serde(rename = "ExposedPorts")]
    exposed_ports: (),
    #[serde(rename = "Tty")]
    tty: bool,
    #[serde(rename = "OpenStdin")]
    open_stdin: bool,
    #[serde(rename = "StdinOnce")]
    stdin_once: bool,
    #[serde(rename = "Env")]
    env: Vec<String>,
    #[serde(rename = "Cmd")]
    cmd: Vec<String>,
    #[serde(rename = "Healthcheck")]
    healthcheck: HealthConfig,
    #[serde(rename = "ArgsEscaped")]
    args_escaped: bool,
    #[serde(rename = "Image")]
    image: String,
    #[serde(rename = "Volumes")]
    volumes: (),
    #[serde(rename = "WorkingDir")]
    working_dir: String,
    #[serde(rename = "Entrypoint")]
    entrypoint: Vec<String>,
    #[serde(rename = "NetworkDisabled")]
    network_disabled: bool,
    #[serde(rename = "MacAddress")]
    mac_address: String,
    #[serde(rename = "OnBuild")]
    on_build: Vec<String>,
    #[serde(rename = "Labels")]
    labels: (),
    #[serde(rename = "StopSignal")]
    stop_signal: String,
    #[serde(rename = "StopTimeout")]
    stop_timeout: i64,
    #[serde(rename = "Shell")]
    shell: Vec<String>,
    #[serde(rename = "HostConfig")]
    host_config: HostConfig,
    #[serde(rename = "NetworkingConfig")]
    networking_config: ContainerCreateNetworkingConfig,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerCreateCreated {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Warnings")]
    warnings: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerCreateHostConfigBlkioWeightDevice {
    #[serde(rename = "Path")]
    path: String,
    #[serde(rename = "Weight")]
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerCreateHostConfigLogConfig {
    #[serde(rename = "Type")]
    type_: ContainerCreateHostConfigLogConfigType,
    #[serde(rename = "Config")]
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerCreateHostConfigUlimits {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Soft")]
    soft: i64,
    #[serde(rename = "Hard")]
    hard: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerCreateNetworkingConfig {
    #[serde(rename = "EndpointsConfig")]
    endpoints_config: (),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerExec {
    #[serde(rename = "AttachStdin")]
    attach_stdin: bool,
    #[serde(rename = "AttachStdout")]
    attach_stdout: bool,
    #[serde(rename = "AttachStderr")]
    attach_stderr: bool,
    #[serde(rename = "DetachKeys")]
    detach_keys: String,
    #[serde(rename = "Tty")]
    tty: bool,
    #[serde(rename = "Env")]
    env: Vec<String>,
    #[serde(rename = "Cmd")]
    cmd: Vec<String>,
    #[serde(rename = "Privileged")]
    privileged: bool,
    #[serde(rename = "User")]
    user: String,
    #[serde(rename = "WorkingDir")]
    working_dir: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerInspect {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Created")]
    created: String,
    #[serde(rename = "Path")]
    path: String,
    #[serde(rename = "Args")]
    args: Vec<String>,
    #[serde(rename = "State")]
    state: ContainerInspectState,
    #[serde(rename = "Image")]
    image: String,
    #[serde(rename = "ResolvConfPath")]
    resolv_conf_path: String,
    #[serde(rename = "HostnamePath")]
    hostname_path: String,
    #[serde(rename = "HostsPath")]
    hosts_path: String,
    #[serde(rename = "LogPath")]
    log_path: String,
    #[serde(rename = "Node")]
    node: (),
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "RestartCount")]
    restart_count: i64,
    #[serde(rename = "Driver")]
    driver: String,
    #[serde(rename = "MountLabel")]
    mount_label: String,
    #[serde(rename = "ProcessLabel")]
    process_label: String,
    #[serde(rename = "AppArmorProfile")]
    app_armor_profile: String,
    #[serde(rename = "ExecIDs")]
    exec_i_ds: Vec<String>,
    #[serde(rename = "HostConfig")]
    host_config: HostConfig,
    #[serde(rename = "GraphDriver")]
    graph_driver: GraphDriverData,
    #[serde(rename = "SizeRw")]
    size_rw: i64,
    #[serde(rename = "SizeRootFs")]
    size_root_fs: i64,
    #[serde(rename = "Mounts")]
    mounts: Vec<MountPoint>,
    #[serde(rename = "Config")]
    config: ContainerConfig,
    #[serde(rename = "NetworkSettings")]
    network_settings: NetworkSettings,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerInspectState {
    #[serde(rename = "Status")]
    status: ContainerInspectStateStatus,
    #[serde(rename = "Running")]
    running: bool,
    #[serde(rename = "Paused")]
    paused: bool,
    #[serde(rename = "Restarting")]
    restarting: bool,
    #[serde(rename = "OOMKilled")]
    oom_killed: bool,
    #[serde(rename = "Dead")]
    dead: bool,
    #[serde(rename = "Pid")]
    pid: i64,
    #[serde(rename = "ExitCode")]
    exit_code: i64,
    #[serde(rename = "Error")]
    error: String,
    #[serde(rename = "StartedAt")]
    started_at: String,
    #[serde(rename = "FinishedAt")]
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerList {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Names")]
    names: Vec<String>,
    #[serde(rename = "Image")]
    image: String,
    #[serde(rename = "ImageID")]
    image_id: String,
    #[serde(rename = "Command")]
    command: String,
    #[serde(rename = "Created")]
    created: i64,
    #[serde(rename = "Ports")]
    ports: Vec<Port>,
    #[serde(rename = "SizeRw")]
    size_rw: i64,
    #[serde(rename = "SizeRootFs")]
    size_root_fs: i64,
    #[serde(rename = "Labels")]
    labels: (),
    #[serde(rename = "State")]
    state: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "HostConfig")]
    host_config: ContainerListHostConfig,
    #[serde(rename = "NetworkSettings")]
    network_settings: ContainerListNetworkSettings,
    #[serde(rename = "Mounts")]
    mounts: Vec<Mount>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerListHostConfig {
    #[serde(rename = "NetworkMode")]
    network_mode: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerListMountsBindOptions {
    #[serde(rename = "Propagation")]
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerListMountsTmpfsOptions {
    #[serde(rename = "SizeBytes")]
    size_bytes: i64,
    #[serde(rename = "Mode")]
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerListMountsVolumeOptions {
    #[serde(rename = "NoCopy")]
    no_copy: bool,
    #[serde(rename = "Labels")]
    labels: (),
    #[serde(rename = "DriverConfig")]
    driver_config: ContainerListMountsVolumeOptionsDriverConfig,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerListMountsVolumeOptionsDriverConfig {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Options")]
    options: (),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerListNetworkSettings {
    #[serde(rename = "Networks")]
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerPrune {
    #[serde(rename = "ContainersDeleted")]
    containers_deleted: Vec<String>,
    #[serde(rename = "SpaceReclaimed")]
    space_reclaimed: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerTop {
    #[serde(rename = "Titles")]
    titles: Vec<String>,
    #[serde(rename = "Processes")]
    processes: Vec<Vec<String>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerUpdateOk {
    #[serde(rename = "Warnings")]
    warnings: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerUpdateUpdate {
    #[serde(rename = "CpuShares")]
    cpu_shares: i64,
    #[serde(rename = "Memory")]
    memory: i64,
    #[serde(rename = "CgroupParent")]
    cgroup_parent: String,
    #[serde(rename = "BlkioWeight")]
    blkio_weight: u64,
    #[serde(rename = "BlkioWeightDevice")]
    blkio_weight_device: Vec<ContainerCreateHostConfigBlkioWeightDevice>,
    #[serde(rename = "BlkioDeviceReadBps")]
    blkio_device_read_bps: Vec<ThrottleDevice>,
    #[serde(rename = "BlkioDeviceWriteBps")]
    blkio_device_write_bps: Vec<ThrottleDevice>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    blkio_device_read_i_ops: Vec<ThrottleDevice>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    blkio_device_write_i_ops: Vec<ThrottleDevice>,
    #[serde(rename = "CpuPeriod")]
    cpu_period: i64,
    #[serde(rename = "CpuQuota")]
    cpu_quota: i64,
    #[serde(rename = "CpuRealtimePeriod")]
    cpu_realtime_period: i64,
    #[serde(rename = "CpuRealtimeRuntime")]
    cpu_realtime_runtime: i64,
    #[serde(rename = "CpusetCpus")]
    cpuset_cpus: String,
    #[serde(rename = "CpusetMems")]
    cpuset_mems: String,
    #[serde(rename = "Devices")]
    devices: Vec<DeviceMapping>,
    #[serde(rename = "DeviceCgroupRules")]
    device_cgroup_rules: Vec<String>,
    #[serde(rename = "DiskQuota")]
    disk_quota: i64,
    #[serde(rename = "KernelMemory")]
    kernel_memory: i64,
    #[serde(rename = "MemoryReservation")]
    memory_reservation: i64,
    #[serde(rename = "MemorySwap")]
    memory_swap: i64,
    #[serde(rename = "MemorySwappiness")]
    memory_swappiness: u64,
    #[serde(rename = "NanoCPUs")]
    nano_cp_us: i64,
    #[serde(rename = "OomKillDisable")]
    oom_kill_disable: bool,
    #[serde(rename = "Init")]
    init: bool,
    #[serde(rename = "PidsLimit")]
    pids_limit: i64,
    #[serde(rename = "Ulimits")]
    ulimits: Vec<ContainerCreateHostConfigUlimits>,
    #[serde(rename = "CpuCount")]
    cpu_count: i64,
    #[serde(rename = "CpuPercent")]
    cpu_percent: i64,
    #[serde(rename = "IOMaximumIOps")]
    io_maximum_i_ops: i64,
    #[serde(rename = "IOMaximumBandwidth")]
    io_maximum_bandwidth: i64,
    #[serde(rename = "RestartPolicy")]
    restart_policy: RestartPolicy,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerWait {
    #[serde(rename = "StatusCode")]
    status_code: i64,
    #[serde(rename = "Error")]
    error: ContainerWaitError,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ContainerWaitError {
    #[serde(rename = "Message")]
    message: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct CreateImageInfo {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "error")]
    error: String,
    #[serde(rename = "status")]
    status: String,
    #[serde(rename = "progress")]
    progress: String,
    #[serde(rename = "progressDetail")]
    progress_detail: ProgressDetail,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct DeviceMapping {
    #[serde(rename = "PathOnHost")]
    path_on_host: String,
    #[serde(rename = "PathInContainer")]
    path_in_container: String,
    #[serde(rename = "CgroupPermissions")]
    cgroup_permissions: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct DistributionInspect {
    #[serde(rename = "Descriptor")]
    descriptor: DistributionInspectDescriptor,
    #[serde(rename = "Platforms")]
    platforms: Vec<DistributionInspectPlatforms>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct DistributionInspectDescriptor {
    #[serde(rename = "MediaType")]
    media_type: String,
    #[serde(rename = "Size")]
    size: i64,
    #[serde(rename = "Digest")]
    digest: String,
    #[serde(rename = "URLs")]
    ur_ls: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct DistributionInspectPlatforms {
    #[serde(rename = "Architecture")]
    architecture: String,
    #[serde(rename = "OS")]
    os: String,
    #[serde(rename = "OSVersion")]
    os_version: String,
    #[serde(rename = "OSFeatures")]
    os_features: Vec<String>,
    #[serde(rename = "Variant")]
    variant: String,
    #[serde(rename = "Features")]
    features: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Driver {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Options")]
    options: (),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct EndpointIPAMConfig {
    #[serde(rename = "IPv4Address")]
    i_pv4_address: String,
    #[serde(rename = "IPv6Address")]
    i_pv6_address: String,
    #[serde(rename = "LinkLocalIPs")]
    link_local_i_ps: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct EndpointPortConfig {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Protocol")]
    protocol: ContainerListPortsType,
    #[serde(rename = "TargetPort")]
    target_port: i64,
    #[serde(rename = "PublishedPort")]
    published_port: i64,
    #[serde(rename = "PublishMode")]
    publish_mode: ServiceListSpecEndpointSpecPortsPublishMode,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct EndpointSettings {
    #[serde(rename = "IPAMConfig")]
    ipam_config: EndpointIPAMConfig,
    #[serde(rename = "Links")]
    links: Vec<String>,
    #[serde(rename = "Aliases")]
    aliases: Vec<String>,
    #[serde(rename = "NetworkID")]
    network_id: String,
    #[serde(rename = "EndpointID")]
    endpoint_id: String,
    #[serde(rename = "Gateway")]
    gateway: String,
    #[serde(rename = "IPAddress")]
    ip_address: String,
    #[serde(rename = "IPPrefixLen")]
    ip_prefix_len: i64,
    #[serde(rename = "IPv6Gateway")]
    i_pv6_gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    global_i_pv6_address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    global_i_pv6_prefix_len: i64,
    #[serde(rename = "MacAddress")]
    mac_address: String,
    #[serde(rename = "DriverOpts")]
    driver_opts: (),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct EndpointSpec {
    #[serde(rename = "Mode")]
    mode: ServiceListSpecEndpointSpecMode,
    #[serde(rename = "Ports")]
    ports: Vec<EndpointPortConfig>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct EngineDescription {
    #[serde(rename = "EngineVersion")]
    engine_version: String,
    #[serde(rename = "Labels")]
    labels: (),
    #[serde(rename = "Plugins")]
    plugins: Vec<NodeListDescriptionEnginePlugins>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ErrorDetail {
    #[serde(rename = "code")]
    code: i64,
    #[serde(rename = "message")]
    message: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ErrorResponse {
    #[serde(rename = "message")]
    message: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ExecInspect {
    #[serde(rename = "CanRemove")]
    can_remove: bool,
    #[serde(rename = "DetachKeys")]
    detach_keys: String,
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Running")]
    running: bool,
    #[serde(rename = "ExitCode")]
    exit_code: i64,
    #[serde(rename = "ProcessConfig")]
    process_config: ProcessConfig,
    #[serde(rename = "OpenStdin")]
    open_stdin: bool,
    #[serde(rename = "OpenStderr")]
    open_stderr: bool,
    #[serde(rename = "OpenStdout")]
    open_stdout: bool,
    #[serde(rename = "ContainerID")]
    container_id: String,
    #[serde(rename = "Pid")]
    pid: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ExecStart {
    #[serde(rename = "Detach")]
    detach: bool,
    #[serde(rename = "Tty")]
    tty: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct GetPluginPrivileges {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Value")]
    value: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct GraphDriverData {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Data")]
    data: (),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct HealthConfig {
    #[serde(rename = "Test")]
    test: Vec<String>,
    #[serde(rename = "Interval")]
    interval: i64,
    #[serde(rename = "Timeout")]
    timeout: i64,
    #[serde(rename = "Retries")]
    retries: i64,
    #[serde(rename = "StartPeriod")]
    start_period: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct HostConfig {
    #[serde(rename = "CpuShares")]
    cpu_shares: i64,
    #[serde(rename = "Memory")]
    memory: i64,
    #[serde(rename = "CgroupParent")]
    cgroup_parent: String,
    #[serde(rename = "BlkioWeight")]
    blkio_weight: u64,
    #[serde(rename = "BlkioWeightDevice")]
    blkio_weight_device: Vec<ContainerCreateHostConfigBlkioWeightDevice>,
    #[serde(rename = "BlkioDeviceReadBps")]
    blkio_device_read_bps: Vec<ThrottleDevice>,
    #[serde(rename = "BlkioDeviceWriteBps")]
    blkio_device_write_bps: Vec<ThrottleDevice>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    blkio_device_read_i_ops: Vec<ThrottleDevice>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    blkio_device_write_i_ops: Vec<ThrottleDevice>,
    #[serde(rename = "CpuPeriod")]
    cpu_period: i64,
    #[serde(rename = "CpuQuota")]
    cpu_quota: i64,
    #[serde(rename = "CpuRealtimePeriod")]
    cpu_realtime_period: i64,
    #[serde(rename = "CpuRealtimeRuntime")]
    cpu_realtime_runtime: i64,
    #[serde(rename = "CpusetCpus")]
    cpuset_cpus: String,
    #[serde(rename = "CpusetMems")]
    cpuset_mems: String,
    #[serde(rename = "Devices")]
    devices: Vec<DeviceMapping>,
    #[serde(rename = "DeviceCgroupRules")]
    device_cgroup_rules: Vec<String>,
    #[serde(rename = "DiskQuota")]
    disk_quota: i64,
    #[serde(rename = "KernelMemory")]
    kernel_memory: i64,
    #[serde(rename = "MemoryReservation")]
    memory_reservation: i64,
    #[serde(rename = "MemorySwap")]
    memory_swap: i64,
    #[serde(rename = "MemorySwappiness")]
    memory_swappiness: u64,
    #[serde(rename = "NanoCPUs")]
    nano_cp_us: i64,
    #[serde(rename = "OomKillDisable")]
    oom_kill_disable: bool,
    #[serde(rename = "Init")]
    init: bool,
    #[serde(rename = "PidsLimit")]
    pids_limit: i64,
    #[serde(rename = "Ulimits")]
    ulimits: Vec<ContainerCreateHostConfigUlimits>,
    #[serde(rename = "CpuCount")]
    cpu_count: i64,
    #[serde(rename = "CpuPercent")]
    cpu_percent: i64,
    #[serde(rename = "IOMaximumIOps")]
    io_maximum_i_ops: i64,
    #[serde(rename = "IOMaximumBandwidth")]
    io_maximum_bandwidth: i64,
    #[serde(rename = "Binds")]
    binds: Vec<String>,
    #[serde(rename = "ContainerIDFile")]
    container_id_file: String,
    #[serde(rename = "LogConfig")]
    log_config: ContainerCreateHostConfigLogConfig,
    #[serde(rename = "NetworkMode")]
    network_mode: String,
    #[serde(rename = "PortBindings")]
    port_bindings: (),
    #[serde(rename = "RestartPolicy")]
    restart_policy: RestartPolicy,
    #[serde(rename = "AutoRemove")]
    auto_remove: bool,
    #[serde(rename = "VolumeDriver")]
    volume_driver: String,
    #[serde(rename = "VolumesFrom")]
    volumes_from: Vec<String>,
    #[serde(rename = "Mounts")]
    mounts: Vec<Mount>,
    #[serde(rename = "CapAdd")]
    cap_add: Vec<String>,
    #[serde(rename = "CapDrop")]
    cap_drop: Vec<String>,
    #[serde(rename = "Dns")]
    dns: Vec<String>,
    #[serde(rename = "DnsOptions")]
    dns_options: Vec<String>,
    #[serde(rename = "DnsSearch")]
    dns_search: Vec<String>,
    #[serde(rename = "ExtraHosts")]
    extra_hosts: Vec<String>,
    #[serde(rename = "GroupAdd")]
    group_add: Vec<String>,
    #[serde(rename = "IpcMode")]
    ipc_mode: String,
    #[serde(rename = "Cgroup")]
    cgroup: String,
    #[serde(rename = "Links")]
    links: Vec<String>,
    #[serde(rename = "OomScoreAdj")]
    oom_score_adj: i64,
    #[serde(rename = "PidMode")]
    pid_mode: String,
    #[serde(rename = "Privileged")]
    privileged: bool,
    #[serde(rename = "PublishAllPorts")]
    publish_all_ports: bool,
    #[serde(rename = "ReadonlyRootfs")]
    readonly_rootfs: bool,
    #[serde(rename = "SecurityOpt")]
    security_opt: Vec<String>,
    #[serde(rename = "StorageOpt")]
    storage_opt: (),
    #[serde(rename = "Tmpfs")]
    tmpfs: (),
    #[serde(rename = "UTSMode")]
    uts_mode: String,
    #[serde(rename = "UsernsMode")]
    userns_mode: String,
    #[serde(rename = "ShmSize")]
    shm_size: u64,
    #[serde(rename = "Sysctls")]
    sysctls: (),
    #[serde(rename = "Runtime")]
    runtime: String,
    #[serde(rename = "ConsoleSize")]
    console_size: Vec<u64>,
    #[serde(rename = "Isolation")]
    isolation: ContainerCreateHostConfigIsolation,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct IPAM {
    #[serde(rename = "Driver")]
    driver: String,
    #[serde(rename = "Config")]
    config: Vec<()>,
    #[serde(rename = "Options")]
    options: Vec<()>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct IdResponse {
    #[serde(rename = "Id")]
    id: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Image {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "RepoTags")]
    repo_tags: Vec<String>,
    #[serde(rename = "RepoDigests")]
    repo_digests: Vec<String>,
    #[serde(rename = "Parent")]
    parent: String,
    #[serde(rename = "Comment")]
    comment: String,
    #[serde(rename = "Created")]
    created: String,
    #[serde(rename = "Container")]
    container: String,
    #[serde(rename = "ContainerConfig")]
    container_config: ContainerConfig,
    #[serde(rename = "DockerVersion")]
    docker_version: String,
    #[serde(rename = "Author")]
    author: String,
    #[serde(rename = "Config")]
    config: ContainerConfig,
    #[serde(rename = "Architecture")]
    architecture: String,
    #[serde(rename = "Os")]
    os: String,
    #[serde(rename = "OsVersion")]
    os_version: String,
    #[serde(rename = "Size")]
    size: i64,
    #[serde(rename = "VirtualSize")]
    virtual_size: i64,
    #[serde(rename = "GraphDriver")]
    graph_driver: GraphDriverData,
    #[serde(rename = "RootFS")]
    root_fs: ImageInspectRootFS,
    #[serde(rename = "Metadata")]
    metadata: ImageInspectMetadata,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ImageBuild {
    #[serde(rename = "application/x-tar")]
    Applicationxtar,
}

impl Default for ImageBuild {
    fn default() -> Self {
        ImageBuild::Applicationxtar
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ImageDeleteResponseItem {
    #[serde(rename = "Untagged")]
    untagged: String,
    #[serde(rename = "Deleted")]
    deleted: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ImageHistory {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Created")]
    created: i64,
    #[serde(rename = "CreatedBy")]
    created_by: String,
    #[serde(rename = "Tags")]
    tags: Vec<String>,
    #[serde(rename = "Size")]
    size: i64,
    #[serde(rename = "Comment")]
    comment: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ImageID {
    #[serde(rename = "ID")]
    id: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ImageInspectMetadata {
    #[serde(rename = "LastTagTime")]
    last_tag_time: ::chrono::DateTime<::chrono::Utc>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ImageInspectRootFS {
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Layers")]
    layers: Vec<String>,
    #[serde(rename = "BaseLayer")]
    base_layer: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ImagePrune {
    #[serde(rename = "ImagesDeleted")]
    images_deleted: Vec<ImageDeleteResponseItem>,
    #[serde(rename = "SpaceReclaimed")]
    space_reclaimed: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ImageSearch {
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "is_official")]
    is_official: bool,
    #[serde(rename = "is_automated")]
    is_automated: bool,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "star_count")]
    star_count: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ImageSummary {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "ParentId")]
    parent_id: String,
    #[serde(rename = "RepoTags")]
    repo_tags: Vec<String>,
    #[serde(rename = "RepoDigests")]
    repo_digests: Vec<String>,
    #[serde(rename = "Created")]
    created: i64,
    #[serde(rename = "Size")]
    size: i64,
    #[serde(rename = "SharedSize")]
    shared_size: i64,
    #[serde(rename = "VirtualSize")]
    virtual_size: i64,
    #[serde(rename = "Labels")]
    labels: (),
    #[serde(rename = "Containers")]
    containers: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct IndexInfo {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Mirrors")]
    mirrors: Vec<String>,
    #[serde(rename = "Secure")]
    secure: bool,
    #[serde(rename = "Official")]
    official: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct JoinTokens {
    #[serde(rename = "Worker")]
    worker: String,
    #[serde(rename = "Manager")]
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
        LocalNodeState::Empty
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ManagerStatus {
    #[serde(rename = "Leader")]
    leader: bool,
    #[serde(rename = "Reachability")]
    reachability: Reachability,
    #[serde(rename = "Addr")]
    addr: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Mount {
    #[serde(rename = "Target")]
    target: String,
    #[serde(rename = "Source")]
    source: String,
    #[serde(rename = "Type")]
    type_: ContainerListMountsType,
    #[serde(rename = "ReadOnly")]
    read_only: bool,
    #[serde(rename = "Consistency")]
    consistency: String,
    #[serde(rename = "BindOptions")]
    bind_options: ContainerListMountsBindOptions,
    #[serde(rename = "VolumeOptions")]
    volume_options: ContainerListMountsVolumeOptions,
    #[serde(rename = "TmpfsOptions")]
    tmpfs_options: ContainerListMountsTmpfsOptions,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct MountPoint {
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Source")]
    source: String,
    #[serde(rename = "Destination")]
    destination: String,
    #[serde(rename = "Driver")]
    driver: String,
    #[serde(rename = "Mode")]
    mode: String,
    #[serde(rename = "RW")]
    rw: bool,
    #[serde(rename = "Propagation")]
    propagation: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Network {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Created")]
    created: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "Scope")]
    scope: String,
    #[serde(rename = "Driver")]
    driver: String,
    #[serde(rename = "EnableIPv6")]
    enable_i_pv6: bool,
    #[serde(rename = "IPAM")]
    ipam: IPAM,
    #[serde(rename = "Internal")]
    internal: bool,
    #[serde(rename = "Attachable")]
    attachable: bool,
    #[serde(rename = "Ingress")]
    ingress: bool,
    #[serde(rename = "Containers")]
    containers: (),
    #[serde(rename = "Options")]
    options: (),
    #[serde(rename = "Labels")]
    labels: (),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct NetworkConnect {
    #[serde(rename = "Container")]
    container: String,
    #[serde(rename = "EndpointConfig")]
    endpoint_config: EndpointSettings,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct NetworkContainer {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "EndpointID")]
    endpoint_id: String,
    #[serde(rename = "MacAddress")]
    mac_address: String,
    #[serde(rename = "IPv4Address")]
    i_pv4_address: String,
    #[serde(rename = "IPv6Address")]
    i_pv6_address: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct NetworkCreateCreated {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Warning")]
    warning: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct NetworkCreateNetworkConfig {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "CheckDuplicate")]
    check_duplicate: bool,
    #[serde(rename = "Driver")]
    driver: String,
    #[serde(rename = "Internal")]
    internal: bool,
    #[serde(rename = "Attachable")]
    attachable: bool,
    #[serde(rename = "Ingress")]
    ingress: bool,
    #[serde(rename = "IPAM")]
    ipam: IPAM,
    #[serde(rename = "EnableIPv6")]
    enable_i_pv6: bool,
    #[serde(rename = "Options")]
    options: (),
    #[serde(rename = "Labels")]
    labels: (),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct NetworkDisconnect {
    #[serde(rename = "Container")]
    container: String,
    #[serde(rename = "Force")]
    force: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct NetworkPrune {
    #[serde(rename = "NetworksDeleted")]
    networks_deleted: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct NetworkSettings {
    #[serde(rename = "Bridge")]
    bridge: String,
    #[serde(rename = "SandboxID")]
    sandbox_id: String,
    #[serde(rename = "HairpinMode")]
    hairpin_mode: bool,
    #[serde(rename = "LinkLocalIPv6Address")]
    link_local_i_pv6_address: String,
    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    link_local_i_pv6_prefix_len: i64,
    #[serde(rename = "Ports")]
    ports: (),
    #[serde(rename = "SandboxKey")]
    sandbox_key: String,
    #[serde(rename = "SecondaryIPAddresses")]
    secondary_ip_addresses: Vec<Address>,
    #[serde(rename = "SecondaryIPv6Addresses")]
    secondary_i_pv6_addresses: Vec<Address>,
    #[serde(rename = "EndpointID")]
    endpoint_id: String,
    #[serde(rename = "Gateway")]
    gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    global_i_pv6_address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    global_i_pv6_prefix_len: i64,
    #[serde(rename = "IPAddress")]
    ip_address: String,
    #[serde(rename = "IPPrefixLen")]
    ip_prefix_len: i64,
    #[serde(rename = "IPv6Gateway")]
    i_pv6_gateway: String,
    #[serde(rename = "MacAddress")]
    mac_address: String,
    #[serde(rename = "Networks")]
    networks: (),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Node {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Version")]
    version: ObjectVersion,
    #[serde(rename = "CreatedAt")]
    created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "UpdatedAt")]
    updated_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "Spec")]
    spec: NodeSpec,
    #[serde(rename = "Description")]
    description: NodeDescription,
    #[serde(rename = "Status")]
    status: NodeStatus,
    #[serde(rename = "ManagerStatus")]
    manager_status: ManagerStatus,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct NodeDescription {
    #[serde(rename = "Hostname")]
    hostname: String,
    #[serde(rename = "Platform")]
    platform: Platform,
    #[serde(rename = "Resources")]
    resources: ResourceObject,
    #[serde(rename = "Engine")]
    engine: EngineDescription,
    #[serde(rename = "TLSInfo")]
    tls_info: TLSInfo,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct NodeListDescriptionEnginePlugins {
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Name")]
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct NodeSpec {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Labels")]
    labels: (),
    #[serde(rename = "Role")]
    role: NodeListSpecRole,
    #[serde(rename = "Availability")]
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct NodeStatus {
    #[serde(rename = "State")]
    state: NodeState,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Addr")]
    addr: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ObjectVersion {
    #[serde(rename = "Inde")]
    inde: u64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PeerNode {
    #[serde(rename = "NodeID")]
    node_id: String,
    #[serde(rename = "Addr")]
    addr: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Platform {
    #[serde(rename = "Architecture")]
    architecture: String,
    #[serde(rename = "OS")]
    os: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Plugin {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Enabled")]
    enabled: bool,
    #[serde(rename = "Settings")]
    settings: PluginListSettings,
    #[serde(rename = "PluginReference")]
    plugin_reference: String,
    #[serde(rename = "Config")]
    config: PluginListConfig,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PluginDevice {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Settable")]
    settable: Vec<String>,
    #[serde(rename = "Path")]
    path: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PluginEnv {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Settable")]
    settable: Vec<String>,
    #[serde(rename = "Value")]
    value: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PluginInterfaceType {
    #[serde(rename = "Prefix")]
    prefix: String,
    #[serde(rename = "Capability")]
    capability: String,
    #[serde(rename = "Version")]
    version: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PluginListConfig {
    #[serde(rename = "DockerVersion")]
    docker_version: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Documentation")]
    documentation: String,
    #[serde(rename = "Interface")]
    interface: PluginListConfigInterface,
    #[serde(rename = "Entrypoint")]
    entrypoint: Vec<String>,
    #[serde(rename = "WorkDir")]
    work_dir: String,
    #[serde(rename = "User")]
    user: PluginListConfigUser,
    #[serde(rename = "Network")]
    network: PluginListConfigNetwork,
    #[serde(rename = "Linux")]
    linux: PluginListConfigLinux,
    #[serde(rename = "PropagatedMount")]
    propagated_mount: String,
    #[serde(rename = "IpcHost")]
    ipc_host: bool,
    #[serde(rename = "PidHost")]
    pid_host: bool,
    #[serde(rename = "Mounts")]
    mounts: Vec<PluginMount>,
    #[serde(rename = "Env")]
    env: Vec<PluginEnv>,
    #[serde(rename = "Args")]
    args: PluginListConfigArgs,
    #[serde(rename = "rootfs")]
    rootfs: PluginListConfigrootfs,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PluginListConfigArgs {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Settable")]
    settable: Vec<String>,
    #[serde(rename = "Value")]
    value: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PluginListConfigInterface {
    #[serde(rename = "Types")]
    types: Vec<PluginInterfaceType>,
    #[serde(rename = "Socket")]
    socket: String,
    #[serde(rename = "ProtocolScheme")]
    protocol_scheme: PluginListConfigInterfaceProtocolScheme,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum PluginListConfigInterfaceProtocolScheme {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "moby.plugins.http/v1")]
    Mobypluginshttpv1,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PluginListConfigLinux {
    #[serde(rename = "Capabilities")]
    capabilities: Vec<String>,
    #[serde(rename = "AllowAllDevices")]
    allow_all_devices: bool,
    #[serde(rename = "Devices")]
    devices: Vec<PluginDevice>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PluginListConfigNetwork {
    #[serde(rename = "Type")]
    type_: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PluginListConfigUser {
    #[serde(rename = "UID")]
    uid: u32,
    #[serde(rename = "GID")]
    gid: u32,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PluginListConfigrootfs {
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename = "diff_ids")]
    diff_ids: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PluginListSettings {
    #[serde(rename = "Mounts")]
    mounts: Vec<PluginMount>,
    #[serde(rename = "Env")]
    env: Vec<String>,
    #[serde(rename = "Args")]
    args: Vec<String>,
    #[serde(rename = "Devices")]
    devices: Vec<PluginDevice>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PluginMount {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Settable")]
    settable: Vec<String>,
    #[serde(rename = "Source")]
    source: String,
    #[serde(rename = "Destination")]
    destination: String,
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Options")]
    options: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PluginsInfo {
    #[serde(rename = "Volume")]
    volume: Vec<String>,
    #[serde(rename = "Network")]
    network: Vec<String>,
    #[serde(rename = "Authorization")]
    authorization: Vec<String>,
    #[serde(rename = "Log")]
    log: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Port {
    #[serde(rename = "IP")]
    ip: ::std::net::IpAddr,
    #[serde(rename = "PrivatePort")]
    private_port: u16,
    #[serde(rename = "PublicPort")]
    public_port: u16,
    #[serde(rename = "Type")]
    type_: ContainerListPortsType,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PortBinding {
    #[serde(rename = "HostIp")]
    host_ip: String,
    #[serde(rename = "HostPort")]
    host_port: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ProcessConfig {
    #[serde(rename = "privileged")]
    privileged: bool,
    #[serde(rename = "user")]
    user: String,
    #[serde(rename = "tty")]
    tty: bool,
    #[serde(rename = "entrypoint")]
    entrypoint: String,
    #[serde(rename = "arguments")]
    arguments: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ProgressDetail {
    #[serde(rename = "current")]
    current: i64,
    #[serde(rename = "total")]
    total: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PushImageInfo {
    #[serde(rename = "error")]
    error: String,
    #[serde(rename = "status")]
    status: String,
    #[serde(rename = "progress")]
    progress: String,
    #[serde(rename = "progressDetail")]
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct RegistryServiceConfig {
    #[serde(rename = "AllowNondistributableArtifactsCIDRs")]
    allow_nondistributable_artifacts_cid_rs: Vec<String>,
    #[serde(rename = "AllowNondistributableArtifactsHostnames")]
    allow_nondistributable_artifacts_hostnames: Vec<String>,
    #[serde(rename = "InsecureRegistryCIDRs")]
    insecure_registry_cid_rs: Vec<String>,
    #[serde(rename = "IndexConfigs")]
    index_configs: (),
    #[serde(rename = "Mirrors")]
    mirrors: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ResourceObject {
    #[serde(rename = "NanoCPUs")]
    nano_cp_us: i64,
    #[serde(rename = "MemoryBytes")]
    memory_bytes: i64,
    #[serde(rename = "GenericResources")]
    generic_resources: Vec<SystemInfoGenericResources>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Resources {
    #[serde(rename = "CpuShares")]
    cpu_shares: i64,
    #[serde(rename = "Memory")]
    memory: i64,
    #[serde(rename = "CgroupParent")]
    cgroup_parent: String,
    #[serde(rename = "BlkioWeight")]
    blkio_weight: u64,
    #[serde(rename = "BlkioWeightDevice")]
    blkio_weight_device: Vec<ContainerCreateHostConfigBlkioWeightDevice>,
    #[serde(rename = "BlkioDeviceReadBps")]
    blkio_device_read_bps: Vec<ThrottleDevice>,
    #[serde(rename = "BlkioDeviceWriteBps")]
    blkio_device_write_bps: Vec<ThrottleDevice>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    blkio_device_read_i_ops: Vec<ThrottleDevice>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    blkio_device_write_i_ops: Vec<ThrottleDevice>,
    #[serde(rename = "CpuPeriod")]
    cpu_period: i64,
    #[serde(rename = "CpuQuota")]
    cpu_quota: i64,
    #[serde(rename = "CpuRealtimePeriod")]
    cpu_realtime_period: i64,
    #[serde(rename = "CpuRealtimeRuntime")]
    cpu_realtime_runtime: i64,
    #[serde(rename = "CpusetCpus")]
    cpuset_cpus: String,
    #[serde(rename = "CpusetMems")]
    cpuset_mems: String,
    #[serde(rename = "Devices")]
    devices: Vec<DeviceMapping>,
    #[serde(rename = "DeviceCgroupRules")]
    device_cgroup_rules: Vec<String>,
    #[serde(rename = "DiskQuota")]
    disk_quota: i64,
    #[serde(rename = "KernelMemory")]
    kernel_memory: i64,
    #[serde(rename = "MemoryReservation")]
    memory_reservation: i64,
    #[serde(rename = "MemorySwap")]
    memory_swap: i64,
    #[serde(rename = "MemorySwappiness")]
    memory_swappiness: u64,
    #[serde(rename = "NanoCPUs")]
    nano_cp_us: i64,
    #[serde(rename = "OomKillDisable")]
    oom_kill_disable: bool,
    #[serde(rename = "Init")]
    init: bool,
    #[serde(rename = "PidsLimit")]
    pids_limit: i64,
    #[serde(rename = "Ulimits")]
    ulimits: Vec<ContainerCreateHostConfigUlimits>,
    #[serde(rename = "CpuCount")]
    cpu_count: i64,
    #[serde(rename = "CpuPercent")]
    cpu_percent: i64,
    #[serde(rename = "IOMaximumIOps")]
    io_maximum_i_ops: i64,
    #[serde(rename = "IOMaximumBandwidth")]
    io_maximum_bandwidth: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct RestartPolicy {
    #[serde(rename = "Name")]
    name: ContainerCreateHostConfigRestartPolicyName,
    #[serde(rename = "MaximumRetryCount")]
    maximum_retry_count: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Runtime {
    #[serde(rename = "path")]
    path: String,
    #[serde(rename = "runtimeArgs")]
    runtime_args: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Secret {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Version")]
    version: ObjectVersion,
    #[serde(rename = "CreatedAt")]
    created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "UpdatedAt")]
    updated_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "Spec")]
    spec: SecretSpec,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SecretSpec {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Labels")]
    labels: (),
    #[serde(rename = "Data")]
    data: String,
    #[serde(rename = "Driver")]
    driver: Driver,
    #[serde(rename = "Templating")]
    templating: Driver,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Service {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Version")]
    version: ObjectVersion,
    #[serde(rename = "CreatedAt")]
    created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "UpdatedAt")]
    updated_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "Spec")]
    spec: ServiceSpec,
    #[serde(rename = "Endpoint")]
    endpoint: ServiceListEndpoint,
    #[serde(rename = "UpdateStatus")]
    update_status: ServiceListUpdateStatus,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceCreate {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Warning")]
    warning: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListEndpoint {
    #[serde(rename = "Spec")]
    spec: EndpointSpec,
    #[serde(rename = "Ports")]
    ports: Vec<EndpointPortConfig>,
    #[serde(rename = "VirtualIPs")]
    virtual_i_ps: Vec<ServiceListEndpointVirtualIPs>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListEndpointVirtualIPs {
    #[serde(rename = "NetworkID")]
    network_id: String,
    #[serde(rename = "Addr")]
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
        ServiceListSpecEndpointSpecMode::Vip
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
        ServiceListSpecEndpointSpecPortsPublishMode::Ingress
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecMode {
    #[serde(rename = "Replicated")]
    replicated: ServiceListSpecModeReplicated,
    #[serde(rename = "Global")]
    global: (),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecModeReplicated {
    #[serde(rename = "Replicas")]
    replicas: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecRollbackConfig {
    #[serde(rename = "Parallelism")]
    parallelism: i64,
    #[serde(rename = "Delay")]
    delay: i64,
    #[serde(rename = "FailureAction")]
    failure_action: ServiceListSpecRollbackConfigFailureAction,
    #[serde(rename = "Monitor")]
    monitor: i64,
    #[serde(rename = "MaxFailureRatio")]
    max_failure_ratio: f64,
    #[serde(rename = "Order")]
    order: ServiceListSpecUpdateConfigOrder,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ServiceListSpecRollbackConfigFailureAction {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "pause")]
    Pause,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecTaskTemplateContainerSpec {
    #[serde(rename = "Image")]
    image: String,
    #[serde(rename = "Labels")]
    labels: (),
    #[serde(rename = "Command")]
    command: Vec<String>,
    #[serde(rename = "Args")]
    args: Vec<String>,
    #[serde(rename = "Hostname")]
    hostname: String,
    #[serde(rename = "Env")]
    env: Vec<String>,
    #[serde(rename = "Dir")]
    dir: String,
    #[serde(rename = "User")]
    user: String,
    #[serde(rename = "Groups")]
    groups: Vec<String>,
    #[serde(rename = "Privileges")]
    privileges: ServiceListSpecTaskTemplateContainerSpecPrivileges,
    #[serde(rename = "TTY")]
    tty: bool,
    #[serde(rename = "OpenStdin")]
    open_stdin: bool,
    #[serde(rename = "ReadOnly")]
    read_only: bool,
    #[serde(rename = "Mounts")]
    mounts: Vec<Mount>,
    #[serde(rename = "StopSignal")]
    stop_signal: String,
    #[serde(rename = "StopGracePeriod")]
    stop_grace_period: i64,
    #[serde(rename = "HealthCheck")]
    health_check: HealthConfig,
    #[serde(rename = "Hosts")]
    hosts: Vec<String>,
    #[serde(rename = "DNSConfig")]
    dns_config: ServiceListSpecTaskTemplateContainerSpecDNSConfig,
    #[serde(rename = "Secrets")]
    secrets: Vec<ServiceListSpecTaskTemplateContainerSpecSecrets>,
    #[serde(rename = "Configs")]
    configs: Vec<ServiceListSpecTaskTemplateContainerSpecConfigs>,
    #[serde(rename = "Isolation")]
    isolation: ContainerCreateHostConfigIsolation,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecTaskTemplateContainerSpecConfigs {
    #[serde(rename = "File")]
    file: ServiceListSpecTaskTemplateContainerSpecSecretsFile,
    #[serde(rename = "ConfigID")]
    config_id: String,
    #[serde(rename = "ConfigName")]
    config_name: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecTaskTemplateContainerSpecDNSConfig {
    #[serde(rename = "Nameservers")]
    nameservers: Vec<String>,
    #[serde(rename = "Search")]
    search: Vec<String>,
    #[serde(rename = "Options")]
    options: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecTaskTemplateContainerSpecPrivileges {
    #[serde(rename = "CredentialSpec")]
    credential_spec: ServiceListSpecTaskTemplateContainerSpecPrivilegesCredentialSpec,
    #[serde(rename = "SELinuxContext")]
    se_linux_context: ServiceListSpecTaskTemplateContainerSpecPrivilegesSELinuxContext,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecTaskTemplateContainerSpecPrivilegesCredentialSpec {
    #[serde(rename = "File")]
    file: String,
    #[serde(rename = "Registry")]
    registry: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecTaskTemplateContainerSpecPrivilegesSELinuxContext {
    #[serde(rename = "Disable")]
    disable: bool,
    #[serde(rename = "User")]
    user: String,
    #[serde(rename = "Role")]
    role: String,
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Level")]
    level: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecTaskTemplateContainerSpecSecrets {
    #[serde(rename = "File")]
    file: ServiceListSpecTaskTemplateContainerSpecSecretsFile,
    #[serde(rename = "SecretID")]
    secret_id: String,
    #[serde(rename = "SecretName")]
    secret_name: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecTaskTemplateContainerSpecSecretsFile {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "UID")]
    uid: String,
    #[serde(rename = "GID")]
    gid: String,
    #[serde(rename = "Mode")]
    mode: u32,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecTaskTemplateLogDriver {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Options")]
    options: (),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecTaskTemplateNetworkAttachmentSpec {
    #[serde(rename = "ContainerID")]
    container_id: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecTaskTemplateNetworks {
    #[serde(rename = "Target")]
    target: String,
    #[serde(rename = "Aliases")]
    aliases: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecTaskTemplatePlacement {
    #[serde(rename = "Constraints")]
    constraints: Vec<String>,
    #[serde(rename = "Preferences")]
    preferences: Vec<ServiceListSpecTaskTemplatePlacementPreferences>,
    #[serde(rename = "Platforms")]
    platforms: Vec<Platform>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecTaskTemplatePlacementPreferences {
    #[serde(rename = "Spread")]
    spread: ServiceListSpecTaskTemplatePlacementPreferencesSpread,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecTaskTemplatePlacementPreferencesSpread {
    #[serde(rename = "SpreadDescriptor")]
    spread_descriptor: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecTaskTemplatePluginSpec {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Remote")]
    remote: String,
    #[serde(rename = "Disabled")]
    disabled: bool,
    #[serde(rename = "PluginPrivilege")]
    plugin_privilege: Vec<GetPluginPrivileges>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecTaskTemplateResources {
    #[serde(rename = "Limits")]
    limits: ResourceObject,
    #[serde(rename = "Reservation")]
    reservation: ResourceObject,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecTaskTemplateRestartPolicy {
    #[serde(rename = "Condition")]
    condition: ServiceListSpecTaskTemplateRestartPolicyCondition,
    #[serde(rename = "Delay")]
    delay: i64,
    #[serde(rename = "MaxAttempts")]
    max_attempts: i64,
    #[serde(rename = "Window")]
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListSpecUpdateConfig {
    #[serde(rename = "Parallelism")]
    parallelism: i64,
    #[serde(rename = "Delay")]
    delay: i64,
    #[serde(rename = "FailureAction")]
    failure_action: ServiceListSpecUpdateConfigFailureAction,
    #[serde(rename = "Monitor")]
    monitor: i64,
    #[serde(rename = "MaxFailureRatio")]
    max_failure_ratio: f64,
    #[serde(rename = "Order")]
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceListUpdateStatus {
    #[serde(rename = "State")]
    state: ServiceListUpdateStatusState,
    #[serde(rename = "StartedAt")]
    started_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "CompletedAt")]
    completed_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "Message")]
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceSpec {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Labels")]
    labels: (),
    #[serde(rename = "TaskTemplate")]
    task_template: TaskSpec,
    #[serde(rename = "Mode")]
    mode: ServiceListSpecMode,
    #[serde(rename = "UpdateConfig")]
    update_config: ServiceListSpecUpdateConfig,
    #[serde(rename = "RollbackConfig")]
    rollback_config: ServiceListSpecRollbackConfig,
    #[serde(rename = "Networks")]
    networks: Vec<ServiceListSpecTaskTemplateNetworks>,
    #[serde(rename = "EndpointSpec")]
    endpoint_spec: EndpointSpec,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ServiceUpdateResponse {
    #[serde(rename = "Warnings")]
    warnings: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Swarm {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Version")]
    version: ObjectVersion,
    #[serde(rename = "CreatedAt")]
    created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "UpdatedAt")]
    updated_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "Spec")]
    spec: SwarmSpec,
    #[serde(rename = "TLSInfo")]
    tls_info: TLSInfo,
    #[serde(rename = "RootRotationInProgress")]
    root_rotation_in_progress: bool,
    #[serde(rename = "JoinTokens")]
    join_tokens: JoinTokens,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SwarmInfo {
    #[serde(rename = "NodeID")]
    node_id: String,
    #[serde(rename = "NodeAddr")]
    node_addr: String,
    #[serde(rename = "LocalNodeState")]
    local_node_state: LocalNodeState,
    #[serde(rename = "ControlAvailable")]
    control_available: bool,
    #[serde(rename = "Error")]
    error: String,
    #[serde(rename = "RemoteManagers")]
    remote_managers: Vec<PeerNode>,
    #[serde(rename = "Nodes")]
    nodes: i64,
    #[serde(rename = "Managers")]
    managers: i64,
    #[serde(rename = "Cluster")]
    cluster: ClusterInfo,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SwarmInit {
    #[serde(rename = "ListenAddr")]
    listen_addr: String,
    #[serde(rename = "AdvertiseAddr")]
    advertise_addr: String,
    #[serde(rename = "DataPathAddr")]
    data_path_addr: String,
    #[serde(rename = "ForceNewCluster")]
    force_new_cluster: bool,
    #[serde(rename = "Spec")]
    spec: SwarmSpec,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SwarmJoin {
    #[serde(rename = "ListenAddr")]
    listen_addr: String,
    #[serde(rename = "AdvertiseAddr")]
    advertise_addr: String,
    #[serde(rename = "DataPathAddr")]
    data_path_addr: String,
    #[serde(rename = "RemoteAddrs")]
    remote_addrs: String,
    #[serde(rename = "JoinToken")]
    join_token: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SwarmSpec {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Labels")]
    labels: (),
    #[serde(rename = "Orchestration")]
    orchestration: SystemInfoSwarmClusterSpecOrchestration,
    #[serde(rename = "Raft")]
    raft: SystemInfoSwarmClusterSpecRaft,
    #[serde(rename = "Dispatcher")]
    dispatcher: SystemInfoSwarmClusterSpecDispatcher,
    #[serde(rename = "CAConfig")]
    ca_config: SystemInfoSwarmClusterSpecCAConfig,
    #[serde(rename = "EncryptionConfig")]
    encryption_config: SystemInfoSwarmClusterSpecEncryptionConfig,
    #[serde(rename = "TaskDefaults")]
    task_defaults: SystemInfoSwarmClusterSpecTaskDefaults,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SwarmUnlockkey {
    #[serde(rename = "UnlockKey")]
    unlock_key: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemAuth {
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "IdentityToken")]
    identity_token: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemDataUsage {
    #[serde(rename = "LayersSize")]
    layers_size: i64,
    #[serde(rename = "Images")]
    images: Vec<ImageSummary>,
    #[serde(rename = "Containers")]
    containers: Vec<Vec<ContainerList>>,
    #[serde(rename = "Volumes")]
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
        SystemDataUsageVolumesScope::Local
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemDataUsageVolumesUsageData {
    #[serde(rename = "Size")]
    size: i64,
    #[serde(rename = "RefCount")]
    ref_count: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemEvents {
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Action")]
    action: String,
    #[serde(rename = "Actor")]
    actor: SystemEventsActor,
    #[serde(rename = "time")]
    time: i64,
    #[serde(rename = "timeNano")]
    time_nano: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemEventsActor {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Attributes")]
    attributes: (),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemInfo {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Containers")]
    containers: i64,
    #[serde(rename = "ContainersRunning")]
    containers_running: i64,
    #[serde(rename = "ContainersPaused")]
    containers_paused: i64,
    #[serde(rename = "ContainersStopped")]
    containers_stopped: i64,
    #[serde(rename = "Images")]
    images: i64,
    #[serde(rename = "Driver")]
    driver: String,
    #[serde(rename = "DriverStatus")]
    driver_status: Vec<Vec<String>>,
    #[serde(rename = "DockerRootDir")]
    docker_root_dir: String,
    #[serde(rename = "SystemStatus")]
    system_status: Vec<Vec<String>>,
    #[serde(rename = "Plugins")]
    plugins: PluginsInfo,
    #[serde(rename = "MemoryLimit")]
    memory_limit: bool,
    #[serde(rename = "SwapLimit")]
    swap_limit: bool,
    #[serde(rename = "KernelMemory")]
    kernel_memory: bool,
    #[serde(rename = "CpuCfsPeriod")]
    cpu_cfs_period: bool,
    #[serde(rename = "CpuCfsQuota")]
    cpu_cfs_quota: bool,
    #[serde(rename = "CPUShares")]
    cpu_shares: bool,
    #[serde(rename = "CPUSet")]
    cpu_set: bool,
    #[serde(rename = "OomKillDisable")]
    oom_kill_disable: bool,
    #[serde(rename = "IPv4Forwarding")]
    i_pv4_forwarding: bool,
    #[serde(rename = "BridgeNfIptables")]
    bridge_nf_iptables: bool,
    #[serde(rename = "BridgeNfIp6tables")]
    bridge_nf_ip6tables: bool,
    #[serde(rename = "Debug")]
    debug: bool,
    #[serde(rename = "NFd")]
    n_fd: i64,
    #[serde(rename = "NGoroutines")]
    n_goroutines: i64,
    #[serde(rename = "SystemTime")]
    system_time: String,
    #[serde(rename = "LoggingDriver")]
    logging_driver: String,
    #[serde(rename = "CgroupDriver")]
    cgroup_driver: SystemInfoCgroupDriver,
    #[serde(rename = "NEventsListener")]
    n_events_listener: i64,
    #[serde(rename = "KernelVersion")]
    kernel_version: String,
    #[serde(rename = "OperatingSystem")]
    operating_system: String,
    #[serde(rename = "OSType")]
    os_type: String,
    #[serde(rename = "Architecture")]
    architecture: String,
    #[serde(rename = "NCPU")]
    ncpu: i64,
    #[serde(rename = "MemTotal")]
    mem_total: i64,
    #[serde(rename = "IndexServerAddress")]
    index_server_address: String,
    #[serde(rename = "RegistryConfig")]
    registry_config: RegistryServiceConfig,
    #[serde(rename = "GenericResources")]
    generic_resources: Vec<SystemInfoGenericResources>,
    #[serde(rename = "HttpProxy")]
    http_proxy: String,
    #[serde(rename = "HttpsProxy")]
    https_proxy: String,
    #[serde(rename = "NoProxy")]
    no_proxy: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Labels")]
    labels: Vec<String>,
    #[serde(rename = "ExperimentalBuild")]
    experimental_build: bool,
    #[serde(rename = "ServerVersion")]
    server_version: String,
    #[serde(rename = "ClusterStore")]
    cluster_store: String,
    #[serde(rename = "ClusterAdvertise")]
    cluster_advertise: String,
    #[serde(rename = "Runtimes")]
    runtimes: (),
    #[serde(rename = "DefaultRuntime")]
    default_runtime: String,
    #[serde(rename = "Swarm")]
    swarm: SwarmInfo,
    #[serde(rename = "LiveRestoreEnabled")]
    live_restore_enabled: bool,
    #[serde(rename = "Isolation")]
    isolation: SystemInfoIsolation,
    #[serde(rename = "InitBinary")]
    init_binary: String,
    #[serde(rename = "ContainerdCommit")]
    containerd_commit: Commit,
    #[serde(rename = "RuncCommit")]
    runc_commit: Commit,
    #[serde(rename = "InitCommit")]
    init_commit: Commit,
    #[serde(rename = "SecurityOptions")]
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
        SystemInfoCgroupDriver::Cgroupfs
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemInfoGenericResources {
    #[serde(rename = "NamedResourceSpec")]
    named_resource_spec: SystemInfoGenericResourcesNamedResourceSpec,
    #[serde(rename = "DiscreteResourceSpec")]
    discrete_resource_spec: SystemInfoGenericResourcesDiscreteResourceSpec,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemInfoGenericResourcesDiscreteResourceSpec {
    #[serde(rename = "Kind")]
    kind: String,
    #[serde(rename = "Value")]
    value: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemInfoGenericResourcesNamedResourceSpec {
    #[serde(rename = "Kind")]
    kind: String,
    #[serde(rename = "Value")]
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
        SystemInfoIsolation::Default
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemInfoSwarmClusterSpecCAConfig {
    #[serde(rename = "NodeCertExpiry")]
    node_cert_expiry: i64,
    #[serde(rename = "ExternalCAs")]
    external_c_as: Vec<SystemInfoSwarmClusterSpecCAConfigExternalCAs>,
    #[serde(rename = "SigningCACert")]
    signing_ca_cert: String,
    #[serde(rename = "SigningCAKey")]
    signing_ca_key: String,
    #[serde(rename = "ForceRotate")]
    force_rotate: u64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemInfoSwarmClusterSpecCAConfigExternalCAs {
    #[serde(rename = "Protocol")]
    protocol: SystemInfoSwarmClusterSpecCAConfigExternalCAsProtocol,
    #[serde(rename = "URL")]
    url: String,
    #[serde(rename = "Options")]
    options: (),
    #[serde(rename = "CACert")]
    ca_cert: String,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum SystemInfoSwarmClusterSpecCAConfigExternalCAsProtocol {
    #[serde(rename = "cfssl")]
    Cfssl,
}

impl Default for SystemInfoSwarmClusterSpecCAConfigExternalCAsProtocol {
    fn default() -> Self {
        SystemInfoSwarmClusterSpecCAConfigExternalCAsProtocol::Cfssl
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemInfoSwarmClusterSpecDispatcher {
    #[serde(rename = "HeartbeatPeriod")]
    heartbeat_period: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemInfoSwarmClusterSpecEncryptionConfig {
    #[serde(rename = "AutoLockManagers")]
    auto_lock_managers: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemInfoSwarmClusterSpecOrchestration {
    #[serde(rename = "TaskHistoryRetentionLimit")]
    task_history_retention_limit: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemInfoSwarmClusterSpecRaft {
    #[serde(rename = "SnapshotInterval")]
    snapshot_interval: u64,
    #[serde(rename = "KeepOldSnapshots")]
    keep_old_snapshots: u64,
    #[serde(rename = "LogEntriesForSlowFollowers")]
    log_entries_for_slow_followers: u64,
    #[serde(rename = "ElectionTick")]
    election_tick: i64,
    #[serde(rename = "HeartbeatTick")]
    heartbeat_tick: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemInfoSwarmClusterSpecTaskDefaults {
    #[serde(rename = "LogDriver")]
    log_driver: SystemInfoSwarmClusterSpecTaskDefaultsLogDriver,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemInfoSwarmClusterSpecTaskDefaultsLogDriver {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Options")]
    options: (),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemVersion {
    #[serde(rename = "Platform")]
    platform: SystemVersionPlatform,
    #[serde(rename = "Components")]
    components: Vec<SystemVersionComponents>,
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "ApiVersion")]
    api_version: String,
    #[serde(rename = "MinAPIVersion")]
    min_api_version: String,
    #[serde(rename = "GitCommit")]
    git_commit: String,
    #[serde(rename = "GoVersion")]
    go_version: String,
    #[serde(rename = "Os")]
    os: String,
    #[serde(rename = "Arch")]
    arch: String,
    #[serde(rename = "KernelVersion")]
    kernel_version: String,
    #[serde(rename = "Experimental")]
    experimental: bool,
    #[serde(rename = "BuildTime")]
    build_time: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemVersionComponents {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "Details")]
    details: (),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SystemVersionPlatform {
    #[serde(rename = "Name")]
    name: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct TLSInfo {
    #[serde(rename = "TrustRoot")]
    trust_root: String,
    #[serde(rename = "CertIssuerSubject")]
    cert_issuer_subject: String,
    #[serde(rename = "CertIssuerPublicKey")]
    cert_issuer_public_key: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Task {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Version")]
    version: ObjectVersion,
    #[serde(rename = "CreatedAt")]
    created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "UpdatedAt")]
    updated_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Labels")]
    labels: (),
    #[serde(rename = "Spec")]
    spec: TaskSpec,
    #[serde(rename = "ServiceID")]
    service_id: String,
    #[serde(rename = "Slot")]
    slot: i64,
    #[serde(rename = "NodeID")]
    node_id: String,
    #[serde(rename = "AssignedGenericResources")]
    assigned_generic_resources: Vec<SystemInfoGenericResources>,
    #[serde(rename = "Status")]
    status: TaskListStatus,
    #[serde(rename = "DesiredState")]
    desired_state: TaskState,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct TaskListStatus {
    #[serde(rename = "Timestamp")]
    timestamp: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "State")]
    state: TaskState,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Err")]
    err: String,
    #[serde(rename = "ContainerStatus")]
    container_status: TaskListStatusContainerStatus,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct TaskListStatusContainerStatus {
    #[serde(rename = "ContainerID")]
    container_id: String,
    #[serde(rename = "PID")]
    pid: i64,
    #[serde(rename = "ExitCode")]
    exit_code: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct TaskSpec {
    #[serde(rename = "PluginSpec")]
    plugin_spec: ServiceListSpecTaskTemplatePluginSpec,
    #[serde(rename = "ContainerSpec")]
    container_spec: ServiceListSpecTaskTemplateContainerSpec,
    #[serde(rename = "NetworkAttachmentSpec")]
    network_attachment_spec: ServiceListSpecTaskTemplateNetworkAttachmentSpec,
    #[serde(rename = "Resources")]
    resources: ServiceListSpecTaskTemplateResources,
    #[serde(rename = "RestartPolicy")]
    restart_policy: ServiceListSpecTaskTemplateRestartPolicy,
    #[serde(rename = "Placement")]
    placement: ServiceListSpecTaskTemplatePlacement,
    #[serde(rename = "ForceUpdate")]
    force_update: i64,
    #[serde(rename = "Runtime")]
    runtime: String,
    #[serde(rename = "Networks")]
    networks: Vec<ServiceListSpecTaskTemplateNetworks>,
    #[serde(rename = "LogDriver")]
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ThrottleDevice {
    #[serde(rename = "Path")]
    path: String,
    #[serde(rename = "Rate")]
    rate: u64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Volume {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Driver")]
    driver: String,
    #[serde(rename = "Mountpoint")]
    mountpoint: String,
    #[serde(rename = "CreatedAt")]
    created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "Status")]
    status: (),
    #[serde(rename = "Labels")]
    labels: (),
    #[serde(rename = "Scope")]
    scope: SystemDataUsageVolumesScope,
    #[serde(rename = "Options")]
    options: (),
    #[serde(rename = "UsageData")]
    usage_data: SystemDataUsageVolumesUsageData,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct VolumeCreate {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Driver")]
    driver: String,
    #[serde(rename = "DriverOpts")]
    driver_opts: (),
    #[serde(rename = "Labels")]
    labels: (),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct VolumeList {
    #[serde(rename = "Volumes")]
    volumes: Vec<Volume>,
    #[serde(rename = "Warnings")]
    warnings: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct VolumePrune {
    #[serde(rename = "VolumesDeleted")]
    volumes_deleted: Vec<String>,
    #[serde(rename = "SpaceReclaimed")]
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
// HEAD /containers/{id}/archive
//   HEAD /containers/{id}/archive
//   HEAD /containers/{id}/archive
// PUT /containers/{id}/archive
//   PUT /containers/{id}/archive
//   PUT /containers/{id}/archive
//   PUT /containers/{id}/archive
//   PUT /containers/{id}/archive
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
// GET /networks/{id}
//   GET /networks/{id}
//   GET /networks/{id}
//   GET /networks/{id}
// DELETE /networks/{id}
//   DELETE /networks/{id}
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
// GET /nodes/{id}
//   GET /nodes/{id}
// DELETE /nodes/{id}
//   DELETE /nodes/{id}
//   DELETE /nodes/{id}
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
// GET /services/{id}
//   GET /services/{id}
//   GET /services/{id}
// DELETE /services/{id}
//   DELETE /services/{id}
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
// GET /configs/{id}
//   GET /configs/{id}
// DELETE /configs/{id}
//   DELETE /configs/{id}
// POST /configs/{id}/update
//   POST /configs/{id}/update
//   POST /configs/{id}/update
//   POST /configs/{id}/update
// GET /distribution/{name}/json
//   GET /distribution/{name}/json
// POST /session
