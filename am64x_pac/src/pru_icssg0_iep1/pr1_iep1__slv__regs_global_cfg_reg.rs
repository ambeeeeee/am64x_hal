#[doc = "Register `PR1_IEP1__SLV__REGS_global_cfg_reg` reader"]
pub type R = crate::R<Pr1Iep1_Slv_RegsGlobalCfgRegSpec>;
#[doc = "Register `PR1_IEP1__SLV__REGS_global_cfg_reg` writer"]
pub type W = crate::W<Pr1Iep1_Slv_RegsGlobalCfgRegSpec>;
#[doc = "Field `CNT_ENABLE` reader - "]
pub type CntEnableR = crate::BitReader;
#[doc = "Field `CNT_ENABLE` writer - "]
pub type CntEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEFAULT_INC` reader - "]
pub type DefaultIncR = crate::FieldReader;
#[doc = "Field `DEFAULT_INC` writer - "]
pub type DefaultIncW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CMP_INC` reader - "]
pub type CmpIncR = crate::FieldReader<u16>;
#[doc = "Field `CMP_INC` writer - "]
pub type CmpIncW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnt_enable(&self) -> CntEnableR {
        CntEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn default_inc(&self) -> DefaultIncR {
        DefaultIncR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:19"]
    #[inline(always)]
    pub fn cmp_inc(&self) -> CmpIncR {
        CmpIncR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_enable(&mut self) -> CntEnableW<Pr1Iep1_Slv_RegsGlobalCfgRegSpec> {
        CntEnableW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn default_inc(&mut self) -> DefaultIncW<Pr1Iep1_Slv_RegsGlobalCfgRegSpec> {
        DefaultIncW::new(self, 4)
    }
    #[doc = "Bits 8:19"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_inc(&mut self) -> CmpIncW<Pr1Iep1_Slv_RegsGlobalCfgRegSpec> {
        CmpIncW::new(self, 8)
    }
}
#[doc = "PR1_IEP1__SLV__REGS_global_cfg_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_global_cfg_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_global_cfg_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep1_Slv_RegsGlobalCfgRegSpec;
impl crate::RegisterSpec for Pr1Iep1_Slv_RegsGlobalCfgRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep1__slv__regs_global_cfg_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep1_Slv_RegsGlobalCfgRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep1__slv__regs_global_cfg_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep1_Slv_RegsGlobalCfgRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP1__SLV__REGS_global_cfg_reg to value 0x0550"]
impl crate::Resettable for Pr1Iep1_Slv_RegsGlobalCfgRegSpec {
    const RESET_VALUE: u32 = 0x0550;
}
