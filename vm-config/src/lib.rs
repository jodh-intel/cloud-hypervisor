use net_util::MacAddr;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::path::PathBuf;
use virtio_devices::RateLimiterConfig;

pub const MAX_NUM_PCI_SEGMENTS: u16 = 16;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct BalloonConfig {
    pub size: u64,
    /// Option to deflate the balloon in case the guest is out of memory.
    //#[serde(default)]
    pub deflate_on_oom: bool,
    /// Option to enable free page reporting from the guest.
    //#[serde(default)]
    pub free_page_reporting: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct CmdlineConfig {
    pub args: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ConsoleConfig {
    //#[serde(default = "default_consoleconfig_file")]
    pub file: Option<PathBuf>,
    pub mode: ConsoleOutputMode,
    //#[serde(default)]
    pub iommu: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ConsoleOutputMode {
    Off,
    Pty,
    Tty,
    File,
    Null,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CpuAffinity {
    pub vcpu: u8,
    pub host_cpus: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CpusConfig {
    pub boot_vcpus: u8,
    pub max_vcpus: u8,
    //#[serde(default)]
    pub topology: Option<CpuTopology>,
    //#[serde(default)]
    pub kvm_hyperv: bool,
    pub max_phys_bits: u8,
    //#[serde(default)]
    pub affinity: Option<Vec<CpuAffinity>>,
    //#[serde(default)]
    pub features: CpuFeatures,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct CpuFeatures {
    #[cfg(target_arch = "x86_64")]
    //#[serde(default)]
    pub amx: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CpuTopology {
    pub threads_per_core: u8,
    pub cores_per_die: u8,
    pub dies_per_package: u8,
    pub packages: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct DeviceConfig {
    pub path: PathBuf,
    //#[serde(default)]
    pub iommu: bool,
    //#[serde(default)]
    pub id: Option<String>,
    //#[serde(default)]
    pub pci_segment: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DiskConfig {
    pub path: Option<PathBuf>,
    //#[serde(default)]
    pub readonly: bool,
    //#[serde(default)]
    pub direct: bool,
    //#[serde(default)]
    pub iommu: bool,
    //#[serde(default = "default_diskconfig_num_queues")]
    pub num_queues: usize,
    //#[serde(default = "default_diskconfig_queue_size")]
    pub queue_size: u16,
    //#[serde(default)]
    pub vhost_user: bool,
    pub vhost_socket: Option<String>,
    //#[serde(default)]
    pub rate_limiter_config: Option<RateLimiterConfig>,
    //#[serde(default)]
    pub id: Option<String>,
    // For testing use only. Not exposed in API.
    //#[serde(default)]
    pub disable_io_uring: bool,
    //#[serde(default)]
    pub pci_segment: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FsConfig {
    pub tag: String,
    pub socket: PathBuf,
    //#[serde(default = "default_fsconfig_num_queues")]
    pub num_queues: usize,
    //#[serde(default = "default_fsconfig_queue_size")]
    pub queue_size: u16,
    //#[serde(default)]
    pub id: Option<String>,
    //#[serde(default)]
    pub pci_segment: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum HotplugMethod {
    Acpi,
    VirtioMem,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct InitramfsConfig {
    pub path: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct KernelConfig {
    pub path: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct MemoryConfig {
    pub size: u64,
    //#[serde(default)]
    pub mergeable: bool,
    //#[serde(default)]
    pub hotplug_method: HotplugMethod,
    //#[serde(default)]
    pub hotplug_size: Option<u64>,
    //#[serde(default)]
    pub hotplugged_size: Option<u64>,
    //#[serde(default)]
    pub shared: bool,
    //#[serde(default)]
    pub hugepages: bool,
    //#[serde(default)]
    pub hugepage_size: Option<u64>,
    //#[serde(default)]
    pub prefault: bool,
    //#[serde(default)]
    pub zones: Option<Vec<MemoryZoneConfig>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct MemoryZoneConfig {
    pub id: String,
    pub size: u64,
    //#[serde(default)]
    pub file: Option<PathBuf>,
    //#[serde(default)]
    pub shared: bool,
    //#[serde(default)]
    pub hugepages: bool,
    //#[serde(default)]
    pub hugepage_size: Option<u64>,
    //#[serde(default)]
    pub host_numa_node: Option<u32>,
    //#[serde(default)]
    pub hotplug_size: Option<u64>,
    //#[serde(default)]
    pub hotplugged_size: Option<u64>,
    //#[serde(default)]
    pub prefault: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct NetConfig {
    //#[serde(default = "default_netconfig_tap")]
    pub tap: Option<String>,
    //#[serde(default = "default_netconfig_ip")]
    pub ip: Ipv4Addr,
    //#[serde(default = "default_netconfig_mask")]
    pub mask: Ipv4Addr,
    //#[serde(default = "default_netconfig_mac")]
    pub mac: MacAddr,
    //#[serde(default)]
    pub host_mac: Option<MacAddr>,
    //#[serde(default)]
    pub mtu: Option<u16>,
    //#[serde(default)]
    pub iommu: bool,
    //#[serde(default = "default_netconfig_num_queues")]
    pub num_queues: usize,
    //#[serde(default = "default_netconfig_queue_size")]
    pub queue_size: u16,
    //#[serde(default)]
    pub vhost_user: bool,
    pub vhost_socket: Option<String>,
    //#[serde(default)]
    pub vhost_mode: VhostMode,
    //#[serde(default)]
    pub id: Option<String>,
    //#[serde(default)]
    pub fds: Option<Vec<i32>>,
    //#[serde(default)]
    pub rate_limiter_config: Option<RateLimiterConfig>,
    //#[serde(default)]
    pub pci_segment: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct NumaConfig {
    //#[serde(default)]
    pub guest_numa_id: u32,
    //#[serde(default)]
    pub cpus: Option<Vec<u8>>,
    //#[serde(default)]
    pub distances: Option<Vec<NumaDistance>>,
    //#[serde(default)]
    pub memory_zones: Option<Vec<String>>,
    #[cfg(target_arch = "x86_64")]
    //#[serde(default)]
    pub sgx_epc_sections: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct NumaDistance {
    //#[serde(default)]
    pub destination: u32,
    //#[serde(default)]
    pub distance: u8,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PayloadConfig {
    //#[serde(default)]
    pub firmware: Option<PathBuf>,
    //#[serde(default)]
    pub kernel: Option<PathBuf>,
    //#[serde(default)]
    pub cmdline: Option<String>,
    //#[serde(default)]
    pub initramfs: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct PlatformConfig {
    //#[serde(default = "default_platformconfig_num_pci_segments")]
    pub num_pci_segments: u16,
    //#[serde(default)]
    pub iommu_segments: Option<Vec<u16>>,
    //#[serde(default)]
    pub serial_number: Option<String>,
    //#[serde(default)]
    pub uuid: Option<String>,
    //#[serde(default)]
    pub oem_strings: Option<Vec<String>>,
    #[cfg(feature = "tdx")]
    //#[serde(default)]
    pub tdx: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct PmemConfig {
    pub file: PathBuf,
    //#[serde(default)]
    pub size: Option<u64>,
    //#[serde(default)]
    pub iommu: bool,
    //#[serde(default)]
    pub discard_writes: bool,
    //#[serde(default)]
    pub id: Option<String>,
    //#[serde(default)]
    pub pci_segment: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RngConfig {
    pub src: PathBuf,
    //#[serde(default)]
    pub iommu: bool,
}

#[cfg(target_arch = "x86_64")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct SgxEpcConfig {
    pub id: String,
    //#[serde(default)]
    pub size: u64,
    //#[serde(default)]
    pub prefault: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct UserDeviceConfig {
    pub socket: PathBuf,
    //#[serde(default)]
    pub id: Option<String>,
    //#[serde(default)]
    pub pci_segment: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct VdpaConfig {
    pub path: PathBuf,
    //#[serde(default = "default_vdpaconfig_num_queues")]
    pub num_queues: usize,
    //#[serde(default)]
    pub iommu: bool,
    //#[serde(default)]
    pub id: Option<String>,
    //#[serde(default)]
    pub pci_segment: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum VhostMode {
    Client,
    Server,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct VmConfig {
    //#[serde(default)]
    pub cpus: CpusConfig,
    //#[serde(default)]
    pub memory: MemoryConfig,
    pub kernel: Option<KernelConfig>,
    //#[serde(default)]
    pub initramfs: Option<InitramfsConfig>,
    //#[serde(default)]
    pub cmdline: CmdlineConfig,
    //#[serde(default)]
    pub payload: Option<PayloadConfig>,
    pub disks: Option<Vec<DiskConfig>>,
    pub net: Option<Vec<NetConfig>>,
    //#[serde(default)]
    pub rng: RngConfig,
    pub balloon: Option<BalloonConfig>,
    pub fs: Option<Vec<FsConfig>>,
    pub pmem: Option<Vec<PmemConfig>>,
    //#[serde(default = "ConsoleConfig::default_serial")]
    pub serial: ConsoleConfig,
    //#[serde(default = "ConsoleConfig::default_console")]
    pub console: ConsoleConfig,
    pub devices: Option<Vec<DeviceConfig>>,
    pub user_devices: Option<Vec<UserDeviceConfig>>,
    pub vdpa: Option<Vec<VdpaConfig>>,
    pub vsock: Option<VsockConfig>,
    //#[serde(default)]
    pub iommu: bool,
    #[cfg(target_arch = "x86_64")]
    pub sgx_epc: Option<Vec<SgxEpcConfig>>,
    pub numa: Option<Vec<NumaConfig>>,
    //#[serde(default)]
    pub watchdog: bool,
    #[cfg(feature = "guest_debug")]
    pub gdb: bool,
    pub platform: Option<PlatformConfig>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct VsockConfig {
    pub cid: u64,
    pub socket: PathBuf,
    //#[serde(default)]
    pub iommu: bool,
    //#[serde(default)]
    pub id: Option<String>,
    //#[serde(default)]
    pub pci_segment: u16,
}
