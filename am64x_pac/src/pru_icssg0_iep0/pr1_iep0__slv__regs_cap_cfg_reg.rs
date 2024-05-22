#[doc = "Register `PR1_IEP0__SLV__REGS_cap_cfg_reg` reader"]
pub type R = crate::R<Pr1Iep0_Slv_RegsCapCfgRegSpec>;
#[doc = "Register `PR1_IEP0__SLV__REGS_cap_cfg_reg` writer"]
pub type W = crate::W<Pr1Iep0_Slv_RegsCapCfgRegSpec>;
#[doc = "Field `CAP_EN` reader - "]
pub type CapEnR = crate::FieldReader<u16>;
#[doc = "Field `CAP_EN` writer - "]
pub type CapEnW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CAP_ASYNC_EN` reader - "]
pub type CapAsyncEnR = crate::FieldReader;
#[doc = "Field `CAP_ASYNC_EN` writer - "]
pub type CapAsyncEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXT_CAP_EN` reader - "]
pub type ExtCapEnR = crate::FieldReader;
#[doc = "Field `EXT_CAP_EN` writer - "]
pub type ExtCapEnW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn cap_en(&self) -> CapEnR {
        CapEnR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:17"]
    #[inline(always)]
    pub fn cap_async_en(&self) -> CapAsyncEnR {
        CapAsyncEnR::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn ext_cap_en(&self) -> ExtCapEnR {
        ExtCapEnR::new(((self.bits >> 18) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn cap_en(&mut self) -> CapEnW<Pr1Iep0_Slv_RegsCapCfgRegSpec> {
        CapEnW::new(self, 0)
    }
    #[doc = "Bits 10:17"]
    #[inline(always)]
    #[must_use]
    pub fn cap_async_en(&mut self) -> CapAsyncEnW<Pr1Iep0_Slv_RegsCapCfgRegSpec> {
        CapAsyncEnW::new(self, 10)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    #[must_use]
    pub fn ext_cap_en(&mut self) -> ExtCapEnW<Pr1Iep0_Slv_RegsCapCfgRegSpec> {
        ExtCapEnW::new(self, 18)
    }
}
#[doc = "PR1_IEP0__SLV__REGS_cap_cfg_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep0__slv__regs_cap_cfg_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep0__slv__regs_cap_cfg_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep0_Slv_RegsCapCfgRegSpec;
impl crate::RegisterSpec for Pr1Iep0_Slv_RegsCapCfgRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep0__slv__regs_cap_cfg_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep0_Slv_RegsCapCfgRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep0__slv__regs_cap_cfg_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep0_Slv_RegsCapCfgRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP0__SLV__REGS_cap_cfg_reg to value 0x0004_9c00"]
impl crate::Resettable for Pr1Iep0_Slv_RegsCapCfgRegSpec {
    const RESET_VALUE: u32 = 0x0004_9c00;
}
