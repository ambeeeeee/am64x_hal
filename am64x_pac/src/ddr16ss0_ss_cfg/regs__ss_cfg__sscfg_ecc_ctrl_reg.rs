#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_CTRL_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgEccCtrlRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_CTRL_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgEccCtrlRegSpec>;
#[doc = "Field `ECC_EN` reader - 0:0\\]
DRAM ECC enable. Setting a 1 causes ECC to be written to DRAM. This bit must be set and kept static before using DDR."]
pub type EccEnR = crate::BitReader;
#[doc = "Field `ECC_EN` writer - 0:0\\]
DRAM ECC enable. Setting a 1 causes ECC to be written to DRAM. This bit must be set and kept static before using DDR."]
pub type EccEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMW_EN` reader - 1:1\\]
Read modify write enable. Set 1 to enable RMW functionality for sub-quanta accesses when ecc_en=1. This bit must be set to 1 if ecc_en is set to a 1 to ensure subquanta accesses to DRAM do not result in ECC errors. This bit must be set and kept static before using DDR."]
pub type RmwEnR = crate::BitReader;
#[doc = "Field `RMW_EN` writer - 1:1\\]
Read modify write enable. Set 1 to enable RMW functionality for sub-quanta accesses when ecc_en=1. This bit must be set to 1 if ecc_en is set to a 1 to ensure subquanta accesses to DRAM do not result in ECC errors. This bit must be set and kept static before using DDR."]
pub type RmwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_CK` reader - 2:2\\]
Set 1 to enable ECC verification for read accesses when ecc_en=1. The value of this field is ignored when ecc_en=0. This bit must be set and kept static before using DDR."]
pub type EccCkR = crate::BitReader;
#[doc = "Field `ECC_CK` writer - 2:2\\]
Set 1 to enable ECC verification for read accesses when ecc_en=1. The value of this field is ignored when ecc_en=0. This bit must be set and kept static before using DDR."]
pub type EccCkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_ALLOC` reader - 4:4\\]
When set to 1, an unassigned ECC cache-line will be allocated for a write with routeID that do not match any of the mapped routeID's."]
pub type WrAllocR = crate::BitReader;
#[doc = "Field `WR_ALLOC` writer - 4:4\\]
When set to 1, an unassigned ECC cache-line will be allocated for a write with routeID that do not match any of the mapped routeID's."]
pub type WrAllocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COR_ECC_THRESH` reader - 10:8\\]
Threshold for 1-bit ECC errors in multiple data words in an SDRAM burst that create an uncorrected error fault indication. Value of 0/1 means 2 or more 1-bit errors in multiple data words will result in an uncorrected error fault indication, value of 2 means 3 or more 1-bit errors will result in an uncorrected error fault indication, and so on. Value of 4 or greater disables this feature. This field must always be kept at default, and only changed for debug."]
pub type CorEccThreshR = crate::FieldReader;
#[doc = "Field `COR_ECC_THRESH` writer - 10:8\\]
Threshold for 1-bit ECC errors in multiple data words in an SDRAM burst that create an uncorrected error fault indication. Value of 0/1 means 2 or more 1-bit errors in multiple data words will result in an uncorrected error fault indication, value of 2 means 3 or more 1-bit errors will result in an uncorrected error fault indication, and so on. Value of 4 or greater disables this feature. This field must always be kept at default, and only changed for debug."]
pub type CorEccThreshW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DRAM ECC enable. Setting a 1 causes ECC to be written to DRAM. This bit must be set and kept static before using DDR."]
    #[inline(always)]
    pub fn ecc_en(&self) -> EccEnR {
        EccEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Read modify write enable. Set 1 to enable RMW functionality for sub-quanta accesses when ecc_en=1. This bit must be set to 1 if ecc_en is set to a 1 to ensure subquanta accesses to DRAM do not result in ECC errors. This bit must be set and kept static before using DDR."]
    #[inline(always)]
    pub fn rmw_en(&self) -> RmwEnR {
        RmwEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Set 1 to enable ECC verification for read accesses when ecc_en=1. The value of this field is ignored when ecc_en=0. This bit must be set and kept static before using DDR."]
    #[inline(always)]
    pub fn ecc_ck(&self) -> EccCkR {
        EccCkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
When set to 1, an unassigned ECC cache-line will be allocated for a write with routeID that do not match any of the mapped routeID's."]
    #[inline(always)]
    pub fn wr_alloc(&self) -> WrAllocR {
        WrAllocR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Threshold for 1-bit ECC errors in multiple data words in an SDRAM burst that create an uncorrected error fault indication. Value of 0/1 means 2 or more 1-bit errors in multiple data words will result in an uncorrected error fault indication, value of 2 means 3 or more 1-bit errors will result in an uncorrected error fault indication, and so on. Value of 4 or greater disables this feature. This field must always be kept at default, and only changed for debug."]
    #[inline(always)]
    pub fn cor_ecc_thresh(&self) -> CorEccThreshR {
        CorEccThreshR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DRAM ECC enable. Setting a 1 causes ECC to be written to DRAM. This bit must be set and kept static before using DDR."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_en(&mut self) -> EccEnW<Regs_SsCfg_SscfgEccCtrlRegSpec> {
        EccEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Read modify write enable. Set 1 to enable RMW functionality for sub-quanta accesses when ecc_en=1. This bit must be set to 1 if ecc_en is set to a 1 to ensure subquanta accesses to DRAM do not result in ECC errors. This bit must be set and kept static before using DDR."]
    #[inline(always)]
    #[must_use]
    pub fn rmw_en(&mut self) -> RmwEnW<Regs_SsCfg_SscfgEccCtrlRegSpec> {
        RmwEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Set 1 to enable ECC verification for read accesses when ecc_en=1. The value of this field is ignored when ecc_en=0. This bit must be set and kept static before using DDR."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_ck(&mut self) -> EccCkW<Regs_SsCfg_SscfgEccCtrlRegSpec> {
        EccCkW::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
When set to 1, an unassigned ECC cache-line will be allocated for a write with routeID that do not match any of the mapped routeID's."]
    #[inline(always)]
    #[must_use]
    pub fn wr_alloc(&mut self) -> WrAllocW<Regs_SsCfg_SscfgEccCtrlRegSpec> {
        WrAllocW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Threshold for 1-bit ECC errors in multiple data words in an SDRAM burst that create an uncorrected error fault indication. Value of 0/1 means 2 or more 1-bit errors in multiple data words will result in an uncorrected error fault indication, value of 2 means 3 or more 1-bit errors will result in an uncorrected error fault indication, and so on. Value of 4 or greater disables this feature. This field must always be kept at default, and only changed for debug."]
    #[inline(always)]
    #[must_use]
    pub fn cor_ecc_thresh(&mut self) -> CorEccThreshW<Regs_SsCfg_SscfgEccCtrlRegSpec> {
        CorEccThreshW::new(self, 8)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_ECC_CTRL_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_ctrl_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_ctrl_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgEccCtrlRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgEccCtrlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ecc_ctrl_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgEccCtrlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ecc_ctrl_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgEccCtrlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_ECC_CTRL_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgEccCtrlRegSpec {
    const RESET_VALUE: u32 = 0;
}
