#[doc = "Register `PR1_CFG__SLV__REGS_mii_rt_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsMiiRtRegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_mii_rt_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsMiiRtRegSpec>;
#[doc = "Field `MII_RT_EVENT_EN` reader - "]
pub type MiiRtEventEnR = crate::BitReader;
#[doc = "Field `MII_RT_EVENT_EN` writer - "]
pub type MiiRtEventEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mii_rt_event_en(&self) -> MiiRtEventEnR {
        MiiRtEventEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn mii_rt_event_en(&mut self) -> MiiRtEventEnW<Pr1Cfg_Slv_RegsMiiRtRegSpec> {
        MiiRtEventEnW::new(self, 0)
    }
}
#[doc = "PR1_CFG__SLV__REGS_mii_rt_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_mii_rt_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_mii_rt_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsMiiRtRegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsMiiRtRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_mii_rt_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsMiiRtRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_mii_rt_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsMiiRtRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_mii_rt_reg to value 0x01"]
impl crate::Resettable for Pr1Cfg_Slv_RegsMiiRtRegSpec {
    const RESET_VALUE: u32 = 0x01;
}
