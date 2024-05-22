#[doc = "Register `ECC_AGGR_CORE0_REGS_ded_status_reg0` reader"]
pub type R = crate::R<EccAggrCore0RegsDedStatusReg0Spec>;
#[doc = "Register `ECC_AGGR_CORE0_REGS_ded_status_reg0` writer"]
pub type W = crate::W<EccAggrCore0RegsDedStatusReg0Spec>;
#[doc = "Field `CPU0_A53_DUAL_U_IDATA_SPRAM_BANK0_LO_ECC_SVBUS_PEND` reader - 0:0\\]
Interrupt Pending Status for cpu0_a53_dual_u_idata_spram_bank0_lo_ecc_svbus_pend"]
pub type Cpu0A53DualUIdataSpramBank0LoEccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_IDATA_SPRAM_BANK0_LO_ECC_SVBUS_PEND` writer - 0:0\\]
Interrupt Pending Status for cpu0_a53_dual_u_idata_spram_bank0_lo_ecc_svbus_pend"]
pub type Cpu0A53DualUIdataSpramBank0LoEccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_IDATA_SPRAM_BANK0_HI_ECC_SVBUS_PEND` reader - 1:1\\]
Interrupt Pending Status for cpu0_a53_dual_u_idata_spram_bank0_hi_ecc_svbus_pend"]
pub type Cpu0A53DualUIdataSpramBank0HiEccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_IDATA_SPRAM_BANK0_HI_ECC_SVBUS_PEND` writer - 1:1\\]
Interrupt Pending Status for cpu0_a53_dual_u_idata_spram_bank0_hi_ecc_svbus_pend"]
pub type Cpu0A53DualUIdataSpramBank0HiEccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_IDATA_SPRAM_BANK1_LO_ECC_SVBUS_PEND` reader - 2:2\\]
Interrupt Pending Status for cpu0_a53_dual_u_idata_spram_bank1_lo_ecc_svbus_pend"]
pub type Cpu0A53DualUIdataSpramBank1LoEccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_IDATA_SPRAM_BANK1_LO_ECC_SVBUS_PEND` writer - 2:2\\]
Interrupt Pending Status for cpu0_a53_dual_u_idata_spram_bank1_lo_ecc_svbus_pend"]
pub type Cpu0A53DualUIdataSpramBank1LoEccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_IDATA_SPRAM_BANK1_HI_ECC_SVBUS_PEND` reader - 3:3\\]
Interrupt Pending Status for cpu0_a53_dual_u_idata_spram_bank1_hi_ecc_svbus_pend"]
pub type Cpu0A53DualUIdataSpramBank1HiEccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_IDATA_SPRAM_BANK1_HI_ECC_SVBUS_PEND` writer - 3:3\\]
Interrupt Pending Status for cpu0_a53_dual_u_idata_spram_bank1_hi_ecc_svbus_pend"]
pub type Cpu0A53DualUIdataSpramBank1HiEccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_ITAG_SPRAM_RAM0_ECC_SVBUS_PEND` reader - 4:4\\]
Interrupt Pending Status for cpu0_a53_dual_u_itag_spram_ram0_ecc_svbus_pend"]
pub type Cpu0A53DualUItagSpramRam0EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_ITAG_SPRAM_RAM0_ECC_SVBUS_PEND` writer - 4:4\\]
Interrupt Pending Status for cpu0_a53_dual_u_itag_spram_ram0_ecc_svbus_pend"]
pub type Cpu0A53DualUItagSpramRam0EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_ITAG_SPRAM_RAM1_ECC_SVBUS_PEND` reader - 5:5\\]
Interrupt Pending Status for cpu0_a53_dual_u_itag_spram_ram1_ecc_svbus_pend"]
pub type Cpu0A53DualUItagSpramRam1EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_ITAG_SPRAM_RAM1_ECC_SVBUS_PEND` writer - 5:5\\]
Interrupt Pending Status for cpu0_a53_dual_u_itag_spram_ram1_ecc_svbus_pend"]
pub type Cpu0A53DualUItagSpramRam1EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_DDATA_SPRAM_BANK0_ECC_SVBUS_PEND` reader - 6:6\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank0_ecc_svbus_pend"]
pub type Cpu0A53DualUDdataSpramBank0EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_DDATA_SPRAM_BANK0_ECC_SVBUS_PEND` writer - 6:6\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank0_ecc_svbus_pend"]
pub type Cpu0A53DualUDdataSpramBank0EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_DDATA_SPRAM_BANK1_ECC_SVBUS_PEND` reader - 7:7\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank1_ecc_svbus_pend"]
pub type Cpu0A53DualUDdataSpramBank1EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_DDATA_SPRAM_BANK1_ECC_SVBUS_PEND` writer - 7:7\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank1_ecc_svbus_pend"]
pub type Cpu0A53DualUDdataSpramBank1EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_DDATA_SPRAM_BANK2_ECC_SVBUS_PEND` reader - 8:8\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank2_ecc_svbus_pend"]
pub type Cpu0A53DualUDdataSpramBank2EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_DDATA_SPRAM_BANK2_ECC_SVBUS_PEND` writer - 8:8\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank2_ecc_svbus_pend"]
pub type Cpu0A53DualUDdataSpramBank2EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_DDATA_SPRAM_BANK3_ECC_SVBUS_PEND` reader - 9:9\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank3_ecc_svbus_pend"]
pub type Cpu0A53DualUDdataSpramBank3EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_DDATA_SPRAM_BANK3_ECC_SVBUS_PEND` writer - 9:9\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank3_ecc_svbus_pend"]
pub type Cpu0A53DualUDdataSpramBank3EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_DDATA_SPRAM_BANK4_ECC_SVBUS_PEND` reader - 10:10\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank4_ecc_svbus_pend"]
pub type Cpu0A53DualUDdataSpramBank4EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_DDATA_SPRAM_BANK4_ECC_SVBUS_PEND` writer - 10:10\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank4_ecc_svbus_pend"]
pub type Cpu0A53DualUDdataSpramBank4EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_DDATA_SPRAM_BANK5_ECC_SVBUS_PEND` reader - 11:11\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank5_ecc_svbus_pend"]
pub type Cpu0A53DualUDdataSpramBank5EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_DDATA_SPRAM_BANK5_ECC_SVBUS_PEND` writer - 11:11\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank5_ecc_svbus_pend"]
pub type Cpu0A53DualUDdataSpramBank5EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_DDATA_SPRAM_BANK6_ECC_SVBUS_PEND` reader - 12:12\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank6_ecc_svbus_pend"]
pub type Cpu0A53DualUDdataSpramBank6EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_DDATA_SPRAM_BANK6_ECC_SVBUS_PEND` writer - 12:12\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank6_ecc_svbus_pend"]
pub type Cpu0A53DualUDdataSpramBank6EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_DDATA_SPRAM_BANK7_ECC_SVBUS_PEND` reader - 13:13\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank7_ecc_svbus_pend"]
pub type Cpu0A53DualUDdataSpramBank7EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_DDATA_SPRAM_BANK7_ECC_SVBUS_PEND` writer - 13:13\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank7_ecc_svbus_pend"]
pub type Cpu0A53DualUDdataSpramBank7EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_DTAG_SPRAM_BANK0_ECC_SVBUS_PEND` reader - 14:14\\]
Interrupt Pending Status for cpu0_a53_dual_u_dtag_spram_bank0_ecc_svbus_pend"]
pub type Cpu0A53DualUDtagSpramBank0EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_DTAG_SPRAM_BANK0_ECC_SVBUS_PEND` writer - 14:14\\]
Interrupt Pending Status for cpu0_a53_dual_u_dtag_spram_bank0_ecc_svbus_pend"]
pub type Cpu0A53DualUDtagSpramBank0EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_DTAG_SPRAM_BANK1_ECC_SVBUS_PEND` reader - 15:15\\]
Interrupt Pending Status for cpu0_a53_dual_u_dtag_spram_bank1_ecc_svbus_pend"]
pub type Cpu0A53DualUDtagSpramBank1EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_DTAG_SPRAM_BANK1_ECC_SVBUS_PEND` writer - 15:15\\]
Interrupt Pending Status for cpu0_a53_dual_u_dtag_spram_bank1_ecc_svbus_pend"]
pub type Cpu0A53DualUDtagSpramBank1EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_DTAG_SPRAM_BANK2_ECC_SVBUS_PEND` reader - 16:16\\]
Interrupt Pending Status for cpu0_a53_dual_u_dtag_spram_bank2_ecc_svbus_pend"]
pub type Cpu0A53DualUDtagSpramBank2EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_DTAG_SPRAM_BANK2_ECC_SVBUS_PEND` writer - 16:16\\]
Interrupt Pending Status for cpu0_a53_dual_u_dtag_spram_bank2_ecc_svbus_pend"]
pub type Cpu0A53DualUDtagSpramBank2EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_DTAG_SPRAM_BANK3_ECC_SVBUS_PEND` reader - 17:17\\]
Interrupt Pending Status for cpu0_a53_dual_u_dtag_spram_bank3_ecc_svbus_pend"]
pub type Cpu0A53DualUDtagSpramBank3EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_DTAG_SPRAM_BANK3_ECC_SVBUS_PEND` writer - 17:17\\]
Interrupt Pending Status for cpu0_a53_dual_u_dtag_spram_bank3_ecc_svbus_pend"]
pub type Cpu0A53DualUDtagSpramBank3EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_DDIRTY_SPRAM_ECC_SVBUS_PEND` reader - 18:18\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddirty_spram_ecc_svbus_pend"]
pub type Cpu0A53DualUDdirtySpramEccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_DDIRTY_SPRAM_ECC_SVBUS_PEND` writer - 18:18\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddirty_spram_ecc_svbus_pend"]
pub type Cpu0A53DualUDdirtySpramEccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_TLB_SPRAM_BANK0_ECC_SVBUS_PEND` reader - 19:19\\]
Interrupt Pending Status for cpu0_a53_dual_u_tlb_spram_bank0_ecc_svbus_pend"]
pub type Cpu0A53DualUTlbSpramBank0EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_TLB_SPRAM_BANK0_ECC_SVBUS_PEND` writer - 19:19\\]
Interrupt Pending Status for cpu0_a53_dual_u_tlb_spram_bank0_ecc_svbus_pend"]
pub type Cpu0A53DualUTlbSpramBank0EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_TLB_SPRAM_BANK1_ECC_SVBUS_PEND` reader - 20:20\\]
Interrupt Pending Status for cpu0_a53_dual_u_tlb_spram_bank1_ecc_svbus_pend"]
pub type Cpu0A53DualUTlbSpramBank1EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_TLB_SPRAM_BANK1_ECC_SVBUS_PEND` writer - 20:20\\]
Interrupt Pending Status for cpu0_a53_dual_u_tlb_spram_bank1_ecc_svbus_pend"]
pub type Cpu0A53DualUTlbSpramBank1EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_TLB_SPRAM_BANK2_ECC_SVBUS_PEND` reader - 21:21\\]
Interrupt Pending Status for cpu0_a53_dual_u_tlb_spram_bank2_ecc_svbus_pend"]
pub type Cpu0A53DualUTlbSpramBank2EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_TLB_SPRAM_BANK2_ECC_SVBUS_PEND` writer - 21:21\\]
Interrupt Pending Status for cpu0_a53_dual_u_tlb_spram_bank2_ecc_svbus_pend"]
pub type Cpu0A53DualUTlbSpramBank2EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_TLB_SPRAM_BANK3_ECC_SVBUS_PEND` reader - 22:22\\]
Interrupt Pending Status for cpu0_a53_dual_u_tlb_spram_bank3_ecc_svbus_pend"]
pub type Cpu0A53DualUTlbSpramBank3EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_TLB_SPRAM_BANK3_ECC_SVBUS_PEND` writer - 22:22\\]
Interrupt Pending Status for cpu0_a53_dual_u_tlb_spram_bank3_ecc_svbus_pend"]
pub type Cpu0A53DualUTlbSpramBank3EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_L1D_TAGRAM_SPRAM_WAY0_ECC_SVBUS_PEND` reader - 23:23\\]
Interrupt Pending Status for cpu0_a53_dual_u_l1d_tagram_spram_way0_ecc_svbus_pend"]
pub type Cpu0A53DualUL1dTagramSpramWay0EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_L1D_TAGRAM_SPRAM_WAY0_ECC_SVBUS_PEND` writer - 23:23\\]
Interrupt Pending Status for cpu0_a53_dual_u_l1d_tagram_spram_way0_ecc_svbus_pend"]
pub type Cpu0A53DualUL1dTagramSpramWay0EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_L1D_TAGRAM_SPRAM_WAY1_ECC_SVBUS_PEND` reader - 24:24\\]
Interrupt Pending Status for cpu0_a53_dual_u_l1d_tagram_spram_way1_ecc_svbus_pend"]
pub type Cpu0A53DualUL1dTagramSpramWay1EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_L1D_TAGRAM_SPRAM_WAY1_ECC_SVBUS_PEND` writer - 24:24\\]
Interrupt Pending Status for cpu0_a53_dual_u_l1d_tagram_spram_way1_ecc_svbus_pend"]
pub type Cpu0A53DualUL1dTagramSpramWay1EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_L1D_TAGRAM_SPRAM_WAY2_ECC_SVBUS_PEND` reader - 25:25\\]
Interrupt Pending Status for cpu0_a53_dual_u_l1d_tagram_spram_way2_ecc_svbus_pend"]
pub type Cpu0A53DualUL1dTagramSpramWay2EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_L1D_TAGRAM_SPRAM_WAY2_ECC_SVBUS_PEND` writer - 25:25\\]
Interrupt Pending Status for cpu0_a53_dual_u_l1d_tagram_spram_way2_ecc_svbus_pend"]
pub type Cpu0A53DualUL1dTagramSpramWay2EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_A53_DUAL_U_L1D_TAGRAM_SPRAM_WAY3_ECC_SVBUS_PEND` reader - 26:26\\]
Interrupt Pending Status for cpu0_a53_dual_u_l1d_tagram_spram_way3_ecc_svbus_pend"]
pub type Cpu0A53DualUL1dTagramSpramWay3EccSvbusPendR = crate::BitReader;
#[doc = "Field `CPU0_A53_DUAL_U_L1D_TAGRAM_SPRAM_WAY3_ECC_SVBUS_PEND` writer - 26:26\\]
Interrupt Pending Status for cpu0_a53_dual_u_l1d_tagram_spram_way3_ecc_svbus_pend"]
pub type Cpu0A53DualUL1dTagramSpramWay3EccSvbusPendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for cpu0_a53_dual_u_idata_spram_bank0_lo_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_idata_spram_bank0_lo_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUIdataSpramBank0LoEccSvbusPendR {
        Cpu0A53DualUIdataSpramBank0LoEccSvbusPendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for cpu0_a53_dual_u_idata_spram_bank0_hi_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_idata_spram_bank0_hi_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUIdataSpramBank0HiEccSvbusPendR {
        Cpu0A53DualUIdataSpramBank0HiEccSvbusPendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for cpu0_a53_dual_u_idata_spram_bank1_lo_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_idata_spram_bank1_lo_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUIdataSpramBank1LoEccSvbusPendR {
        Cpu0A53DualUIdataSpramBank1LoEccSvbusPendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Pending Status for cpu0_a53_dual_u_idata_spram_bank1_hi_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_idata_spram_bank1_hi_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUIdataSpramBank1HiEccSvbusPendR {
        Cpu0A53DualUIdataSpramBank1HiEccSvbusPendR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Pending Status for cpu0_a53_dual_u_itag_spram_ram0_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_itag_spram_ram0_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUItagSpramRam0EccSvbusPendR {
        Cpu0A53DualUItagSpramRam0EccSvbusPendR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Pending Status for cpu0_a53_dual_u_itag_spram_ram1_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_itag_spram_ram1_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUItagSpramRam1EccSvbusPendR {
        Cpu0A53DualUItagSpramRam1EccSvbusPendR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank0_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_ddata_spram_bank0_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUDdataSpramBank0EccSvbusPendR {
        Cpu0A53DualUDdataSpramBank0EccSvbusPendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank1_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_ddata_spram_bank1_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUDdataSpramBank1EccSvbusPendR {
        Cpu0A53DualUDdataSpramBank1EccSvbusPendR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank2_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_ddata_spram_bank2_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUDdataSpramBank2EccSvbusPendR {
        Cpu0A53DualUDdataSpramBank2EccSvbusPendR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank3_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_ddata_spram_bank3_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUDdataSpramBank3EccSvbusPendR {
        Cpu0A53DualUDdataSpramBank3EccSvbusPendR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank4_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_ddata_spram_bank4_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUDdataSpramBank4EccSvbusPendR {
        Cpu0A53DualUDdataSpramBank4EccSvbusPendR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank5_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_ddata_spram_bank5_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUDdataSpramBank5EccSvbusPendR {
        Cpu0A53DualUDdataSpramBank5EccSvbusPendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank6_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_ddata_spram_bank6_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUDdataSpramBank6EccSvbusPendR {
        Cpu0A53DualUDdataSpramBank6EccSvbusPendR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank7_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_ddata_spram_bank7_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUDdataSpramBank7EccSvbusPendR {
        Cpu0A53DualUDdataSpramBank7EccSvbusPendR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt Pending Status for cpu0_a53_dual_u_dtag_spram_bank0_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_dtag_spram_bank0_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUDtagSpramBank0EccSvbusPendR {
        Cpu0A53DualUDtagSpramBank0EccSvbusPendR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt Pending Status for cpu0_a53_dual_u_dtag_spram_bank1_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_dtag_spram_bank1_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUDtagSpramBank1EccSvbusPendR {
        Cpu0A53DualUDtagSpramBank1EccSvbusPendR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt Pending Status for cpu0_a53_dual_u_dtag_spram_bank2_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_dtag_spram_bank2_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUDtagSpramBank2EccSvbusPendR {
        Cpu0A53DualUDtagSpramBank2EccSvbusPendR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt Pending Status for cpu0_a53_dual_u_dtag_spram_bank3_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_dtag_spram_bank3_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUDtagSpramBank3EccSvbusPendR {
        Cpu0A53DualUDtagSpramBank3EccSvbusPendR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddirty_spram_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_ddirty_spram_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUDdirtySpramEccSvbusPendR {
        Cpu0A53DualUDdirtySpramEccSvbusPendR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt Pending Status for cpu0_a53_dual_u_tlb_spram_bank0_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_tlb_spram_bank0_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUTlbSpramBank0EccSvbusPendR {
        Cpu0A53DualUTlbSpramBank0EccSvbusPendR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Interrupt Pending Status for cpu0_a53_dual_u_tlb_spram_bank1_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_tlb_spram_bank1_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUTlbSpramBank1EccSvbusPendR {
        Cpu0A53DualUTlbSpramBank1EccSvbusPendR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Interrupt Pending Status for cpu0_a53_dual_u_tlb_spram_bank2_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_tlb_spram_bank2_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUTlbSpramBank2EccSvbusPendR {
        Cpu0A53DualUTlbSpramBank2EccSvbusPendR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt Pending Status for cpu0_a53_dual_u_tlb_spram_bank3_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_tlb_spram_bank3_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUTlbSpramBank3EccSvbusPendR {
        Cpu0A53DualUTlbSpramBank3EccSvbusPendR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Interrupt Pending Status for cpu0_a53_dual_u_l1d_tagram_spram_way0_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_l1d_tagram_spram_way0_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUL1dTagramSpramWay0EccSvbusPendR {
        Cpu0A53DualUL1dTagramSpramWay0EccSvbusPendR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Interrupt Pending Status for cpu0_a53_dual_u_l1d_tagram_spram_way1_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_l1d_tagram_spram_way1_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUL1dTagramSpramWay1EccSvbusPendR {
        Cpu0A53DualUL1dTagramSpramWay1EccSvbusPendR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Interrupt Pending Status for cpu0_a53_dual_u_l1d_tagram_spram_way2_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_l1d_tagram_spram_way2_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUL1dTagramSpramWay2EccSvbusPendR {
        Cpu0A53DualUL1dTagramSpramWay2EccSvbusPendR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Interrupt Pending Status for cpu0_a53_dual_u_l1d_tagram_spram_way3_ecc_svbus_pend"]
    #[inline(always)]
    pub fn cpu0_a53_dual_u_l1d_tagram_spram_way3_ecc_svbus_pend(
        &self,
    ) -> Cpu0A53DualUL1dTagramSpramWay3EccSvbusPendR {
        Cpu0A53DualUL1dTagramSpramWay3EccSvbusPendR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for cpu0_a53_dual_u_idata_spram_bank0_lo_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_idata_spram_bank0_lo_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUIdataSpramBank0LoEccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUIdataSpramBank0LoEccSvbusPendW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for cpu0_a53_dual_u_idata_spram_bank0_hi_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_idata_spram_bank0_hi_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUIdataSpramBank0HiEccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUIdataSpramBank0HiEccSvbusPendW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for cpu0_a53_dual_u_idata_spram_bank1_lo_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_idata_spram_bank1_lo_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUIdataSpramBank1LoEccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUIdataSpramBank1LoEccSvbusPendW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Pending Status for cpu0_a53_dual_u_idata_spram_bank1_hi_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_idata_spram_bank1_hi_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUIdataSpramBank1HiEccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUIdataSpramBank1HiEccSvbusPendW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Pending Status for cpu0_a53_dual_u_itag_spram_ram0_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_itag_spram_ram0_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUItagSpramRam0EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUItagSpramRam0EccSvbusPendW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Pending Status for cpu0_a53_dual_u_itag_spram_ram1_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_itag_spram_ram1_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUItagSpramRam1EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUItagSpramRam1EccSvbusPendW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank0_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_ddata_spram_bank0_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUDdataSpramBank0EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUDdataSpramBank0EccSvbusPendW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank1_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_ddata_spram_bank1_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUDdataSpramBank1EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUDdataSpramBank1EccSvbusPendW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank2_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_ddata_spram_bank2_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUDdataSpramBank2EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUDdataSpramBank2EccSvbusPendW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank3_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_ddata_spram_bank3_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUDdataSpramBank3EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUDdataSpramBank3EccSvbusPendW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank4_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_ddata_spram_bank4_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUDdataSpramBank4EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUDdataSpramBank4EccSvbusPendW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank5_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_ddata_spram_bank5_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUDdataSpramBank5EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUDdataSpramBank5EccSvbusPendW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank6_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_ddata_spram_bank6_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUDdataSpramBank6EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUDdataSpramBank6EccSvbusPendW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddata_spram_bank7_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_ddata_spram_bank7_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUDdataSpramBank7EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUDdataSpramBank7EccSvbusPendW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt Pending Status for cpu0_a53_dual_u_dtag_spram_bank0_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_dtag_spram_bank0_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUDtagSpramBank0EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUDtagSpramBank0EccSvbusPendW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt Pending Status for cpu0_a53_dual_u_dtag_spram_bank1_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_dtag_spram_bank1_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUDtagSpramBank1EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUDtagSpramBank1EccSvbusPendW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt Pending Status for cpu0_a53_dual_u_dtag_spram_bank2_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_dtag_spram_bank2_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUDtagSpramBank2EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUDtagSpramBank2EccSvbusPendW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt Pending Status for cpu0_a53_dual_u_dtag_spram_bank3_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_dtag_spram_bank3_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUDtagSpramBank3EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUDtagSpramBank3EccSvbusPendW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt Pending Status for cpu0_a53_dual_u_ddirty_spram_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_ddirty_spram_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUDdirtySpramEccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUDdirtySpramEccSvbusPendW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt Pending Status for cpu0_a53_dual_u_tlb_spram_bank0_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_tlb_spram_bank0_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUTlbSpramBank0EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUTlbSpramBank0EccSvbusPendW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Interrupt Pending Status for cpu0_a53_dual_u_tlb_spram_bank1_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_tlb_spram_bank1_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUTlbSpramBank1EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUTlbSpramBank1EccSvbusPendW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Interrupt Pending Status for cpu0_a53_dual_u_tlb_spram_bank2_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_tlb_spram_bank2_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUTlbSpramBank2EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUTlbSpramBank2EccSvbusPendW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt Pending Status for cpu0_a53_dual_u_tlb_spram_bank3_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_tlb_spram_bank3_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUTlbSpramBank3EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUTlbSpramBank3EccSvbusPendW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Interrupt Pending Status for cpu0_a53_dual_u_l1d_tagram_spram_way0_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_l1d_tagram_spram_way0_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUL1dTagramSpramWay0EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUL1dTagramSpramWay0EccSvbusPendW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Interrupt Pending Status for cpu0_a53_dual_u_l1d_tagram_spram_way1_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_l1d_tagram_spram_way1_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUL1dTagramSpramWay1EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUL1dTagramSpramWay1EccSvbusPendW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Interrupt Pending Status for cpu0_a53_dual_u_l1d_tagram_spram_way2_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_l1d_tagram_spram_way2_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUL1dTagramSpramWay2EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUL1dTagramSpramWay2EccSvbusPendW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Interrupt Pending Status for cpu0_a53_dual_u_l1d_tagram_spram_way3_ecc_svbus_pend"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_a53_dual_u_l1d_tagram_spram_way3_ecc_svbus_pend(
        &mut self,
    ) -> Cpu0A53DualUL1dTagramSpramWay3EccSvbusPendW<EccAggrCore0RegsDedStatusReg0Spec> {
        Cpu0A53DualUL1dTagramSpramWay3EccSvbusPendW::new(self, 26)
    }
}
#[doc = "Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_ded_status_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_ded_status_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccAggrCore0RegsDedStatusReg0Spec;
impl crate::RegisterSpec for EccAggrCore0RegsDedStatusReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_aggr_core0_regs_ded_status_reg0::R`](R) reader structure"]
impl crate::Readable for EccAggrCore0RegsDedStatusReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ecc_aggr_core0_regs_ded_status_reg0::W`](W) writer structure"]
impl crate::Writable for EccAggrCore0RegsDedStatusReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_AGGR_CORE0_REGS_ded_status_reg0 to value 0"]
impl crate::Resettable for EccAggrCore0RegsDedStatusReg0Spec {
    const RESET_VALUE: u32 = 0;
}
