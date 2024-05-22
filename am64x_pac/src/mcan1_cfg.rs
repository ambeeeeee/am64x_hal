#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcan_wrap__mcan_cfg_vbp__mcan_regs_crel: McanWrap_McanCfgVbp_McanRegsCrel,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_endn: McanWrap_McanCfgVbp_McanRegsEndn,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_cust: McanWrap_McanCfgVbp_McanRegsCust,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_dbtp: McanWrap_McanCfgVbp_McanRegsDbtp,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_test: McanWrap_McanCfgVbp_McanRegsTest,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_rwd: McanWrap_McanCfgVbp_McanRegsRwd,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_cccr: McanWrap_McanCfgVbp_McanRegsCccr,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_nbtp: McanWrap_McanCfgVbp_McanRegsNbtp,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_tscc: McanWrap_McanCfgVbp_McanRegsTscc,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_tscv: McanWrap_McanCfgVbp_McanRegsTscv,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_tocc: McanWrap_McanCfgVbp_McanRegsTocc,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_tocv: McanWrap_McanCfgVbp_McanRegsTocv,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved00: McanWrap_McanCfgVbp_McanRegsReserved00,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved11: McanWrap_McanCfgVbp_McanRegsReserved11,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved22: McanWrap_McanCfgVbp_McanRegsReserved22,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved33: McanWrap_McanCfgVbp_McanRegsReserved33,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_ecr: McanWrap_McanCfgVbp_McanRegsEcr,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_psr: McanWrap_McanCfgVbp_McanRegsPsr,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_tdcr: McanWrap_McanCfgVbp_McanRegsTdcr,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved44: McanWrap_McanCfgVbp_McanRegsReserved44,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_ir: McanWrap_McanCfgVbp_McanRegsIr,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_ie: McanWrap_McanCfgVbp_McanRegsIe,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_ils: McanWrap_McanCfgVbp_McanRegsIls,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_ile: McanWrap_McanCfgVbp_McanRegsIle,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved55: McanWrap_McanCfgVbp_McanRegsReserved55,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved66: McanWrap_McanCfgVbp_McanRegsReserved66,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved77: McanWrap_McanCfgVbp_McanRegsReserved77,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved88: McanWrap_McanCfgVbp_McanRegsReserved88,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved99: McanWrap_McanCfgVbp_McanRegsReserved99,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1010: McanWrap_McanCfgVbp_McanRegsReserved1010,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1111: McanWrap_McanCfgVbp_McanRegsReserved1111,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1212: McanWrap_McanCfgVbp_McanRegsReserved1212,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_gfc: McanWrap_McanCfgVbp_McanRegsGfc,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_sidfc: McanWrap_McanCfgVbp_McanRegsSidfc,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_xidfc: McanWrap_McanCfgVbp_McanRegsXidfc,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1313: McanWrap_McanCfgVbp_McanRegsReserved1313,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_xidam: McanWrap_McanCfgVbp_McanRegsXidam,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_hpms: McanWrap_McanCfgVbp_McanRegsHpms,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_ndat1: McanWrap_McanCfgVbp_McanRegsNdat1,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_ndat2: McanWrap_McanCfgVbp_McanRegsNdat2,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0c: McanWrap_McanCfgVbp_McanRegsRxf0c,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0s: McanWrap_McanCfgVbp_McanRegsRxf0s,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0a: McanWrap_McanCfgVbp_McanRegsRxf0a,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_rxbc: McanWrap_McanCfgVbp_McanRegsRxbc,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1c: McanWrap_McanCfgVbp_McanRegsRxf1c,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1s: McanWrap_McanCfgVbp_McanRegsRxf1s,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1a: McanWrap_McanCfgVbp_McanRegsRxf1a,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_rxesc: McanWrap_McanCfgVbp_McanRegsRxesc,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_txbc: McanWrap_McanCfgVbp_McanRegsTxbc,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_txfqs: McanWrap_McanCfgVbp_McanRegsTxfqs,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_txesc: McanWrap_McanCfgVbp_McanRegsTxesc,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_txbrp: McanWrap_McanCfgVbp_McanRegsTxbrp,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_txbar: McanWrap_McanCfgVbp_McanRegsTxbar,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcr: McanWrap_McanCfgVbp_McanRegsTxbcr,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_txbto: McanWrap_McanCfgVbp_McanRegsTxbto,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcf: McanWrap_McanCfgVbp_McanRegsTxbcf,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_txbtie: McanWrap_McanCfgVbp_McanRegsTxbtie,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcie: McanWrap_McanCfgVbp_McanRegsTxbcie,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1414: McanWrap_McanCfgVbp_McanRegsReserved1414,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1515: McanWrap_McanCfgVbp_McanRegsReserved1515,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_txefc: McanWrap_McanCfgVbp_McanRegsTxefc,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_txefs: McanWrap_McanCfgVbp_McanRegsTxefs,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_txefa: McanWrap_McanCfgVbp_McanRegsTxefa,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1616: McanWrap_McanCfgVbp_McanRegsReserved1616,
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserv_upper256: McanWrap_McanCfgVbp_McanRegsReservUpper256,
}
impl RegisterBlock {
    #[doc = "0x00 - Release dependent constant (version + date)"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_crel(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsCrel {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_crel
    }
    #[doc = "0x04 - Constant 0x8765 4321"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_endn(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsEndn {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_endn
    }
    #[doc = "0x08 - Optional customer-specific register"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_cust(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsCust {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_cust
    }
    #[doc = "0x0c - Configuration of data phase bit timing, transmitter delay compensation enable"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_dbtp(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsDbtp {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_dbtp
    }
    #[doc = "0x10 - Test mode selection"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_test(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTest {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_test
    }
    #[doc = "0x14 - Monitors the READY output of the Message RAM"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_rwd(&self) -> &McanWrap_McanCfgVbp_McanRegsRwd {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_rwd
    }
    #[doc = "0x18 - Operation mode configuration"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_cccr(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsCccr {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_cccr
    }
    #[doc = "0x1c - Configuration of arbitration phase bit timing"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_nbtp(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsNbtp {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_nbtp
    }
    #[doc = "0x20 - Timestamp counter prescaler setting, selection of internal/external timestamp vector"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_tscc(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTscc {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_tscc
    }
    #[doc = "0x24 - Read/reset timestamp counter"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_tscv(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTscv {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_tscv
    }
    #[doc = "0x28 - Configuration of timeout period, selection of timeout counter operation mode"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_tocc(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTocc {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_tocc
    }
    #[doc = "0x2c - Read/reset timeout counter"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_tocv(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTocv {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_tocv
    }
    #[doc = "0x30 - Reserved field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved00(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReserved00 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved00
    }
    #[doc = "0x34 - Reserved field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved11(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReserved11 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved11
    }
    #[doc = "0x38 - Reserved field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved22(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReserved22 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved22
    }
    #[doc = "0x3c - Reserved field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved33(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReserved33 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved33
    }
    #[doc = "0x40 - State of Rx/Tx Error Counter, CAN Error Logging"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_ecr(&self) -> &McanWrap_McanCfgVbp_McanRegsEcr {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_ecr
    }
    #[doc = "0x44 - CAN protocol controller status, transmitter delay compensation value"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_psr(&self) -> &McanWrap_McanCfgVbp_McanRegsPsr {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_psr
    }
    #[doc = "0x48 - configuration of transmitter delay compensation offset and filter window length"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_tdcr(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTdcr {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_tdcr
    }
    #[doc = "0x4c - Reserved field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved44(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReserved44 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved44
    }
    #[doc = "0x50 - Interrupt flags"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_ir(&self) -> &McanWrap_McanCfgVbp_McanRegsIr {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_ir
    }
    #[doc = "0x54 - Interrupt enable/disable"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_ie(&self) -> &McanWrap_McanCfgVbp_McanRegsIe {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_ie
    }
    #[doc = "0x58 - Interrupt line select (m_can_int0 or m_can_int1)"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_ils(&self) -> &McanWrap_McanCfgVbp_McanRegsIls {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_ils
    }
    #[doc = "0x5c - Enable/disable interrupt lines m_can_int0 / m_can_int1"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_ile(&self) -> &McanWrap_McanCfgVbp_McanRegsIle {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_ile
    }
    #[doc = "0x60 - Reserved field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved55(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReserved55 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved55
    }
    #[doc = "0x64 - Reserved field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved66(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReserved66 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved66
    }
    #[doc = "0x68 - Reserved field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved77(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReserved77 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved77
    }
    #[doc = "0x6c - Reserved field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved88(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReserved88 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved88
    }
    #[doc = "0x70 - Reserved field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved99(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReserved99 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved99
    }
    #[doc = "0x74 - Reserved field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1010(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReserved1010 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1010
    }
    #[doc = "0x78 - Reserved field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1111(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReserved1111 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1111
    }
    #[doc = "0x7c - Reserved field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1212(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReserved1212 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1212
    }
    #[doc = "0x80 - Handling of non-matching frames and remote frames"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_gfc(&self) -> &McanWrap_McanCfgVbp_McanRegsGfc {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_gfc
    }
    #[doc = "0x84 - Number of filter elements, pointer to start of filter list"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_sidfc(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsSidfc {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_sidfc
    }
    #[doc = "0x88 - Number of filter elements, pointer to start of filter list"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_xidfc(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsXidfc {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_xidfc
    }
    #[doc = "0x8c - Reserved field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1313(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReserved1313 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1313
    }
    #[doc = "0x90 - 29-bit logical AND mask for J1939"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_xidam(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsXidam {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_xidam
    }
    #[doc = "0x94 - Status monitoring of incoming high priority messages"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_hpms(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsHpms {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_hpms
    }
    #[doc = "0x98 - NewDat flags of dedicated Rx buffers 0-31"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_ndat1(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsNdat1 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_ndat1
    }
    #[doc = "0x9c - NewDat flags of dedicated Rx buffers 32-63"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_ndat2(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsNdat2 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_ndat2
    }
    #[doc = "0xa0 - FIFO 0 operation mode, watermark, size and start address"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0c(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsRxf0c {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0c
    }
    #[doc = "0xa4 - FIFO 0 message lost/full indication, put index, get index and fill level"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0s(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsRxf0s {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0s
    }
    #[doc = "0xa8 - FIFO 0 acknowledge last index of read buffers, updates get index and fill level"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0a(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsRxf0a {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0a
    }
    #[doc = "0xac - Start address of Rx buffer section"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_rxbc(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsRxbc {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_rxbc
    }
    #[doc = "0xb0 - FIFO 1 operation mode, watermark, size and start address"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1c(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsRxf1c {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1c
    }
    #[doc = "0xb4 - FIFO 1 message lost/full indication, put index, get index and fill level"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1s(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsRxf1s {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1s
    }
    #[doc = "0xb8 - FIFO 1 acknowledge last index of read buffers, updates get index and fill level"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1a(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsRxf1a {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1a
    }
    #[doc = "0xbc - Configure data field size for storage of accepted frames"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_rxesc(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsRxesc {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_rxesc
    }
    #[doc = "0xc0 - Configure Tx FIFO/Queue mode, Tx FIFO/Queue size, number of dedicated Tx buffers, Tx buffer start address"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_txbc(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTxbc {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_txbc
    }
    #[doc = "0xc4 - Tx FIFO/Queue full indication and put index, Tx FIFO get index and fill level"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_txfqs(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTxfqs {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_txfqs
    }
    #[doc = "0xc8 - Configure data field size for frame transmission"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_txesc(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTxesc {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_txesc
    }
    #[doc = "0xcc - Tx buffers with pending transmission request"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_txbrp(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTxbrp {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_txbrp
    }
    #[doc = "0xd0 - Add transmission requests"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_txbar(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTxbar {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_txbar
    }
    #[doc = "0xd4 - Request cancellation of pending transmissions"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcr(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTxbcr {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcr
    }
    #[doc = "0xd8 - Signals successful transmissions, set when corresponding TXBRP flag is cleared"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_txbto(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTxbto {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_txbto
    }
    #[doc = "0xdc - Signals successful transmit cancellation, set when corresponding TXBRP flag is cleared after cancellation request"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcf(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTxbcf {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcf
    }
    #[doc = "0xe0 - Enable transmit interrupts for selected Tx buffers"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_txbtie(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTxbtie {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_txbtie
    }
    #[doc = "0xe4 - Enable cancellation finished interrupts for selected Tx buffers"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcie(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTxbcie {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcie
    }
    #[doc = "0xe8 - Reserved Field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1414(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReserved1414 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1414
    }
    #[doc = "0xec - Reserved Field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1515(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReserved1515 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1515
    }
    #[doc = "0xf0 - Tx event FIFO watermark, size and start address"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_txefc(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTxefc {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_txefc
    }
    #[doc = "0xf4 - Tx event FIFO element lost/full indication, put index, get index, and fill level"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_txefs(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTxefs {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_txefs
    }
    #[doc = "0xf8 - Tx event FIFO acknowledge last index of read elements, updates get index and fill level"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_txefa(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsTxefa {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_txefa
    }
    #[doc = "0xfc - Reserved Field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1616(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReserved1616 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1616
    }
    #[doc = "0x100 - Reserved Field"]
    #[inline(always)]
    pub const fn mcan_wrap__mcan_cfg_vbp__mcan_regs_reserv_upper256(
        &self,
    ) -> &McanWrap_McanCfgVbp_McanRegsReservUpper256 {
        &self.mcan_wrap__mcan_cfg_vbp__mcan_regs_reserv_upper256
    }
}
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_CREL (rw) register accessor: Release dependent constant (version + date)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_crel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_crel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_crel`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_CREL")]
pub type McanWrap_McanCfgVbp_McanRegsCrel =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_crel::McanWrap_McanCfgVbp_McanRegsCrelSpec>;
#[doc = "Release dependent constant (version + date)"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_crel;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ENDN (rw) register accessor: Constant 0x8765 4321\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_endn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_endn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_endn`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ENDN")]
pub type McanWrap_McanCfgVbp_McanRegsEndn =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_endn::McanWrap_McanCfgVbp_McanRegsEndnSpec>;
#[doc = "Constant 0x8765 4321"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_endn;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_CUST (rw) register accessor: Optional customer-specific register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_cust::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_cust::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_cust`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_CUST")]
pub type McanWrap_McanCfgVbp_McanRegsCust =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_cust::McanWrap_McanCfgVbp_McanRegsCustSpec>;
#[doc = "Optional customer-specific register"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_cust;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_DBTP (rw) register accessor: Configuration of data phase bit timing, transmitter delay compensation enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_dbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_dbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_dbtp`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_DBTP")]
pub type McanWrap_McanCfgVbp_McanRegsDbtp =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_dbtp::McanWrap_McanCfgVbp_McanRegsDbtpSpec>;
#[doc = "Configuration of data phase bit timing, transmitter delay compensation enable"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_dbtp;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TEST (rw) register accessor: Test mode selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_test`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TEST")]
pub type McanWrap_McanCfgVbp_McanRegsTest =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_test::McanWrap_McanCfgVbp_McanRegsTestSpec>;
#[doc = "Test mode selection"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_test;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RWD (rw) register accessor: Monitors the READY output of the Message RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_rwd`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RWD")]
pub type McanWrap_McanCfgVbp_McanRegsRwd =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_rwd::McanWrap_McanCfgVbp_McanRegsRwdSpec>;
#[doc = "Monitors the READY output of the Message RAM"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_rwd;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_CCCR (rw) register accessor: Operation mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_cccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_cccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_cccr`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_CCCR")]
pub type McanWrap_McanCfgVbp_McanRegsCccr =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_cccr::McanWrap_McanCfgVbp_McanRegsCccrSpec>;
#[doc = "Operation mode configuration"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_cccr;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_NBTP (rw) register accessor: Configuration of arbitration phase bit timing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_nbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_nbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_nbtp`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_NBTP")]
pub type McanWrap_McanCfgVbp_McanRegsNbtp =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_nbtp::McanWrap_McanCfgVbp_McanRegsNbtpSpec>;
#[doc = "Configuration of arbitration phase bit timing"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_nbtp;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TSCC (rw) register accessor: Timestamp counter prescaler setting, selection of internal/external timestamp vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tscc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tscc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_tscc`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TSCC")]
pub type McanWrap_McanCfgVbp_McanRegsTscc =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_tscc::McanWrap_McanCfgVbp_McanRegsTsccSpec>;
#[doc = "Timestamp counter prescaler setting, selection of internal/external timestamp vector"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_tscc;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TSCV (rw) register accessor: Read/reset timestamp counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tscv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tscv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_tscv`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TSCV")]
pub type McanWrap_McanCfgVbp_McanRegsTscv =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_tscv::McanWrap_McanCfgVbp_McanRegsTscvSpec>;
#[doc = "Read/reset timestamp counter"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_tscv;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TOCC (rw) register accessor: Configuration of timeout period, selection of timeout counter operation mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tocc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tocc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_tocc`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TOCC")]
pub type McanWrap_McanCfgVbp_McanRegsTocc =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_tocc::McanWrap_McanCfgVbp_McanRegsToccSpec>;
#[doc = "Configuration of timeout period, selection of timeout counter operation mode"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_tocc;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TOCV (rw) register accessor: Read/reset timeout counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tocv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tocv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_tocv`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TOCV")]
pub type McanWrap_McanCfgVbp_McanRegsTocv =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_tocv::McanWrap_McanCfgVbp_McanRegsTocvSpec>;
#[doc = "Read/reset timeout counter"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_tocv;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved00 (rw) register accessor: Reserved field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved00::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved00::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved00`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved00")]
pub type McanWrap_McanCfgVbp_McanRegsReserved00 = crate::Reg<
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved00::McanWrap_McanCfgVbp_McanRegsReserved00Spec,
>;
#[doc = "Reserved field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved00;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved11 (rw) register accessor: Reserved field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved11`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved11")]
pub type McanWrap_McanCfgVbp_McanRegsReserved11 = crate::Reg<
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved11::McanWrap_McanCfgVbp_McanRegsReserved11Spec,
>;
#[doc = "Reserved field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved11;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved22 (rw) register accessor: Reserved field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved22`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved22")]
pub type McanWrap_McanCfgVbp_McanRegsReserved22 = crate::Reg<
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved22::McanWrap_McanCfgVbp_McanRegsReserved22Spec,
>;
#[doc = "Reserved field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved22;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved33 (rw) register accessor: Reserved field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved33`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved33")]
pub type McanWrap_McanCfgVbp_McanRegsReserved33 = crate::Reg<
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved33::McanWrap_McanCfgVbp_McanRegsReserved33Spec,
>;
#[doc = "Reserved field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved33;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ECR (rw) register accessor: State of Rx/Tx Error Counter, CAN Error Logging\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_ecr`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ECR")]
pub type McanWrap_McanCfgVbp_McanRegsEcr =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_ecr::McanWrap_McanCfgVbp_McanRegsEcrSpec>;
#[doc = "State of Rx/Tx Error Counter, CAN Error Logging"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_ecr;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_PSR (rw) register accessor: CAN protocol controller status, transmitter delay compensation value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_psr`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_PSR")]
pub type McanWrap_McanCfgVbp_McanRegsPsr =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_psr::McanWrap_McanCfgVbp_McanRegsPsrSpec>;
#[doc = "CAN protocol controller status, transmitter delay compensation value"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_psr;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TDCR (rw) register accessor: configuration of transmitter delay compensation offset and filter window length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_tdcr`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TDCR")]
pub type McanWrap_McanCfgVbp_McanRegsTdcr =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_tdcr::McanWrap_McanCfgVbp_McanRegsTdcrSpec>;
#[doc = "configuration of transmitter delay compensation offset and filter window length"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_tdcr;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved44 (rw) register accessor: Reserved field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved44`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved44")]
pub type McanWrap_McanCfgVbp_McanRegsReserved44 = crate::Reg<
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved44::McanWrap_McanCfgVbp_McanRegsReserved44Spec,
>;
#[doc = "Reserved field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved44;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_IR (rw) register accessor: Interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_ir`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_IR")]
pub type McanWrap_McanCfgVbp_McanRegsIr =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_ir::McanWrap_McanCfgVbp_McanRegsIrSpec>;
#[doc = "Interrupt flags"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_ir;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_IE (rw) register accessor: Interrupt enable/disable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_ie`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_IE")]
pub type McanWrap_McanCfgVbp_McanRegsIe =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_ie::McanWrap_McanCfgVbp_McanRegsIeSpec>;
#[doc = "Interrupt enable/disable"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_ie;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ILS (rw) register accessor: Interrupt line select (m_can_int0 or m_can_int1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ils::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ils::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_ils`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ILS")]
pub type McanWrap_McanCfgVbp_McanRegsIls =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_ils::McanWrap_McanCfgVbp_McanRegsIlsSpec>;
#[doc = "Interrupt line select (m_can_int0 or m_can_int1)"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_ils;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ILE (rw) register accessor: Enable/disable interrupt lines m_can_int0 / m_can_int1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ile::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ile::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_ile`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ILE")]
pub type McanWrap_McanCfgVbp_McanRegsIle =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_ile::McanWrap_McanCfgVbp_McanRegsIleSpec>;
#[doc = "Enable/disable interrupt lines m_can_int0 / m_can_int1"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_ile;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved55 (rw) register accessor: Reserved field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved55`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved55")]
pub type McanWrap_McanCfgVbp_McanRegsReserved55 = crate::Reg<
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved55::McanWrap_McanCfgVbp_McanRegsReserved55Spec,
>;
#[doc = "Reserved field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved55;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved66 (rw) register accessor: Reserved field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved66::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved66::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved66`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved66")]
pub type McanWrap_McanCfgVbp_McanRegsReserved66 = crate::Reg<
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved66::McanWrap_McanCfgVbp_McanRegsReserved66Spec,
>;
#[doc = "Reserved field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved66;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved77 (rw) register accessor: Reserved field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved77::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved77::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved77`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved77")]
pub type McanWrap_McanCfgVbp_McanRegsReserved77 = crate::Reg<
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved77::McanWrap_McanCfgVbp_McanRegsReserved77Spec,
>;
#[doc = "Reserved field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved77;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved88 (rw) register accessor: Reserved field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved88::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved88::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved88`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved88")]
pub type McanWrap_McanCfgVbp_McanRegsReserved88 = crate::Reg<
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved88::McanWrap_McanCfgVbp_McanRegsReserved88Spec,
>;
#[doc = "Reserved field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved88;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved99 (rw) register accessor: Reserved field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved99::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved99::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved99`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved99")]
pub type McanWrap_McanCfgVbp_McanRegsReserved99 = crate::Reg<
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved99::McanWrap_McanCfgVbp_McanRegsReserved99Spec,
>;
#[doc = "Reserved field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved99;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved1010 (rw) register accessor: Reserved field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1010::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1010::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1010`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved1010")]
pub type McanWrap_McanCfgVbp_McanRegsReserved1010 = crate::Reg<
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1010::McanWrap_McanCfgVbp_McanRegsReserved1010Spec,
>;
#[doc = "Reserved field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1010;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved1111 (rw) register accessor: Reserved field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1111::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1111::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1111`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved1111")]
pub type McanWrap_McanCfgVbp_McanRegsReserved1111 = crate::Reg<
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1111::McanWrap_McanCfgVbp_McanRegsReserved1111Spec,
>;
#[doc = "Reserved field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1111;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved1212 (rw) register accessor: Reserved field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1212::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1212::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1212`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved1212")]
pub type McanWrap_McanCfgVbp_McanRegsReserved1212 = crate::Reg<
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1212::McanWrap_McanCfgVbp_McanRegsReserved1212Spec,
>;
#[doc = "Reserved field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1212;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_GFC (rw) register accessor: Handling of non-matching frames and remote frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_gfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_gfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_gfc`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_GFC")]
pub type McanWrap_McanCfgVbp_McanRegsGfc =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_gfc::McanWrap_McanCfgVbp_McanRegsGfcSpec>;
#[doc = "Handling of non-matching frames and remote frames"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_gfc;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_SIDFC (rw) register accessor: Number of filter elements, pointer to start of filter list\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_sidfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_sidfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_sidfc`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_SIDFC")]
pub type McanWrap_McanCfgVbp_McanRegsSidfc =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_sidfc::McanWrap_McanCfgVbp_McanRegsSidfcSpec>;
#[doc = "Number of filter elements, pointer to start of filter list"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_sidfc;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_XIDFC (rw) register accessor: Number of filter elements, pointer to start of filter list\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_xidfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_xidfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_xidfc`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_XIDFC")]
pub type McanWrap_McanCfgVbp_McanRegsXidfc =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_xidfc::McanWrap_McanCfgVbp_McanRegsXidfcSpec>;
#[doc = "Number of filter elements, pointer to start of filter list"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_xidfc;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved1313 (rw) register accessor: Reserved field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1313::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1313::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1313`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved1313")]
pub type McanWrap_McanCfgVbp_McanRegsReserved1313 = crate::Reg<
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1313::McanWrap_McanCfgVbp_McanRegsReserved1313Spec,
>;
#[doc = "Reserved field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1313;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_XIDAM (rw) register accessor: 29-bit logical AND mask for J1939\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_xidam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_xidam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_xidam`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_XIDAM")]
pub type McanWrap_McanCfgVbp_McanRegsXidam =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_xidam::McanWrap_McanCfgVbp_McanRegsXidamSpec>;
#[doc = "29-bit logical AND mask for J1939"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_xidam;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_HPMS (rw) register accessor: Status monitoring of incoming high priority messages\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_hpms::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_hpms::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_hpms`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_HPMS")]
pub type McanWrap_McanCfgVbp_McanRegsHpms =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_hpms::McanWrap_McanCfgVbp_McanRegsHpmsSpec>;
#[doc = "Status monitoring of incoming high priority messages"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_hpms;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_NDAT1 (rw) register accessor: NewDat flags of dedicated Rx buffers 0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ndat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ndat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_ndat1`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_NDAT1")]
pub type McanWrap_McanCfgVbp_McanRegsNdat1 =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_ndat1::McanWrap_McanCfgVbp_McanRegsNdat1Spec>;
#[doc = "NewDat flags of dedicated Rx buffers 0-31"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_ndat1;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_NDAT2 (rw) register accessor: NewDat flags of dedicated Rx buffers 32-63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ndat2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ndat2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_ndat2`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_NDAT2")]
pub type McanWrap_McanCfgVbp_McanRegsNdat2 =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_ndat2::McanWrap_McanCfgVbp_McanRegsNdat2Spec>;
#[doc = "NewDat flags of dedicated Rx buffers 32-63"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_ndat2;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF0C (rw) register accessor: FIFO 0 operation mode, watermark, size and start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0c`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF0C")]
pub type McanWrap_McanCfgVbp_McanRegsRxf0c =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0c::McanWrap_McanCfgVbp_McanRegsRxf0cSpec>;
#[doc = "FIFO 0 operation mode, watermark, size and start address"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0c;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF0S (rw) register accessor: FIFO 0 message lost/full indication, put index, get index and fill level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0s`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF0S")]
pub type McanWrap_McanCfgVbp_McanRegsRxf0s =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0s::McanWrap_McanCfgVbp_McanRegsRxf0sSpec>;
#[doc = "FIFO 0 message lost/full indication, put index, get index and fill level"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0s;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF0A (rw) register accessor: FIFO 0 acknowledge last index of read buffers, updates get index and fill level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0a`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF0A")]
pub type McanWrap_McanCfgVbp_McanRegsRxf0a =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0a::McanWrap_McanCfgVbp_McanRegsRxf0aSpec>;
#[doc = "FIFO 0 acknowledge last index of read buffers, updates get index and fill level"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0a;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXBC (rw) register accessor: Start address of Rx buffer section\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_rxbc`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXBC")]
pub type McanWrap_McanCfgVbp_McanRegsRxbc =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_rxbc::McanWrap_McanCfgVbp_McanRegsRxbcSpec>;
#[doc = "Start address of Rx buffer section"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_rxbc;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF1C (rw) register accessor: FIFO 1 operation mode, watermark, size and start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1c`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF1C")]
pub type McanWrap_McanCfgVbp_McanRegsRxf1c =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1c::McanWrap_McanCfgVbp_McanRegsRxf1cSpec>;
#[doc = "FIFO 1 operation mode, watermark, size and start address"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1c;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF1S (rw) register accessor: FIFO 1 message lost/full indication, put index, get index and fill level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1s`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF1S")]
pub type McanWrap_McanCfgVbp_McanRegsRxf1s =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1s::McanWrap_McanCfgVbp_McanRegsRxf1sSpec>;
#[doc = "FIFO 1 message lost/full indication, put index, get index and fill level"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1s;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF1A (rw) register accessor: FIFO 1 acknowledge last index of read buffers, updates get index and fill level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1a`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF1A")]
pub type McanWrap_McanCfgVbp_McanRegsRxf1a =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1a::McanWrap_McanCfgVbp_McanRegsRxf1aSpec>;
#[doc = "FIFO 1 acknowledge last index of read buffers, updates get index and fill level"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1a;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXESC (rw) register accessor: Configure data field size for storage of accepted frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_rxesc`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXESC")]
pub type McanWrap_McanCfgVbp_McanRegsRxesc =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_rxesc::McanWrap_McanCfgVbp_McanRegsRxescSpec>;
#[doc = "Configure data field size for storage of accepted frames"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_rxesc;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBC (rw) register accessor: Configure Tx FIFO/Queue mode, Tx FIFO/Queue size, number of dedicated Tx buffers, Tx buffer start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_txbc`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBC")]
pub type McanWrap_McanCfgVbp_McanRegsTxbc =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_txbc::McanWrap_McanCfgVbp_McanRegsTxbcSpec>;
#[doc = "Configure Tx FIFO/Queue mode, Tx FIFO/Queue size, number of dedicated Tx buffers, Tx buffer start address"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_txbc;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXFQS (rw) register accessor: Tx FIFO/Queue full indication and put index, Tx FIFO get index and fill level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txfqs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txfqs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_txfqs`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXFQS")]
pub type McanWrap_McanCfgVbp_McanRegsTxfqs =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_txfqs::McanWrap_McanCfgVbp_McanRegsTxfqsSpec>;
#[doc = "Tx FIFO/Queue full indication and put index, Tx FIFO get index and fill level"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_txfqs;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXESC (rw) register accessor: Configure data field size for frame transmission\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_txesc`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXESC")]
pub type McanWrap_McanCfgVbp_McanRegsTxesc =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_txesc::McanWrap_McanCfgVbp_McanRegsTxescSpec>;
#[doc = "Configure data field size for frame transmission"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_txesc;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBRP (rw) register accessor: Tx buffers with pending transmission request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbrp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbrp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_txbrp`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBRP")]
pub type McanWrap_McanCfgVbp_McanRegsTxbrp =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_txbrp::McanWrap_McanCfgVbp_McanRegsTxbrpSpec>;
#[doc = "Tx buffers with pending transmission request"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_txbrp;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBAR (rw) register accessor: Add transmission requests\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_txbar`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBAR")]
pub type McanWrap_McanCfgVbp_McanRegsTxbar =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_txbar::McanWrap_McanCfgVbp_McanRegsTxbarSpec>;
#[doc = "Add transmission requests"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_txbar;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBCR (rw) register accessor: Request cancellation of pending transmissions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcr`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBCR")]
pub type McanWrap_McanCfgVbp_McanRegsTxbcr =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcr::McanWrap_McanCfgVbp_McanRegsTxbcrSpec>;
#[doc = "Request cancellation of pending transmissions"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcr;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBTO (rw) register accessor: Signals successful transmissions, set when corresponding TXBRP flag is cleared\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbto::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbto::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_txbto`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBTO")]
pub type McanWrap_McanCfgVbp_McanRegsTxbto =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_txbto::McanWrap_McanCfgVbp_McanRegsTxbtoSpec>;
#[doc = "Signals successful transmissions, set when corresponding TXBRP flag is cleared"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_txbto;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBCF (rw) register accessor: Signals successful transmit cancellation, set when corresponding TXBRP flag is cleared after cancellation request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcf`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBCF")]
pub type McanWrap_McanCfgVbp_McanRegsTxbcf =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcf::McanWrap_McanCfgVbp_McanRegsTxbcfSpec>;
#[doc = "Signals successful transmit cancellation, set when corresponding TXBRP flag is cleared after cancellation request"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcf;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBTIE (rw) register accessor: Enable transmit interrupts for selected Tx buffers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbtie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbtie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_txbtie`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBTIE")]
pub type McanWrap_McanCfgVbp_McanRegsTxbtie =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_txbtie::McanWrap_McanCfgVbp_McanRegsTxbtieSpec>;
#[doc = "Enable transmit interrupts for selected Tx buffers"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_txbtie;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBCIE (rw) register accessor: Enable cancellation finished interrupts for selected Tx buffers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcie`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBCIE")]
pub type McanWrap_McanCfgVbp_McanRegsTxbcie =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcie::McanWrap_McanCfgVbp_McanRegsTxbcieSpec>;
#[doc = "Enable cancellation finished interrupts for selected Tx buffers"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_txbcie;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved1414 (rw) register accessor: Reserved Field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1414::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1414::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1414`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved1414")]
pub type McanWrap_McanCfgVbp_McanRegsReserved1414 = crate::Reg<
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1414::McanWrap_McanCfgVbp_McanRegsReserved1414Spec,
>;
#[doc = "Reserved Field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1414;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved1515 (rw) register accessor: Reserved Field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1515::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1515::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1515`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved1515")]
pub type McanWrap_McanCfgVbp_McanRegsReserved1515 = crate::Reg<
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1515::McanWrap_McanCfgVbp_McanRegsReserved1515Spec,
>;
#[doc = "Reserved Field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1515;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXEFC (rw) register accessor: Tx event FIFO watermark, size and start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_txefc`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXEFC")]
pub type McanWrap_McanCfgVbp_McanRegsTxefc =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_txefc::McanWrap_McanCfgVbp_McanRegsTxefcSpec>;
#[doc = "Tx event FIFO watermark, size and start address"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_txefc;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXEFS (rw) register accessor: Tx event FIFO element lost/full indication, put index, get index, and fill level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_txefs`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXEFS")]
pub type McanWrap_McanCfgVbp_McanRegsTxefs =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_txefs::McanWrap_McanCfgVbp_McanRegsTxefsSpec>;
#[doc = "Tx event FIFO element lost/full indication, put index, get index, and fill level"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_txefs;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXEFA (rw) register accessor: Tx event FIFO acknowledge last index of read elements, updates get index and fill level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_txefa`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXEFA")]
pub type McanWrap_McanCfgVbp_McanRegsTxefa =
    crate::Reg<mcan_wrap__mcan_cfg_vbp__mcan_regs_txefa::McanWrap_McanCfgVbp_McanRegsTxefaSpec>;
#[doc = "Tx event FIFO acknowledge last index of read elements, updates get index and fill level"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_txefa;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved1616 (rw) register accessor: Reserved Field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1616::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1616::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1616`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved1616")]
pub type McanWrap_McanCfgVbp_McanRegsReserved1616 = crate::Reg<
    mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1616::McanWrap_McanCfgVbp_McanRegsReserved1616Spec,
>;
#[doc = "Reserved Field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1616;
#[doc = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ReservUpper256 (rw) register accessor: Reserved Field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserv_upper256::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserv_upper256::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_wrap__mcan_cfg_vbp__mcan_regs_reserv_upper256`]
module"]
#[doc(alias = "MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ReservUpper256")]
pub type McanWrap_McanCfgVbp_McanRegsReservUpper256 = crate :: Reg < mcan_wrap__mcan_cfg_vbp__mcan_regs_reserv_upper256 :: McanWrap_McanCfgVbp_McanRegsReservUpper256Spec > ;
#[doc = "Reserved Field"]
pub mod mcan_wrap__mcan_cfg_vbp__mcan_regs_reserv_upper256;
