#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_41` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl41Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_41` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl41Spec>;
#[doc = "Field `TMOD_PAR_F1` reader - 7:0\\]
DRAM TMOD value when CA parity is enabled in cycles. FC=1"]
pub type TmodParF1R = crate::FieldReader;
#[doc = "Field `TMOD_PAR_F1` writer - 7:0\\]
DRAM TMOD value when CA parity is enabled in cycles. FC=1"]
pub type TmodParF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMRD_PAR_F1` reader - 15:8\\]
DRAM TMRD value when CA parity is enabled in cycles. FC=1"]
pub type TmrdParF1R = crate::FieldReader;
#[doc = "Field `TMRD_PAR_F1` writer - 15:8\\]
DRAM TMRD value when CA parity is enabled in cycles. FC=1"]
pub type TmrdParF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMOD_PAR_MAX_PL_F1` reader - 23:16\\]
DRAM TMOD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=1"]
pub type TmodParMaxPlF1R = crate::FieldReader;
#[doc = "Field `TMOD_PAR_MAX_PL_F1` writer - 23:16\\]
DRAM TMOD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=1"]
pub type TmodParMaxPlF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMRD_PAR_MAX_PL_F1` reader - 31:24\\]
DRAM TMRD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=1"]
pub type TmrdParMaxPlF1R = crate::FieldReader;
#[doc = "Field `TMRD_PAR_MAX_PL_F1` writer - 31:24\\]
DRAM TMRD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=1"]
pub type TmrdParMaxPlF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TMOD value when CA parity is enabled in cycles. FC=1"]
    #[inline(always)]
    pub fn tmod_par_f1(&self) -> TmodParF1R {
        TmodParF1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TMRD value when CA parity is enabled in cycles. FC=1"]
    #[inline(always)]
    pub fn tmrd_par_f1(&self) -> TmrdParF1R {
        TmrdParF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TMOD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=1"]
    #[inline(always)]
    pub fn tmod_par_max_pl_f1(&self) -> TmodParMaxPlF1R {
        TmodParMaxPlF1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TMRD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=1"]
    #[inline(always)]
    pub fn tmrd_par_max_pl_f1(&self) -> TmrdParMaxPlF1R {
        TmrdParMaxPlF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TMOD value when CA parity is enabled in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tmod_par_f1(&mut self) -> TmodParF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl41Spec> {
        TmodParF1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TMRD value when CA parity is enabled in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tmrd_par_f1(&mut self) -> TmrdParF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl41Spec> {
        TmrdParF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TMOD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tmod_par_max_pl_f1(
        &mut self,
    ) -> TmodParMaxPlF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl41Spec> {
        TmodParMaxPlF1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TMRD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tmrd_par_max_pl_f1(
        &mut self,
    ) -> TmrdParMaxPlF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl41Spec> {
        TmrdParMaxPlF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_41::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_41::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl41Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl41Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_41::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl41Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_41::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl41Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_41 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl41Spec {
    const RESET_VALUE: u32 = 0;
}
