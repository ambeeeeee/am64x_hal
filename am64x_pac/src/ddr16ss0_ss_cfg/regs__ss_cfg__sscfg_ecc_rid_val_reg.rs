#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_RID_VAL_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgEccRidValRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_RID_VAL_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgEccRidValRegSpec>;
#[doc = "Field `ECCRID_VAL` reader - 11:0\\]
RouteID value written or read."]
pub type EccridValR = crate::FieldReader<u16>;
#[doc = "Field `ECCRID_VAL` writer - 11:0\\]
RouteID value written or read."]
pub type EccridValW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ECCRID_VAL_VLD` reader - 15:15\\]
A 1 in this field indicates that value in eccrid_val is valid."]
pub type EccridValVldR = crate::BitReader;
#[doc = "Field `ECCRID_VAL_VLD` writer - 15:15\\]
A 1 in this field indicates that value in eccrid_val is valid."]
pub type EccridValVldW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
RouteID value written or read."]
    #[inline(always)]
    pub fn eccrid_val(&self) -> EccridValR {
        EccridValR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\]
A 1 in this field indicates that value in eccrid_val is valid."]
    #[inline(always)]
    pub fn eccrid_val_vld(&self) -> EccridValVldR {
        EccridValVldR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
RouteID value written or read."]
    #[inline(always)]
    #[must_use]
    pub fn eccrid_val(&mut self) -> EccridValW<Regs_SsCfg_SscfgEccRidValRegSpec> {
        EccridValW::new(self, 0)
    }
    #[doc = "Bit 15 - 15:15\\]
A 1 in this field indicates that value in eccrid_val is valid."]
    #[inline(always)]
    #[must_use]
    pub fn eccrid_val_vld(&mut self) -> EccridValVldW<Regs_SsCfg_SscfgEccRidValRegSpec> {
        EccridValVldW::new(self, 15)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_ECC_RID_VAL_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_rid_val_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_rid_val_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgEccRidValRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgEccRidValRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ecc_rid_val_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgEccRidValRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ecc_rid_val_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgEccRidValRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_ECC_RID_VAL_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgEccRidValRegSpec {
    const RESET_VALUE: u32 = 0;
}
