use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum HardwareConstraintVariant {
    HardwareConstraint(Box<HardwareConstraint>),
    MinimumCoreHardware(Box<MinimumCore>),
    MaximumCoreHardware(Box<MaximumCore>),
    MinimumRamHardware(Box<MinimumRam>),
    MaximumRamHardware(Box<MaximumRam>),
    SpecificHardware(Box<Specific>),
    MinimumRamCoreRatioHardware(Box<MinimumRamCoreRatio>),
    MaximumRamCoreRatioHardware(Box<MaximumRamCoreRatio>),
    SsdHardware(Box<Ssd>),
    NoSsdHardware(Box<NoSsd>),
    NoGpuHardware(Box<NoGpu>),
    GpuHardware(Box<Gpu>),
    CpuModelHardware(Box<CpuModel>),
}

/// HardwareConstraint : Base hardware constraint
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "discriminator")]
pub enum HardwareConstraint {
    #[serde(rename = "CpuModelHardwareConstraint")]
    CpuModel {},
    #[serde(rename = "GpuHardwareConstraint")]
    Gpu {},
    #[serde(rename = "MaximumCoreHardwareConstraint")]
    MaximumCore {},
    #[serde(rename = "MaximumRamCoreRatioHardwareConstraint")]
    MaximumRamCoreRatio {},
    #[serde(rename = "MaximumRamHardwareConstraint")]
    MaximumRam {},
    #[serde(rename = "MinimumCoreHardwareConstraint")]
    MinimumCore {},
    #[serde(rename = "MinimumRamCoreRatioHardwareConstraint")]
    MinimumRamCoreRatio {},
    #[serde(rename = "MinimumRamHardwareConstraint")]
    MinimumRam {},
    #[serde(rename = "NoGpuHardwareConstraint")]
    NoGpu {},
    #[serde(rename = "NoSSDHardwareConstraint")]
    NoSsd {},
    #[serde(rename = "SSDHardwareConstraint")]
    Ssd {},
    #[serde(rename = "SpecificHardwareConstraint")]
    Specific {},
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "discriminator")]
pub struct HardwareConstraintResponse {
    pub data: Option<Vec<HardwareConstraintVariant>>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    pub total: Option<u32>,
}

/// CpuModel : Constraint for CPU model
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuModel {
    /// Type of hardware constraint
    pub discriminator: Option<String>,
    /// Cpu model required
    pub cpu_model: Option<String>,
}

/// Gpu : Constraint for a hardware with a GPU
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Gpu {
    /// Type of hardware constraint
    pub discriminator: Option<String>,
}

/// MaximumCore : Constraint for maximum cores
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaximumCore {
    /// Type of hardware constraint
    #[serde(rename = "discriminator")]
    pub discriminator: Option<String>,
    /// Maximum number of cores required
    #[serde(rename = "coreCount")]
    pub core_count: Option<i32>,
}

/// MaximumRamCoreRatio : Constraint for maximum ratio RAM/cores
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaximumRamCoreRatio {
    /// Type of hardware constraint
    pub discriminator: Option<String>,
    /// Maximum memory per core ratio required (in GB)
    pub maximum_memory_gb_core_ratio: Option<f64>,
}

/// MaximumRam : Constraint for maximum RAM
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaximumRam {
    /// Type of hardware constraint
    pub discriminator: Option<String>,
    /// Maximum memory size required (in GB)
    pub maximum_memory_mb: Option<f64>,
}

/// MinimumCore : Constraint for minimum cores
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MinimumCore {
    /// Type of hardware constraint
    pub discriminator: Option<String>,
    /// Minimum number of cores required
    pub core_count: Option<i32>,
}

/// MinimumRamCoreRatio : Constraint for minimum ratio RAM/cores
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MinimumRamCoreRatio {
    /// Type of hardware constraint
    pub discriminator: Option<String>,
    /// Minimum memory per core ratio required (in GB)
    pub minimum_memory_gb_core_ratio: Option<f64>,
}

/// MinimumRam : Constraint for minimum RAM
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MinimumRam {
    /// Type of hardware constraint
    pub discriminator: Option<String>,
    /// Minimum memory size required (in GB)
    pub minimum_memory_mb: Option<f64>,
}

/// NoGpu : Constraint for a hardware with a GPU
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoGpu {
    /// Type of hardware constraint
    pub discriminator: Option<String>,
}

/// NoSsd : Constraint for a hardware without a SSD
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoSsd {
    /// Type of hardware constraint
    pub discriminator: Option<String>,
}

/// Ssd : Constraint for a hardware with a SSD
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ssd {
    /// Type of hardware constraint
    pub discriminator: Option<String>,
}

/// Specific : Constraint for a specific hardware
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Specific {
    /// Type of hardware constraint
    pub discriminator: Option<String>,
    /// Key name of the required specific hardware.  All the hardware containing the value in their name will be accepted.
    pub specification_key: Option<String>,
}

impl Default for HardwareConstraintVariant {
    fn default() -> Self {
        Self::HardwareConstraint(Default::default())
    }
}

impl Default for HardwareConstraint {
    fn default() -> Self {
        Self::CpuModel {}
    }
}
