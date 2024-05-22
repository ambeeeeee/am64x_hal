#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_275` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl275Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_275` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl275Spec>;
#[doc = "Field `MR22_DATA_F2_1` reader - 16:0\\]
Data to program into memory mode register 22. FC=2"]
pub type Mr22DataF2_1R = crate::FieldReader<u32>;
#[doc = "Field `MR22_DATA_F2_1` writer - 16:0\\]
Data to program into memory mode register 22. FC=2"]
pub type Mr22DataF2_1W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - 16:0\\]
Data to program into memory mode register 22. FC=2"]
    #[inline(always)]
    pub fn mr22_data_f2_1(&self) -> Mr22DataF2_1R {
        Mr22DataF2_1R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - 16:0\\]
Data to program into memory mode register 22. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn mr22_data_f2_1(&mut self) -> Mr22DataF2_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl275Spec> {
        Mr22DataF2_1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_275\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_275::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_275::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl275Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl275Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_275::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl275Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_275::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl275Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_275 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl275Spec {
    const RESET_VALUE: u32 = 0;
}