#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg_gpmc_revision: CfgGpmcRevision,
    _reserved1: [u8; 0x0c],
    cfg_gpmc_sysconfig: CfgGpmcSysconfig,
    cfg_gpmc_sysstatus: CfgGpmcSysstatus,
    cfg_gpmc_irqstatus: CfgGpmcIrqstatus,
    cfg_gpmc_irqenable: CfgGpmcIrqenable,
    _reserved5: [u8; 0x20],
    cfg_gpmc_timeout_control: CfgGpmcTimeoutControl,
    cfg_gpmc_err_address: CfgGpmcErrAddress,
    cfg_gpmc_err_type: CfgGpmcErrType,
    _reserved8: [u8; 0x04],
    cfg_gpmc_config: CfgGpmcConfig,
    cfg_gpmc_status: CfgGpmcStatus,
    _reserved10: [u8; 0x08],
    cfg_gpmc_config1: CfgGpmcConfig1,
    cfg_gpmc_config2: CfgGpmcConfig2,
    cfg_gpmc_config3: CfgGpmcConfig3,
    cfg_gpmc_config4: CfgGpmcConfig4,
    cfg_gpmc_config5: CfgGpmcConfig5,
    cfg_gpmc_config6: CfgGpmcConfig6,
    cfg_gpmc_config7: CfgGpmcConfig7,
    cfg_gpmc_nand_command: CfgGpmcNandCommand,
    cfg_gpmc_nand_address: CfgGpmcNandAddress,
    cfg_gpmc_nand_data: CfgGpmcNandData,
    _reserved20: [u8; 0x0158],
    cfg_gpmc_prefetch_config1: CfgGpmcPrefetchConfig1,
    cfg_gpmc_prefetch_config2: CfgGpmcPrefetchConfig2,
    _reserved22: [u8; 0x04],
    cfg_gpmc_prefetch_control: CfgGpmcPrefetchControl,
    cfg_gpmc_prefetch_status: CfgGpmcPrefetchStatus,
    cfg_gpmc_ecc_config: CfgGpmcEccConfig,
    cfg_gpmc_ecc_control: CfgGpmcEccControl,
    cfg_gpmc_ecc_size_config: CfgGpmcEccSizeConfig,
    cfg_gpmc_ecc_result: CfgGpmcEccResult,
    _reserved28: [u8; 0x3c],
    cfg_gpmc_bch_result_0: CfgGpmcBchResult0,
    cfg_gpmc_bch_result_1: CfgGpmcBchResult1,
    cfg_gpmc_bch_result_2: CfgGpmcBchResult2,
    cfg_gpmc_bch_result_3: CfgGpmcBchResult3,
    _reserved32: [u8; 0x80],
    cfg_gpmc_bch_swdata: CfgGpmcBchSwdata,
    _reserved33: [u8; 0x2c],
    cfg_gpmc_bch_result_4: CfgGpmcBchResult4,
    cfg_gpmc_bch_result_5: CfgGpmcBchResult5,
    cfg_gpmc_bch_result_6: CfgGpmcBchResult6,
}
impl RegisterBlock {
    #[doc = "0x00 - This register contains the IP revision code"]
    #[inline(always)]
    pub const fn cfg_gpmc_revision(&self) -> &CfgGpmcRevision {
        &self.cfg_gpmc_revision
    }
    #[doc = "0x10 - This register controls the various parameters of the OCP interface"]
    #[inline(always)]
    pub const fn cfg_gpmc_sysconfig(&self) -> &CfgGpmcSysconfig {
        &self.cfg_gpmc_sysconfig
    }
    #[doc = "0x14 - This register provides status information about the module, excluding the interrupt status information"]
    #[inline(always)]
    pub const fn cfg_gpmc_sysstatus(&self) -> &CfgGpmcSysstatus {
        &self.cfg_gpmc_sysstatus
    }
    #[doc = "0x18 - This interrupt status register regroups all the status of the module internal events that can generate an interrupt."]
    #[inline(always)]
    pub const fn cfg_gpmc_irqstatus(&self) -> &CfgGpmcIrqstatus {
        &self.cfg_gpmc_irqstatus
    }
    #[doc = "0x1c - The interrupt enable register allows to mask/unmask the module internal sources of interrupt, on a event-by-event basis."]
    #[inline(always)]
    pub const fn cfg_gpmc_irqenable(&self) -> &CfgGpmcIrqenable {
        &self.cfg_gpmc_irqenable
    }
    #[doc = "0x40 - The GPMC_TIMEOUT_CONTROL register allows the user to set the start value of the timeout counter"]
    #[inline(always)]
    pub const fn cfg_gpmc_timeout_control(&self) -> &CfgGpmcTimeoutControl {
        &self.cfg_gpmc_timeout_control
    }
    #[doc = "0x44 - The GPMC_ERR_ADDRESS register stores the address of the illegal access when an error occurs"]
    #[inline(always)]
    pub const fn cfg_gpmc_err_address(&self) -> &CfgGpmcErrAddress {
        &self.cfg_gpmc_err_address
    }
    #[doc = "0x48 - The GPMC_ERR_TYPE register stores the type of error when an error occurs"]
    #[inline(always)]
    pub const fn cfg_gpmc_err_type(&self) -> &CfgGpmcErrType {
        &self.cfg_gpmc_err_type
    }
    #[doc = "0x50 - The configuration register allows global configuration of the GPMC"]
    #[inline(always)]
    pub const fn cfg_gpmc_config(&self) -> &CfgGpmcConfig {
        &self.cfg_gpmc_config
    }
    #[doc = "0x54 - The status register provides global status bits of the GPMC"]
    #[inline(always)]
    pub const fn cfg_gpmc_status(&self) -> &CfgGpmcStatus {
        &self.cfg_gpmc_status
    }
    #[doc = "0x60 - The configuration 1 register sets signal control parameters per chip select"]
    #[inline(always)]
    pub const fn cfg_gpmc_config1(&self) -> &CfgGpmcConfig1 {
        &self.cfg_gpmc_config1
    }
    #[doc = "0x64 - Chip-select signal timing parameter configuration"]
    #[inline(always)]
    pub const fn cfg_gpmc_config2(&self) -> &CfgGpmcConfig2 {
        &self.cfg_gpmc_config2
    }
    #[doc = "0x68 - ADV# signal timing parameter configuration"]
    #[inline(always)]
    pub const fn cfg_gpmc_config3(&self) -> &CfgGpmcConfig3 {
        &self.cfg_gpmc_config3
    }
    #[doc = "0x6c - WE# and OE# signals timing parameter configuration"]
    #[inline(always)]
    pub const fn cfg_gpmc_config4(&self) -> &CfgGpmcConfig4 {
        &self.cfg_gpmc_config4
    }
    #[doc = "0x70 - RdAccessTime and CycleTime timing parameters configuration"]
    #[inline(always)]
    pub const fn cfg_gpmc_config5(&self) -> &CfgGpmcConfig5 {
        &self.cfg_gpmc_config5
    }
    #[doc = "0x74 - WrAccessTime, WrDataOnADmuxBus, Cycle2Cycle and BusTurnAround parameters configuration"]
    #[inline(always)]
    pub const fn cfg_gpmc_config6(&self) -> &CfgGpmcConfig6 {
        &self.cfg_gpmc_config6
    }
    #[doc = "0x78 - Chip-select address mapping configuration Note: For CS0, the register reset is 0xf40 while for all the other instances CS1-CS7, the reset is 0xf00."]
    #[inline(always)]
    pub const fn cfg_gpmc_config7(&self) -> &CfgGpmcConfig7 {
        &self.cfg_gpmc_config7
    }
    #[doc = "0x7c - This Register is not a true register, just a address location."]
    #[inline(always)]
    pub const fn cfg_gpmc_nand_command(&self) -> &CfgGpmcNandCommand {
        &self.cfg_gpmc_nand_command
    }
    #[doc = "0x80 - This Register is not a true register, just a address location."]
    #[inline(always)]
    pub const fn cfg_gpmc_nand_address(&self) -> &CfgGpmcNandAddress {
        &self.cfg_gpmc_nand_address
    }
    #[doc = "0x84 - This Register is not a true register, just a address location."]
    #[inline(always)]
    pub const fn cfg_gpmc_nand_data(&self) -> &CfgGpmcNandData {
        &self.cfg_gpmc_nand_data
    }
    #[doc = "0x1e0 - Prefetch engine configuration 1"]
    #[inline(always)]
    pub const fn cfg_gpmc_prefetch_config1(&self) -> &CfgGpmcPrefetchConfig1 {
        &self.cfg_gpmc_prefetch_config1
    }
    #[doc = "0x1e4 - Prefetch engine configuration 2"]
    #[inline(always)]
    pub const fn cfg_gpmc_prefetch_config2(&self) -> &CfgGpmcPrefetchConfig2 {
        &self.cfg_gpmc_prefetch_config2
    }
    #[doc = "0x1ec - Prefetch engine control"]
    #[inline(always)]
    pub const fn cfg_gpmc_prefetch_control(&self) -> &CfgGpmcPrefetchControl {
        &self.cfg_gpmc_prefetch_control
    }
    #[doc = "0x1f0 - Prefetch engine status"]
    #[inline(always)]
    pub const fn cfg_gpmc_prefetch_status(&self) -> &CfgGpmcPrefetchStatus {
        &self.cfg_gpmc_prefetch_status
    }
    #[doc = "0x1f4 - ECC configuration"]
    #[inline(always)]
    pub const fn cfg_gpmc_ecc_config(&self) -> &CfgGpmcEccConfig {
        &self.cfg_gpmc_ecc_config
    }
    #[doc = "0x1f8 - ECC control"]
    #[inline(always)]
    pub const fn cfg_gpmc_ecc_control(&self) -> &CfgGpmcEccControl {
        &self.cfg_gpmc_ecc_control
    }
    #[doc = "0x1fc - ECC size"]
    #[inline(always)]
    pub const fn cfg_gpmc_ecc_size_config(&self) -> &CfgGpmcEccSizeConfig {
        &self.cfg_gpmc_ecc_size_config
    }
    #[doc = "0x200 - ECC result register"]
    #[inline(always)]
    pub const fn cfg_gpmc_ecc_result(&self) -> &CfgGpmcEccResult {
        &self.cfg_gpmc_ecc_result
    }
    #[doc = "0x240 - BCH ECC result, bits 0 to 31"]
    #[inline(always)]
    pub const fn cfg_gpmc_bch_result_0(&self) -> &CfgGpmcBchResult0 {
        &self.cfg_gpmc_bch_result_0
    }
    #[doc = "0x244 - BCH ECC result, bits 32 to 63"]
    #[inline(always)]
    pub const fn cfg_gpmc_bch_result_1(&self) -> &CfgGpmcBchResult1 {
        &self.cfg_gpmc_bch_result_1
    }
    #[doc = "0x248 - BCH ECC result, bits 64 to 95"]
    #[inline(always)]
    pub const fn cfg_gpmc_bch_result_2(&self) -> &CfgGpmcBchResult2 {
        &self.cfg_gpmc_bch_result_2
    }
    #[doc = "0x24c - BCH ECC result, bits 96 to 127"]
    #[inline(always)]
    pub const fn cfg_gpmc_bch_result_3(&self) -> &CfgGpmcBchResult3 {
        &self.cfg_gpmc_bch_result_3
    }
    #[doc = "0x2d0 - This register is used to directly pass data to the BCH ECC calculator without accessing the actual NAND flash interface."]
    #[inline(always)]
    pub const fn cfg_gpmc_bch_swdata(&self) -> &CfgGpmcBchSwdata {
        &self.cfg_gpmc_bch_swdata
    }
    #[doc = "0x300 - BCH ECC result, bits 128 to 159"]
    #[inline(always)]
    pub const fn cfg_gpmc_bch_result_4(&self) -> &CfgGpmcBchResult4 {
        &self.cfg_gpmc_bch_result_4
    }
    #[doc = "0x304 - BCH ECC result, bits 160 to 191"]
    #[inline(always)]
    pub const fn cfg_gpmc_bch_result_5(&self) -> &CfgGpmcBchResult5 {
        &self.cfg_gpmc_bch_result_5
    }
    #[doc = "0x308 - BCH ECC result, bits 192 to 207"]
    #[inline(always)]
    pub const fn cfg_gpmc_bch_result_6(&self) -> &CfgGpmcBchResult6 {
        &self.cfg_gpmc_bch_result_6
    }
}
#[doc = "CFG_GPMC_REVISION (rw) register accessor: This register contains the IP revision code\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_revision`]
module"]
#[doc(alias = "CFG_GPMC_REVISION")]
pub type CfgGpmcRevision = crate::Reg<cfg_gpmc_revision::CfgGpmcRevisionSpec>;
#[doc = "This register contains the IP revision code"]
pub mod cfg_gpmc_revision;
#[doc = "CFG_GPMC_SYSCONFIG (rw) register accessor: This register controls the various parameters of the OCP interface\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_sysconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_sysconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_sysconfig`]
module"]
#[doc(alias = "CFG_GPMC_SYSCONFIG")]
pub type CfgGpmcSysconfig = crate::Reg<cfg_gpmc_sysconfig::CfgGpmcSysconfigSpec>;
#[doc = "This register controls the various parameters of the OCP interface"]
pub mod cfg_gpmc_sysconfig;
#[doc = "CFG_GPMC_SYSSTATUS (rw) register accessor: This register provides status information about the module, excluding the interrupt status information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_sysstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_sysstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_sysstatus`]
module"]
#[doc(alias = "CFG_GPMC_SYSSTATUS")]
pub type CfgGpmcSysstatus = crate::Reg<cfg_gpmc_sysstatus::CfgGpmcSysstatusSpec>;
#[doc = "This register provides status information about the module, excluding the interrupt status information"]
pub mod cfg_gpmc_sysstatus;
#[doc = "CFG_GPMC_IRQSTATUS (rw) register accessor: This interrupt status register regroups all the status of the module internal events that can generate an interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_irqstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_irqstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_irqstatus`]
module"]
#[doc(alias = "CFG_GPMC_IRQSTATUS")]
pub type CfgGpmcIrqstatus = crate::Reg<cfg_gpmc_irqstatus::CfgGpmcIrqstatusSpec>;
#[doc = "This interrupt status register regroups all the status of the module internal events that can generate an interrupt."]
pub mod cfg_gpmc_irqstatus;
#[doc = "CFG_GPMC_IRQENABLE (rw) register accessor: The interrupt enable register allows to mask/unmask the module internal sources of interrupt, on a event-by-event basis.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_irqenable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_irqenable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_irqenable`]
module"]
#[doc(alias = "CFG_GPMC_IRQENABLE")]
pub type CfgGpmcIrqenable = crate::Reg<cfg_gpmc_irqenable::CfgGpmcIrqenableSpec>;
#[doc = "The interrupt enable register allows to mask/unmask the module internal sources of interrupt, on a event-by-event basis."]
pub mod cfg_gpmc_irqenable;
#[doc = "CFG_GPMC_TIMEOUT_CONTROL (rw) register accessor: The GPMC_TIMEOUT_CONTROL register allows the user to set the start value of the timeout counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_timeout_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_timeout_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_timeout_control`]
module"]
#[doc(alias = "CFG_GPMC_TIMEOUT_CONTROL")]
pub type CfgGpmcTimeoutControl = crate::Reg<cfg_gpmc_timeout_control::CfgGpmcTimeoutControlSpec>;
#[doc = "The GPMC_TIMEOUT_CONTROL register allows the user to set the start value of the timeout counter"]
pub mod cfg_gpmc_timeout_control;
#[doc = "CFG_GPMC_ERR_ADDRESS (rw) register accessor: The GPMC_ERR_ADDRESS register stores the address of the illegal access when an error occurs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_err_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_err_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_err_address`]
module"]
#[doc(alias = "CFG_GPMC_ERR_ADDRESS")]
pub type CfgGpmcErrAddress = crate::Reg<cfg_gpmc_err_address::CfgGpmcErrAddressSpec>;
#[doc = "The GPMC_ERR_ADDRESS register stores the address of the illegal access when an error occurs"]
pub mod cfg_gpmc_err_address;
#[doc = "CFG_GPMC_ERR_TYPE (rw) register accessor: The GPMC_ERR_TYPE register stores the type of error when an error occurs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_err_type::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_err_type::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_err_type`]
module"]
#[doc(alias = "CFG_GPMC_ERR_TYPE")]
pub type CfgGpmcErrType = crate::Reg<cfg_gpmc_err_type::CfgGpmcErrTypeSpec>;
#[doc = "The GPMC_ERR_TYPE register stores the type of error when an error occurs"]
pub mod cfg_gpmc_err_type;
#[doc = "CFG_GPMC_CONFIG (rw) register accessor: The configuration register allows global configuration of the GPMC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_config`]
module"]
#[doc(alias = "CFG_GPMC_CONFIG")]
pub type CfgGpmcConfig = crate::Reg<cfg_gpmc_config::CfgGpmcConfigSpec>;
#[doc = "The configuration register allows global configuration of the GPMC"]
pub mod cfg_gpmc_config;
#[doc = "CFG_GPMC_STATUS (rw) register accessor: The status register provides global status bits of the GPMC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_status`]
module"]
#[doc(alias = "CFG_GPMC_STATUS")]
pub type CfgGpmcStatus = crate::Reg<cfg_gpmc_status::CfgGpmcStatusSpec>;
#[doc = "The status register provides global status bits of the GPMC"]
pub mod cfg_gpmc_status;
#[doc = "CFG_GPMC_PREFETCH_CONFIG1 (rw) register accessor: Prefetch engine configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_prefetch_config1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_prefetch_config1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_prefetch_config1`]
module"]
#[doc(alias = "CFG_GPMC_PREFETCH_CONFIG1")]
pub type CfgGpmcPrefetchConfig1 = crate::Reg<cfg_gpmc_prefetch_config1::CfgGpmcPrefetchConfig1Spec>;
#[doc = "Prefetch engine configuration 1"]
pub mod cfg_gpmc_prefetch_config1;
#[doc = "CFG_GPMC_PREFETCH_CONFIG2 (rw) register accessor: Prefetch engine configuration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_prefetch_config2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_prefetch_config2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_prefetch_config2`]
module"]
#[doc(alias = "CFG_GPMC_PREFETCH_CONFIG2")]
pub type CfgGpmcPrefetchConfig2 = crate::Reg<cfg_gpmc_prefetch_config2::CfgGpmcPrefetchConfig2Spec>;
#[doc = "Prefetch engine configuration 2"]
pub mod cfg_gpmc_prefetch_config2;
#[doc = "CFG_GPMC_PREFETCH_CONTROL (rw) register accessor: Prefetch engine control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_prefetch_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_prefetch_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_prefetch_control`]
module"]
#[doc(alias = "CFG_GPMC_PREFETCH_CONTROL")]
pub type CfgGpmcPrefetchControl = crate::Reg<cfg_gpmc_prefetch_control::CfgGpmcPrefetchControlSpec>;
#[doc = "Prefetch engine control"]
pub mod cfg_gpmc_prefetch_control;
#[doc = "CFG_GPMC_PREFETCH_STATUS (rw) register accessor: Prefetch engine status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_prefetch_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_prefetch_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_prefetch_status`]
module"]
#[doc(alias = "CFG_GPMC_PREFETCH_STATUS")]
pub type CfgGpmcPrefetchStatus = crate::Reg<cfg_gpmc_prefetch_status::CfgGpmcPrefetchStatusSpec>;
#[doc = "Prefetch engine status"]
pub mod cfg_gpmc_prefetch_status;
#[doc = "CFG_GPMC_ECC_CONFIG (rw) register accessor: ECC configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_ecc_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_ecc_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_ecc_config`]
module"]
#[doc(alias = "CFG_GPMC_ECC_CONFIG")]
pub type CfgGpmcEccConfig = crate::Reg<cfg_gpmc_ecc_config::CfgGpmcEccConfigSpec>;
#[doc = "ECC configuration"]
pub mod cfg_gpmc_ecc_config;
#[doc = "CFG_GPMC_ECC_CONTROL (rw) register accessor: ECC control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_ecc_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_ecc_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_ecc_control`]
module"]
#[doc(alias = "CFG_GPMC_ECC_CONTROL")]
pub type CfgGpmcEccControl = crate::Reg<cfg_gpmc_ecc_control::CfgGpmcEccControlSpec>;
#[doc = "ECC control"]
pub mod cfg_gpmc_ecc_control;
#[doc = "CFG_GPMC_ECC_SIZE_CONFIG (rw) register accessor: ECC size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_ecc_size_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_ecc_size_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_ecc_size_config`]
module"]
#[doc(alias = "CFG_GPMC_ECC_SIZE_CONFIG")]
pub type CfgGpmcEccSizeConfig = crate::Reg<cfg_gpmc_ecc_size_config::CfgGpmcEccSizeConfigSpec>;
#[doc = "ECC size"]
pub mod cfg_gpmc_ecc_size_config;
#[doc = "CFG_GPMC_ECC_RESULT (rw) register accessor: ECC result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_ecc_result::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_ecc_result::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_ecc_result`]
module"]
#[doc(alias = "CFG_GPMC_ECC_RESULT")]
pub type CfgGpmcEccResult = crate::Reg<cfg_gpmc_ecc_result::CfgGpmcEccResultSpec>;
#[doc = "ECC result register"]
pub mod cfg_gpmc_ecc_result;
#[doc = "CFG_GPMC_BCH_SWDATA (rw) register accessor: This register is used to directly pass data to the BCH ECC calculator without accessing the actual NAND flash interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_bch_swdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_bch_swdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_bch_swdata`]
module"]
#[doc(alias = "CFG_GPMC_BCH_SWDATA")]
pub type CfgGpmcBchSwdata = crate::Reg<cfg_gpmc_bch_swdata::CfgGpmcBchSwdataSpec>;
#[doc = "This register is used to directly pass data to the BCH ECC calculator without accessing the actual NAND flash interface."]
pub mod cfg_gpmc_bch_swdata;
#[doc = "CFG_GPMC_CONFIG1 (rw) register accessor: The configuration 1 register sets signal control parameters per chip select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_config1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_config1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_config1`]
module"]
#[doc(alias = "CFG_GPMC_CONFIG1")]
pub type CfgGpmcConfig1 = crate::Reg<cfg_gpmc_config1::CfgGpmcConfig1Spec>;
#[doc = "The configuration 1 register sets signal control parameters per chip select"]
pub mod cfg_gpmc_config1;
#[doc = "CFG_GPMC_CONFIG2 (rw) register accessor: Chip-select signal timing parameter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_config2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_config2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_config2`]
module"]
#[doc(alias = "CFG_GPMC_CONFIG2")]
pub type CfgGpmcConfig2 = crate::Reg<cfg_gpmc_config2::CfgGpmcConfig2Spec>;
#[doc = "Chip-select signal timing parameter configuration"]
pub mod cfg_gpmc_config2;
#[doc = "CFG_GPMC_CONFIG3 (rw) register accessor: ADV# signal timing parameter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_config3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_config3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_config3`]
module"]
#[doc(alias = "CFG_GPMC_CONFIG3")]
pub type CfgGpmcConfig3 = crate::Reg<cfg_gpmc_config3::CfgGpmcConfig3Spec>;
#[doc = "ADV# signal timing parameter configuration"]
pub mod cfg_gpmc_config3;
#[doc = "CFG_GPMC_CONFIG4 (rw) register accessor: WE# and OE# signals timing parameter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_config4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_config4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_config4`]
module"]
#[doc(alias = "CFG_GPMC_CONFIG4")]
pub type CfgGpmcConfig4 = crate::Reg<cfg_gpmc_config4::CfgGpmcConfig4Spec>;
#[doc = "WE# and OE# signals timing parameter configuration"]
pub mod cfg_gpmc_config4;
#[doc = "CFG_GPMC_CONFIG5 (rw) register accessor: RdAccessTime and CycleTime timing parameters configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_config5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_config5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_config5`]
module"]
#[doc(alias = "CFG_GPMC_CONFIG5")]
pub type CfgGpmcConfig5 = crate::Reg<cfg_gpmc_config5::CfgGpmcConfig5Spec>;
#[doc = "RdAccessTime and CycleTime timing parameters configuration"]
pub mod cfg_gpmc_config5;
#[doc = "CFG_GPMC_CONFIG6 (rw) register accessor: WrAccessTime, WrDataOnADmuxBus, Cycle2Cycle and BusTurnAround parameters configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_config6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_config6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_config6`]
module"]
#[doc(alias = "CFG_GPMC_CONFIG6")]
pub type CfgGpmcConfig6 = crate::Reg<cfg_gpmc_config6::CfgGpmcConfig6Spec>;
#[doc = "WrAccessTime, WrDataOnADmuxBus, Cycle2Cycle and BusTurnAround parameters configuration"]
pub mod cfg_gpmc_config6;
#[doc = "CFG_GPMC_CONFIG7 (rw) register accessor: Chip-select address mapping configuration Note: For CS0, the register reset is 0xf40 while for all the other instances CS1-CS7, the reset is 0xf00.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_config7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_config7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_config7`]
module"]
#[doc(alias = "CFG_GPMC_CONFIG7")]
pub type CfgGpmcConfig7 = crate::Reg<cfg_gpmc_config7::CfgGpmcConfig7Spec>;
#[doc = "Chip-select address mapping configuration Note: For CS0, the register reset is 0xf40 while for all the other instances CS1-CS7, the reset is 0xf00."]
pub mod cfg_gpmc_config7;
#[doc = "CFG_GPMC_NAND_COMMAND (rw) register accessor: This Register is not a true register, just a address location.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_nand_command::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_nand_command::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_nand_command`]
module"]
#[doc(alias = "CFG_GPMC_NAND_COMMAND")]
pub type CfgGpmcNandCommand = crate::Reg<cfg_gpmc_nand_command::CfgGpmcNandCommandSpec>;
#[doc = "This Register is not a true register, just a address location."]
pub mod cfg_gpmc_nand_command;
#[doc = "CFG_GPMC_NAND_ADDRESS (rw) register accessor: This Register is not a true register, just a address location.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_nand_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_nand_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_nand_address`]
module"]
#[doc(alias = "CFG_GPMC_NAND_ADDRESS")]
pub type CfgGpmcNandAddress = crate::Reg<cfg_gpmc_nand_address::CfgGpmcNandAddressSpec>;
#[doc = "This Register is not a true register, just a address location."]
pub mod cfg_gpmc_nand_address;
#[doc = "CFG_GPMC_NAND_DATA (rw) register accessor: This Register is not a true register, just a address location.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_nand_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_nand_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_nand_data`]
module"]
#[doc(alias = "CFG_GPMC_NAND_DATA")]
pub type CfgGpmcNandData = crate::Reg<cfg_gpmc_nand_data::CfgGpmcNandDataSpec>;
#[doc = "This Register is not a true register, just a address location."]
pub mod cfg_gpmc_nand_data;
#[doc = "CFG_GPMC_BCH_RESULT_0 (rw) register accessor: BCH ECC result, bits 0 to 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_bch_result_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_bch_result_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_bch_result_0`]
module"]
#[doc(alias = "CFG_GPMC_BCH_RESULT_0")]
pub type CfgGpmcBchResult0 = crate::Reg<cfg_gpmc_bch_result_0::CfgGpmcBchResult0Spec>;
#[doc = "BCH ECC result, bits 0 to 31"]
pub mod cfg_gpmc_bch_result_0;
#[doc = "CFG_GPMC_BCH_RESULT_1 (rw) register accessor: BCH ECC result, bits 32 to 63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_bch_result_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_bch_result_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_bch_result_1`]
module"]
#[doc(alias = "CFG_GPMC_BCH_RESULT_1")]
pub type CfgGpmcBchResult1 = crate::Reg<cfg_gpmc_bch_result_1::CfgGpmcBchResult1Spec>;
#[doc = "BCH ECC result, bits 32 to 63"]
pub mod cfg_gpmc_bch_result_1;
#[doc = "CFG_GPMC_BCH_RESULT_2 (rw) register accessor: BCH ECC result, bits 64 to 95\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_bch_result_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_bch_result_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_bch_result_2`]
module"]
#[doc(alias = "CFG_GPMC_BCH_RESULT_2")]
pub type CfgGpmcBchResult2 = crate::Reg<cfg_gpmc_bch_result_2::CfgGpmcBchResult2Spec>;
#[doc = "BCH ECC result, bits 64 to 95"]
pub mod cfg_gpmc_bch_result_2;
#[doc = "CFG_GPMC_BCH_RESULT_3 (rw) register accessor: BCH ECC result, bits 96 to 127\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_bch_result_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_bch_result_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_bch_result_3`]
module"]
#[doc(alias = "CFG_GPMC_BCH_RESULT_3")]
pub type CfgGpmcBchResult3 = crate::Reg<cfg_gpmc_bch_result_3::CfgGpmcBchResult3Spec>;
#[doc = "BCH ECC result, bits 96 to 127"]
pub mod cfg_gpmc_bch_result_3;
#[doc = "CFG_GPMC_BCH_RESULT_4 (rw) register accessor: BCH ECC result, bits 128 to 159\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_bch_result_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_bch_result_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_bch_result_4`]
module"]
#[doc(alias = "CFG_GPMC_BCH_RESULT_4")]
pub type CfgGpmcBchResult4 = crate::Reg<cfg_gpmc_bch_result_4::CfgGpmcBchResult4Spec>;
#[doc = "BCH ECC result, bits 128 to 159"]
pub mod cfg_gpmc_bch_result_4;
#[doc = "CFG_GPMC_BCH_RESULT_5 (rw) register accessor: BCH ECC result, bits 160 to 191\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_bch_result_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_bch_result_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_bch_result_5`]
module"]
#[doc(alias = "CFG_GPMC_BCH_RESULT_5")]
pub type CfgGpmcBchResult5 = crate::Reg<cfg_gpmc_bch_result_5::CfgGpmcBchResult5Spec>;
#[doc = "BCH ECC result, bits 160 to 191"]
pub mod cfg_gpmc_bch_result_5;
#[doc = "CFG_GPMC_BCH_RESULT_6 (rw) register accessor: BCH ECC result, bits 192 to 207\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_bch_result_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_bch_result_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gpmc_bch_result_6`]
module"]
#[doc(alias = "CFG_GPMC_BCH_RESULT_6")]
pub type CfgGpmcBchResult6 = crate::Reg<cfg_gpmc_bch_result_6::CfgGpmcBchResult6Spec>;
#[doc = "BCH ECC result, bits 192 to 207"]
pub mod cfg_gpmc_bch_result_6;
