#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_39` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl39Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_39` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl39Spec>;
#[doc = "Field `TMOD_PAR_F0` reader - 7:0\\]
DRAM TMOD value when CA parity is enabled in cycles. FC=0"]
pub type TmodParF0R = crate::FieldReader;
#[doc = "Field `TMOD_PAR_F0` writer - 7:0\\]
DRAM TMOD value when CA parity is enabled in cycles. FC=0"]
pub type TmodParF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMRD_PAR_F0` reader - 15:8\\]
DRAM TMRD value when CA parity is enabled in cycles. FC=0"]
pub type TmrdParF0R = crate::FieldReader;
#[doc = "Field `TMRD_PAR_F0` writer - 15:8\\]
DRAM TMRD value when CA parity is enabled in cycles. FC=0"]
pub type TmrdParF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMOD_PAR_MAX_PL_F0` reader - 23:16\\]
DRAM TMOD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=0"]
pub type TmodParMaxPlF0R = crate::FieldReader;
#[doc = "Field `TMOD_PAR_MAX_PL_F0` writer - 23:16\\]
DRAM TMOD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=0"]
pub type TmodParMaxPlF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMRD_PAR_MAX_PL_F0` reader - 31:24\\]
DRAM TMRD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=0"]
pub type TmrdParMaxPlF0R = crate::FieldReader;
#[doc = "Field `TMRD_PAR_MAX_PL_F0` writer - 31:24\\]
DRAM TMRD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=0"]
pub type TmrdParMaxPlF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TMOD value when CA parity is enabled in cycles. FC=0"]
    #[inline(always)]
    pub fn tmod_par_f0(&self) -> TmodParF0R {
        TmodParF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TMRD value when CA parity is enabled in cycles. FC=0"]
    #[inline(always)]
    pub fn tmrd_par_f0(&self) -> TmrdParF0R {
        TmrdParF0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TMOD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=0"]
    #[inline(always)]
    pub fn tmod_par_max_pl_f0(&self) -> TmodParMaxPlF0R {
        TmodParMaxPlF0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TMRD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=0"]
    #[inline(always)]
    pub fn tmrd_par_max_pl_f0(&self) -> TmrdParMaxPlF0R {
        TmrdParMaxPlF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TMOD value when CA parity is enabled in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tmod_par_f0(&mut self) -> TmodParF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl39Spec> {
        TmodParF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TMRD value when CA parity is enabled in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tmrd_par_f0(&mut self) -> TmrdParF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl39Spec> {
        TmrdParF0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TMOD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tmod_par_max_pl_f0(
        &mut self,
    ) -> TmodParMaxPlF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl39Spec> {
        TmodParMaxPlF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TMRD value with maximum CA parity for frequency copy 2 in cycles. Used during changes of CA parity latency value in DRAM. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tmrd_par_max_pl_f0(
        &mut self,
    ) -> TmrdParMaxPlF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl39Spec> {
        TmrdParMaxPlF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_39::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_39::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl39Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl39Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_39::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl39Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_39::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl39Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_39 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl39Spec {
    const RESET_VALUE: u32 = 0;
}
