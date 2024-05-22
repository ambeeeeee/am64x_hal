#[doc = "Register `PR1_IEP1__SLV__REGS_cmp_cfg_reg` reader"]
pub type R = crate::R<Pr1Iep1_Slv_RegsCmpCfgRegSpec>;
#[doc = "Register `PR1_IEP1__SLV__REGS_cmp_cfg_reg` writer"]
pub type W = crate::W<Pr1Iep1_Slv_RegsCmpCfgRegSpec>;
#[doc = "Field `CMP0_RST_CNT_EN` reader - "]
pub type Cmp0RstCntEnR = crate::BitReader;
#[doc = "Field `CMP0_RST_CNT_EN` writer - "]
pub type Cmp0RstCntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP_EN` reader - "]
pub type CmpEnR = crate::FieldReader<u16>;
#[doc = "Field `CMP_EN` writer - "]
pub type CmpEnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SHADOW_EN` reader - "]
pub type ShadowEnR = crate::BitReader;
#[doc = "Field `SHADOW_EN` writer - "]
pub type ShadowEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmp0_rst_cnt_en(&self) -> Cmp0RstCntEnR {
        Cmp0RstCntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:16"]
    #[inline(always)]
    pub fn cmp_en(&self) -> CmpEnR {
        CmpEnR::new(((self.bits >> 1) & 0xffff) as u16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn shadow_en(&self) -> ShadowEnR {
        ShadowEnR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0_rst_cnt_en(&mut self) -> Cmp0RstCntEnW<Pr1Iep1_Slv_RegsCmpCfgRegSpec> {
        Cmp0RstCntEnW::new(self, 0)
    }
    #[doc = "Bits 1:16"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_en(&mut self) -> CmpEnW<Pr1Iep1_Slv_RegsCmpCfgRegSpec> {
        CmpEnW::new(self, 1)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn shadow_en(&mut self) -> ShadowEnW<Pr1Iep1_Slv_RegsCmpCfgRegSpec> {
        ShadowEnW::new(self, 17)
    }
}
#[doc = "PR1_IEP1__SLV__REGS_cmp_cfg_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp_cfg_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp_cfg_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep1_Slv_RegsCmpCfgRegSpec;
impl crate::RegisterSpec for Pr1Iep1_Slv_RegsCmpCfgRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep1__slv__regs_cmp_cfg_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep1_Slv_RegsCmpCfgRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep1__slv__regs_cmp_cfg_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep1_Slv_RegsCmpCfgRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP1__SLV__REGS_cmp_cfg_reg to value 0"]
impl crate::Resettable for Pr1Iep1_Slv_RegsCmpCfgRegSpec {
    const RESET_VALUE: u32 = 0;
}
