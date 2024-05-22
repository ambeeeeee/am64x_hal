#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_43` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl43Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_43` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl43Spec>;
#[doc = "Field `TMOD_PAR_F2` reader - 7:0\\]
DRAM TMOD value when CA parity is enabled in cycles. FC=2"]
pub type TmodParF2R = crate::FieldReader;
#[doc = "Field `TMOD_PAR_F2` writer - 7:0\\]
DRAM TMOD value when CA parity is enabled in cycles. FC=2"]
pub type TmodParF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMRD_PAR_F2` reader - 15:8\\]
DRAM TMRD value when CA parity is enabled in cycles. FC=2"]
pub type TmrdParF2R = crate::FieldReader;
#[doc = "Field `TMRD_PAR_F2` writer - 15:8\\]
DRAM TMRD value when CA parity is enabled in cycles. FC=2"]
pub type TmrdParF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMOD_PAR_MAX_PL_F2` reader - 23:16\\]
DRAM TMOD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=2"]
pub type TmodParMaxPlF2R = crate::FieldReader;
#[doc = "Field `TMOD_PAR_MAX_PL_F2` writer - 23:16\\]
DRAM TMOD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=2"]
pub type TmodParMaxPlF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMRD_PAR_MAX_PL_F2` reader - 31:24\\]
DRAM TMRD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=2"]
pub type TmrdParMaxPlF2R = crate::FieldReader;
#[doc = "Field `TMRD_PAR_MAX_PL_F2` writer - 31:24\\]
DRAM TMRD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=2"]
pub type TmrdParMaxPlF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TMOD value when CA parity is enabled in cycles. FC=2"]
    #[inline(always)]
    pub fn tmod_par_f2(&self) -> TmodParF2R {
        TmodParF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TMRD value when CA parity is enabled in cycles. FC=2"]
    #[inline(always)]
    pub fn tmrd_par_f2(&self) -> TmrdParF2R {
        TmrdParF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TMOD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=2"]
    #[inline(always)]
    pub fn tmod_par_max_pl_f2(&self) -> TmodParMaxPlF2R {
        TmodParMaxPlF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TMRD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=2"]
    #[inline(always)]
    pub fn tmrd_par_max_pl_f2(&self) -> TmrdParMaxPlF2R {
        TmrdParMaxPlF2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TMOD value when CA parity is enabled in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tmod_par_f2(&mut self) -> TmodParF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl43Spec> {
        TmodParF2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TMRD value when CA parity is enabled in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tmrd_par_f2(&mut self) -> TmrdParF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl43Spec> {
        TmrdParF2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TMOD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tmod_par_max_pl_f2(
        &mut self,
    ) -> TmodParMaxPlF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl43Spec> {
        TmodParMaxPlF2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TMRD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tmrd_par_max_pl_f2(
        &mut self,
    ) -> TmrdParMaxPlF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl43Spec> {
        TmrdParMaxPlF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_43::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_43::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl43Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl43Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_43::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl43Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_43::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl43Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_43 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl43Spec {
    const RESET_VALUE: u32 = 0;
}
