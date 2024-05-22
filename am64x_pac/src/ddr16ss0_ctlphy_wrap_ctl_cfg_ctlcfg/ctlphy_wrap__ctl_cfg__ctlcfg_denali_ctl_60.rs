#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_60` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl60Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_60` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl60Spec>;
#[doc = "Field `TMRD_F2` reader - 7:0\\]
DRAM TMRD value in cycles. FC=2"]
pub type TmrdF2R = crate::FieldReader;
#[doc = "Field `TMRD_F2` writer - 7:0\\]
DRAM TMRD value in cycles. FC=2"]
pub type TmrdF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMOD_F2` reader - 15:8\\]
Number of cycles after MRS command and before any other command. FC=2"]
pub type TmodF2R = crate::FieldReader;
#[doc = "Field `TMOD_F2` writer - 15:8\\]
Number of cycles after MRS command and before any other command. FC=2"]
pub type TmodF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TMRD value in cycles. FC=2"]
    #[inline(always)]
    pub fn tmrd_f2(&self) -> TmrdF2R {
        TmrdF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of cycles after MRS command and before any other command. FC=2"]
    #[inline(always)]
    pub fn tmod_f2(&self) -> TmodF2R {
        TmodF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TMRD value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tmrd_f2(&mut self) -> TmrdF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl60Spec> {
        TmrdF2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of cycles after MRS command and before any other command. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tmod_f2(&mut self) -> TmodF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl60Spec> {
        TmodF2W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_60::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_60::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl60Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_60::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl60Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_60::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl60Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_60 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl60Spec {
    const RESET_VALUE: u32 = 0;
}
