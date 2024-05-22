#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg_i2c_revnb_lo: CfgI2cRevnbLo,
    cfg_i2c_revnb_hi: CfgI2cRevnbHi,
    _reserved2: [u8; 0x08],
    cfg_i2c_sysc: CfgI2cSysc,
    _reserved3: [u8; 0x0c],
    cfg_i2c_eoi: CfgI2cEoi,
    cfg_i2c_irqstatus_raw: CfgI2cIrqstatusRaw,
    cfg_i2c_irqstatus: CfgI2cIrqstatus,
    cfg_i2c_irqenable_set: CfgI2cIrqenableSet,
    cfg_i2c_irqenable_clr: CfgI2cIrqenableClr,
    cfg_i2c_we: CfgI2cWe,
    cfg_i2c_dmarxenable_set: CfgI2cDmarxenableSet,
    cfg_i2c_dmatxenable_set: CfgI2cDmatxenableSet,
    cfg_i2c_dmarxenable_clr: CfgI2cDmarxenableClr,
    cfg_i2c_dmatxenable_clr: CfgI2cDmatxenableClr,
    cfg_i2c_dmarxwake_en: CfgI2cDmarxwakeEn,
    cfg_i2c_dmatxwake_en: CfgI2cDmatxwakeEn,
    _reserved15: [u8; 0x34],
    cfg_i2c_ie: CfgI2cIe,
    cfg_i2c_stat: CfgI2cStat,
    _reserved17: [u8; 0x04],
    cfg_i2c_syss: CfgI2cSyss,
    cfg_i2c_buf: CfgI2cBuf,
    cfg_i2c_cnt: CfgI2cCnt,
    cfg_i2c_data: CfgI2cData,
    _reserved21: [u8; 0x04],
    cfg_i2c_con: CfgI2cCon,
    cfg_i2c_oa: CfgI2cOa,
    cfg_i2c_sa: CfgI2cSa,
    cfg_i2c_psc: CfgI2cPsc,
    cfg_i2c_scll: CfgI2cScll,
    cfg_i2c_sclh: CfgI2cSclh,
    cfg_i2c_systest: CfgI2cSystest,
    cfg_i2c_bufstat: CfgI2cBufstat,
    cfg_i2c_oa1: CfgI2cOa1,
    cfg_i2c_oa2: CfgI2cOa2,
    cfg_i2c_oa3: CfgI2cOa3,
    cfg_i2c_actoa: CfgI2cActoa,
    cfg_i2c_sblock: CfgI2cSblock,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision Number register (Low)"]
    #[inline(always)]
    pub const fn cfg_i2c_revnb_lo(&self) -> &CfgI2cRevnbLo {
        &self.cfg_i2c_revnb_lo
    }
    #[doc = "0x04 - Revision Number register (High)"]
    #[inline(always)]
    pub const fn cfg_i2c_revnb_hi(&self) -> &CfgI2cRevnbHi {
        &self.cfg_i2c_revnb_hi
    }
    #[doc = "0x10 - System Configuration register"]
    #[inline(always)]
    pub const fn cfg_i2c_sysc(&self) -> &CfgI2cSysc {
        &self.cfg_i2c_sysc
    }
    #[doc = "0x20 - End Of Interrupt number specification"]
    #[inline(always)]
    pub const fn cfg_i2c_eoi(&self) -> &CfgI2cEoi {
        &self.cfg_i2c_eoi
    }
    #[doc = "0x24 - Per-event raw interrupt status vector"]
    #[inline(always)]
    pub const fn cfg_i2c_irqstatus_raw(&self) -> &CfgI2cIrqstatusRaw {
        &self.cfg_i2c_irqstatus_raw
    }
    #[doc = "0x28 - Per-event enabled interrupt status vector"]
    #[inline(always)]
    pub const fn cfg_i2c_irqstatus(&self) -> &CfgI2cIrqstatus {
        &self.cfg_i2c_irqstatus
    }
    #[doc = "0x2c - Per-event interrupt enable bit vector."]
    #[inline(always)]
    pub const fn cfg_i2c_irqenable_set(&self) -> &CfgI2cIrqenableSet {
        &self.cfg_i2c_irqenable_set
    }
    #[doc = "0x30 - Per-event interrupt clear bit vector."]
    #[inline(always)]
    pub const fn cfg_i2c_irqenable_clr(&self) -> &CfgI2cIrqenableClr {
        &self.cfg_i2c_irqenable_clr
    }
    #[doc = "0x34 - I2C wakeup enable vector (legacy)."]
    #[inline(always)]
    pub const fn cfg_i2c_we(&self) -> &CfgI2cWe {
        &self.cfg_i2c_we
    }
    #[doc = "0x38 - Per-event DMA RX enable set."]
    #[inline(always)]
    pub const fn cfg_i2c_dmarxenable_set(&self) -> &CfgI2cDmarxenableSet {
        &self.cfg_i2c_dmarxenable_set
    }
    #[doc = "0x3c - Per-event DMA TX enable set."]
    #[inline(always)]
    pub const fn cfg_i2c_dmatxenable_set(&self) -> &CfgI2cDmatxenableSet {
        &self.cfg_i2c_dmatxenable_set
    }
    #[doc = "0x40 - Per-event DMA RX enable clear."]
    #[inline(always)]
    pub const fn cfg_i2c_dmarxenable_clr(&self) -> &CfgI2cDmarxenableClr {
        &self.cfg_i2c_dmarxenable_clr
    }
    #[doc = "0x44 - Per-event DMA TX enable clear."]
    #[inline(always)]
    pub const fn cfg_i2c_dmatxenable_clr(&self) -> &CfgI2cDmatxenableClr {
        &self.cfg_i2c_dmatxenable_clr
    }
    #[doc = "0x48 - Per-event DMA RX wakeup enable."]
    #[inline(always)]
    pub const fn cfg_i2c_dmarxwake_en(&self) -> &CfgI2cDmarxwakeEn {
        &self.cfg_i2c_dmarxwake_en
    }
    #[doc = "0x4c - Per-event DMA TX wakeup enable."]
    #[inline(always)]
    pub const fn cfg_i2c_dmatxwake_en(&self) -> &CfgI2cDmatxwakeEn {
        &self.cfg_i2c_dmatxwake_en
    }
    #[doc = "0x84 - I2C interrupt enable vector (legacy)."]
    #[inline(always)]
    pub const fn cfg_i2c_ie(&self) -> &CfgI2cIe {
        &self.cfg_i2c_ie
    }
    #[doc = "0x88 - I2C interrupt status vector (legacy)."]
    #[inline(always)]
    pub const fn cfg_i2c_stat(&self) -> &CfgI2cStat {
        &self.cfg_i2c_stat
    }
    #[doc = "0x90 - System Status register"]
    #[inline(always)]
    pub const fn cfg_i2c_syss(&self) -> &CfgI2cSyss {
        &self.cfg_i2c_syss
    }
    #[doc = "0x94 - Buffer Configuration register"]
    #[inline(always)]
    pub const fn cfg_i2c_buf(&self) -> &CfgI2cBuf {
        &self.cfg_i2c_buf
    }
    #[doc = "0x98 - Data counter register"]
    #[inline(always)]
    pub const fn cfg_i2c_cnt(&self) -> &CfgI2cCnt {
        &self.cfg_i2c_cnt
    }
    #[doc = "0x9c - Data access register"]
    #[inline(always)]
    pub const fn cfg_i2c_data(&self) -> &CfgI2cData {
        &self.cfg_i2c_data
    }
    #[doc = "0xa4 - I2C configuration register."]
    #[inline(always)]
    pub const fn cfg_i2c_con(&self) -> &CfgI2cCon {
        &self.cfg_i2c_con
    }
    #[doc = "0xa8 - Own address register"]
    #[inline(always)]
    pub const fn cfg_i2c_oa(&self) -> &CfgI2cOa {
        &self.cfg_i2c_oa
    }
    #[doc = "0xac - Slave address register"]
    #[inline(always)]
    pub const fn cfg_i2c_sa(&self) -> &CfgI2cSa {
        &self.cfg_i2c_sa
    }
    #[doc = "0xb0 - I2C Clock Prescaler Register"]
    #[inline(always)]
    pub const fn cfg_i2c_psc(&self) -> &CfgI2cPsc {
        &self.cfg_i2c_psc
    }
    #[doc = "0xb4 - I2C SCL Low Time Register."]
    #[inline(always)]
    pub const fn cfg_i2c_scll(&self) -> &CfgI2cScll {
        &self.cfg_i2c_scll
    }
    #[doc = "0xb8 - I2C SCL High Time Register."]
    #[inline(always)]
    pub const fn cfg_i2c_sclh(&self) -> &CfgI2cSclh {
        &self.cfg_i2c_sclh
    }
    #[doc = "0xbc - I2C System Test Register."]
    #[inline(always)]
    pub const fn cfg_i2c_systest(&self) -> &CfgI2cSystest {
        &self.cfg_i2c_systest
    }
    #[doc = "0xc0 - I2C Buffer Status Register."]
    #[inline(always)]
    pub const fn cfg_i2c_bufstat(&self) -> &CfgI2cBufstat {
        &self.cfg_i2c_bufstat
    }
    #[doc = "0xc4 - I2C Own Address 1 Register"]
    #[inline(always)]
    pub const fn cfg_i2c_oa1(&self) -> &CfgI2cOa1 {
        &self.cfg_i2c_oa1
    }
    #[doc = "0xc8 - I2C Own Address 2 Register"]
    #[inline(always)]
    pub const fn cfg_i2c_oa2(&self) -> &CfgI2cOa2 {
        &self.cfg_i2c_oa2
    }
    #[doc = "0xcc - I2C Own Address 3 Register"]
    #[inline(always)]
    pub const fn cfg_i2c_oa3(&self) -> &CfgI2cOa3 {
        &self.cfg_i2c_oa3
    }
    #[doc = "0xd0 - I2C Active Own Address Register."]
    #[inline(always)]
    pub const fn cfg_i2c_actoa(&self) -> &CfgI2cActoa {
        &self.cfg_i2c_actoa
    }
    #[doc = "0xd4 - I2C Clock Blocking Enable Register."]
    #[inline(always)]
    pub const fn cfg_i2c_sblock(&self) -> &CfgI2cSblock {
        &self.cfg_i2c_sblock
    }
}
#[doc = "CFG_I2C_REVNB_LO (rw) register accessor: Revision Number register (Low)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_revnb_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_revnb_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_revnb_lo`]
module"]
#[doc(alias = "CFG_I2C_REVNB_LO")]
pub type CfgI2cRevnbLo = crate::Reg<cfg_i2c_revnb_lo::CfgI2cRevnbLoSpec>;
#[doc = "Revision Number register (Low)"]
pub mod cfg_i2c_revnb_lo;
#[doc = "CFG_I2C_REVNB_HI (rw) register accessor: Revision Number register (High)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_revnb_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_revnb_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_revnb_hi`]
module"]
#[doc(alias = "CFG_I2C_REVNB_HI")]
pub type CfgI2cRevnbHi = crate::Reg<cfg_i2c_revnb_hi::CfgI2cRevnbHiSpec>;
#[doc = "Revision Number register (High)"]
pub mod cfg_i2c_revnb_hi;
#[doc = "CFG_I2C_SYSC (rw) register accessor: System Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_sysc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_sysc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_sysc`]
module"]
#[doc(alias = "CFG_I2C_SYSC")]
pub type CfgI2cSysc = crate::Reg<cfg_i2c_sysc::CfgI2cSyscSpec>;
#[doc = "System Configuration register"]
pub mod cfg_i2c_sysc;
#[doc = "CFG_I2C_EOI (rw) register accessor: End Of Interrupt number specification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_eoi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_eoi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_eoi`]
module"]
#[doc(alias = "CFG_I2C_EOI")]
pub type CfgI2cEoi = crate::Reg<cfg_i2c_eoi::CfgI2cEoiSpec>;
#[doc = "End Of Interrupt number specification"]
pub mod cfg_i2c_eoi;
#[doc = "CFG_I2C_IRQSTATUS_RAW (rw) register accessor: Per-event raw interrupt status vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_irqstatus_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_irqstatus_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_irqstatus_raw`]
module"]
#[doc(alias = "CFG_I2C_IRQSTATUS_RAW")]
pub type CfgI2cIrqstatusRaw = crate::Reg<cfg_i2c_irqstatus_raw::CfgI2cIrqstatusRawSpec>;
#[doc = "Per-event raw interrupt status vector"]
pub mod cfg_i2c_irqstatus_raw;
#[doc = "CFG_I2C_IRQSTATUS (rw) register accessor: Per-event enabled interrupt status vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_irqstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_irqstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_irqstatus`]
module"]
#[doc(alias = "CFG_I2C_IRQSTATUS")]
pub type CfgI2cIrqstatus = crate::Reg<cfg_i2c_irqstatus::CfgI2cIrqstatusSpec>;
#[doc = "Per-event enabled interrupt status vector"]
pub mod cfg_i2c_irqstatus;
#[doc = "CFG_I2C_IRQENABLE_SET (rw) register accessor: Per-event interrupt enable bit vector.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_irqenable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_irqenable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_irqenable_set`]
module"]
#[doc(alias = "CFG_I2C_IRQENABLE_SET")]
pub type CfgI2cIrqenableSet = crate::Reg<cfg_i2c_irqenable_set::CfgI2cIrqenableSetSpec>;
#[doc = "Per-event interrupt enable bit vector."]
pub mod cfg_i2c_irqenable_set;
#[doc = "CFG_I2C_IRQENABLE_CLR (rw) register accessor: Per-event interrupt clear bit vector.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_irqenable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_irqenable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_irqenable_clr`]
module"]
#[doc(alias = "CFG_I2C_IRQENABLE_CLR")]
pub type CfgI2cIrqenableClr = crate::Reg<cfg_i2c_irqenable_clr::CfgI2cIrqenableClrSpec>;
#[doc = "Per-event interrupt clear bit vector."]
pub mod cfg_i2c_irqenable_clr;
#[doc = "CFG_I2C_WE (rw) register accessor: I2C wakeup enable vector (legacy).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_we::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_we::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_we`]
module"]
#[doc(alias = "CFG_I2C_WE")]
pub type CfgI2cWe = crate::Reg<cfg_i2c_we::CfgI2cWeSpec>;
#[doc = "I2C wakeup enable vector (legacy)."]
pub mod cfg_i2c_we;
#[doc = "CFG_I2C_DMARXENABLE_SET (rw) register accessor: Per-event DMA RX enable set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_dmarxenable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_dmarxenable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_dmarxenable_set`]
module"]
#[doc(alias = "CFG_I2C_DMARXENABLE_SET")]
pub type CfgI2cDmarxenableSet = crate::Reg<cfg_i2c_dmarxenable_set::CfgI2cDmarxenableSetSpec>;
#[doc = "Per-event DMA RX enable set."]
pub mod cfg_i2c_dmarxenable_set;
#[doc = "CFG_I2C_DMATXENABLE_SET (rw) register accessor: Per-event DMA TX enable set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_dmatxenable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_dmatxenable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_dmatxenable_set`]
module"]
#[doc(alias = "CFG_I2C_DMATXENABLE_SET")]
pub type CfgI2cDmatxenableSet = crate::Reg<cfg_i2c_dmatxenable_set::CfgI2cDmatxenableSetSpec>;
#[doc = "Per-event DMA TX enable set."]
pub mod cfg_i2c_dmatxenable_set;
#[doc = "CFG_I2C_DMARXENABLE_CLR (rw) register accessor: Per-event DMA RX enable clear.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_dmarxenable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_dmarxenable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_dmarxenable_clr`]
module"]
#[doc(alias = "CFG_I2C_DMARXENABLE_CLR")]
pub type CfgI2cDmarxenableClr = crate::Reg<cfg_i2c_dmarxenable_clr::CfgI2cDmarxenableClrSpec>;
#[doc = "Per-event DMA RX enable clear."]
pub mod cfg_i2c_dmarxenable_clr;
#[doc = "CFG_I2C_DMATXENABLE_CLR (rw) register accessor: Per-event DMA TX enable clear.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_dmatxenable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_dmatxenable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_dmatxenable_clr`]
module"]
#[doc(alias = "CFG_I2C_DMATXENABLE_CLR")]
pub type CfgI2cDmatxenableClr = crate::Reg<cfg_i2c_dmatxenable_clr::CfgI2cDmatxenableClrSpec>;
#[doc = "Per-event DMA TX enable clear."]
pub mod cfg_i2c_dmatxenable_clr;
#[doc = "CFG_I2C_DMARXWAKE_EN (rw) register accessor: Per-event DMA RX wakeup enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_dmarxwake_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_dmarxwake_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_dmarxwake_en`]
module"]
#[doc(alias = "CFG_I2C_DMARXWAKE_EN")]
pub type CfgI2cDmarxwakeEn = crate::Reg<cfg_i2c_dmarxwake_en::CfgI2cDmarxwakeEnSpec>;
#[doc = "Per-event DMA RX wakeup enable."]
pub mod cfg_i2c_dmarxwake_en;
#[doc = "CFG_I2C_DMATXWAKE_EN (rw) register accessor: Per-event DMA TX wakeup enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_dmatxwake_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_dmatxwake_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_dmatxwake_en`]
module"]
#[doc(alias = "CFG_I2C_DMATXWAKE_EN")]
pub type CfgI2cDmatxwakeEn = crate::Reg<cfg_i2c_dmatxwake_en::CfgI2cDmatxwakeEnSpec>;
#[doc = "Per-event DMA TX wakeup enable."]
pub mod cfg_i2c_dmatxwake_en;
#[doc = "CFG_I2C_IE (rw) register accessor: I2C interrupt enable vector (legacy).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_ie`]
module"]
#[doc(alias = "CFG_I2C_IE")]
pub type CfgI2cIe = crate::Reg<cfg_i2c_ie::CfgI2cIeSpec>;
#[doc = "I2C interrupt enable vector (legacy)."]
pub mod cfg_i2c_ie;
#[doc = "CFG_I2C_STAT (rw) register accessor: I2C interrupt status vector (legacy).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_stat`]
module"]
#[doc(alias = "CFG_I2C_STAT")]
pub type CfgI2cStat = crate::Reg<cfg_i2c_stat::CfgI2cStatSpec>;
#[doc = "I2C interrupt status vector (legacy)."]
pub mod cfg_i2c_stat;
#[doc = "CFG_I2C_SYSS (rw) register accessor: System Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_syss::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_syss::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_syss`]
module"]
#[doc(alias = "CFG_I2C_SYSS")]
pub type CfgI2cSyss = crate::Reg<cfg_i2c_syss::CfgI2cSyssSpec>;
#[doc = "System Status register"]
pub mod cfg_i2c_syss;
#[doc = "CFG_I2C_BUF (rw) register accessor: Buffer Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_buf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_buf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_buf`]
module"]
#[doc(alias = "CFG_I2C_BUF")]
pub type CfgI2cBuf = crate::Reg<cfg_i2c_buf::CfgI2cBufSpec>;
#[doc = "Buffer Configuration register"]
pub mod cfg_i2c_buf;
#[doc = "CFG_I2C_CNT (rw) register accessor: Data counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_cnt`]
module"]
#[doc(alias = "CFG_I2C_CNT")]
pub type CfgI2cCnt = crate::Reg<cfg_i2c_cnt::CfgI2cCntSpec>;
#[doc = "Data counter register"]
pub mod cfg_i2c_cnt;
#[doc = "CFG_I2C_DATA (rw) register accessor: Data access register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_data`]
module"]
#[doc(alias = "CFG_I2C_DATA")]
pub type CfgI2cData = crate::Reg<cfg_i2c_data::CfgI2cDataSpec>;
#[doc = "Data access register"]
pub mod cfg_i2c_data;
#[doc = "CFG_I2C_CON (rw) register accessor: I2C configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_con`]
module"]
#[doc(alias = "CFG_I2C_CON")]
pub type CfgI2cCon = crate::Reg<cfg_i2c_con::CfgI2cConSpec>;
#[doc = "I2C configuration register."]
pub mod cfg_i2c_con;
#[doc = "CFG_I2C_OA (rw) register accessor: Own address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_oa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_oa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_oa`]
module"]
#[doc(alias = "CFG_I2C_OA")]
pub type CfgI2cOa = crate::Reg<cfg_i2c_oa::CfgI2cOaSpec>;
#[doc = "Own address register"]
pub mod cfg_i2c_oa;
#[doc = "CFG_I2C_SA (rw) register accessor: Slave address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_sa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_sa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_sa`]
module"]
#[doc(alias = "CFG_I2C_SA")]
pub type CfgI2cSa = crate::Reg<cfg_i2c_sa::CfgI2cSaSpec>;
#[doc = "Slave address register"]
pub mod cfg_i2c_sa;
#[doc = "CFG_I2C_PSC (rw) register accessor: I2C Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_psc`]
module"]
#[doc(alias = "CFG_I2C_PSC")]
pub type CfgI2cPsc = crate::Reg<cfg_i2c_psc::CfgI2cPscSpec>;
#[doc = "I2C Clock Prescaler Register"]
pub mod cfg_i2c_psc;
#[doc = "CFG_I2C_SCLL (rw) register accessor: I2C SCL Low Time Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_scll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_scll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_scll`]
module"]
#[doc(alias = "CFG_I2C_SCLL")]
pub type CfgI2cScll = crate::Reg<cfg_i2c_scll::CfgI2cScllSpec>;
#[doc = "I2C SCL Low Time Register."]
pub mod cfg_i2c_scll;
#[doc = "CFG_I2C_SCLH (rw) register accessor: I2C SCL High Time Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_sclh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_sclh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_sclh`]
module"]
#[doc(alias = "CFG_I2C_SCLH")]
pub type CfgI2cSclh = crate::Reg<cfg_i2c_sclh::CfgI2cSclhSpec>;
#[doc = "I2C SCL High Time Register."]
pub mod cfg_i2c_sclh;
#[doc = "CFG_I2C_SYSTEST (rw) register accessor: I2C System Test Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_systest::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_systest::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_systest`]
module"]
#[doc(alias = "CFG_I2C_SYSTEST")]
pub type CfgI2cSystest = crate::Reg<cfg_i2c_systest::CfgI2cSystestSpec>;
#[doc = "I2C System Test Register."]
pub mod cfg_i2c_systest;
#[doc = "CFG_I2C_BUFSTAT (rw) register accessor: I2C Buffer Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_bufstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_bufstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_bufstat`]
module"]
#[doc(alias = "CFG_I2C_BUFSTAT")]
pub type CfgI2cBufstat = crate::Reg<cfg_i2c_bufstat::CfgI2cBufstatSpec>;
#[doc = "I2C Buffer Status Register."]
pub mod cfg_i2c_bufstat;
#[doc = "CFG_I2C_OA1 (rw) register accessor: I2C Own Address 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_oa1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_oa1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_oa1`]
module"]
#[doc(alias = "CFG_I2C_OA1")]
pub type CfgI2cOa1 = crate::Reg<cfg_i2c_oa1::CfgI2cOa1Spec>;
#[doc = "I2C Own Address 1 Register"]
pub mod cfg_i2c_oa1;
#[doc = "CFG_I2C_OA2 (rw) register accessor: I2C Own Address 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_oa2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_oa2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_oa2`]
module"]
#[doc(alias = "CFG_I2C_OA2")]
pub type CfgI2cOa2 = crate::Reg<cfg_i2c_oa2::CfgI2cOa2Spec>;
#[doc = "I2C Own Address 2 Register"]
pub mod cfg_i2c_oa2;
#[doc = "CFG_I2C_OA3 (rw) register accessor: I2C Own Address 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_oa3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_oa3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_oa3`]
module"]
#[doc(alias = "CFG_I2C_OA3")]
pub type CfgI2cOa3 = crate::Reg<cfg_i2c_oa3::CfgI2cOa3Spec>;
#[doc = "I2C Own Address 3 Register"]
pub mod cfg_i2c_oa3;
#[doc = "CFG_I2C_ACTOA (rw) register accessor: I2C Active Own Address Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_actoa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_actoa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_actoa`]
module"]
#[doc(alias = "CFG_I2C_ACTOA")]
pub type CfgI2cActoa = crate::Reg<cfg_i2c_actoa::CfgI2cActoaSpec>;
#[doc = "I2C Active Own Address Register."]
pub mod cfg_i2c_actoa;
#[doc = "CFG_I2C_SBLOCK (rw) register accessor: I2C Clock Blocking Enable Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_sblock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_sblock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_i2c_sblock`]
module"]
#[doc(alias = "CFG_I2C_SBLOCK")]
pub type CfgI2cSblock = crate::Reg<cfg_i2c_sblock::CfgI2cSblockSpec>;
#[doc = "I2C Clock Blocking Enable Register."]
pub mod cfg_i2c_sblock;
