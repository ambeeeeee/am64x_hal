#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_57` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl57Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_57` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl57Spec>;
#[doc = "Field `TMRD_F1` reader - 7:0\\]
DRAM TMRD value in cycles. FC=1"]
pub type TmrdF1R = crate::FieldReader;
#[doc = "Field `TMRD_F1` writer - 7:0\\]
DRAM TMRD value in cycles. FC=1"]
pub type TmrdF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMOD_F1` reader - 15:8\\]
Number of cycles after MRS command and before any other command. FC=1"]
pub type TmodF1R = crate::FieldReader;
#[doc = "Field `TMOD_F1` writer - 15:8\\]
Number of cycles after MRS command and before any other command. FC=1"]
pub type TmodF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TMRD value in cycles. FC=1"]
    #[inline(always)]
    pub fn tmrd_f1(&self) -> TmrdF1R {
        TmrdF1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of cycles after MRS command and before any other command. FC=1"]
    #[inline(always)]
    pub fn tmod_f1(&self) -> TmodF1R {
        TmodF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TMRD value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tmrd_f1(&mut self) -> TmrdF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl57Spec> {
        TmrdF1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of cycles after MRS command and before any other command. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tmod_f1(&mut self) -> TmodF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl57Spec> {
        TmodF1W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_57\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_57::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_57::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl57Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl57Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_57::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl57Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_57::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl57Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_57 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl57Spec {
    const RESET_VALUE: u32 = 0;
}
