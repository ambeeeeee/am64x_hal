#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_253` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi253Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_253` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi253Spec>;
#[doc = "Field `PI_TXSR_F0` reader - 15:0\\]
DRAM TXSR value for frequency set 0 in cycles."]
pub type PiTxsrF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TXSR_F0` writer - 15:0\\]
DRAM TXSR value for frequency set 0 in cycles."]
pub type PiTxsrF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_TXSR_F1` reader - 31:16\\]
DRAM TXSR value for frequency set 1 in cycles."]
pub type PiTxsrF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TXSR_F1` writer - 31:16\\]
DRAM TXSR value for frequency set 1 in cycles."]
pub type PiTxsrF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
DRAM TXSR value for frequency set 0 in cycles."]
    #[inline(always)]
    pub fn pi_txsr_f0(&self) -> PiTxsrF0R {
        PiTxsrF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
DRAM TXSR value for frequency set 1 in cycles."]
    #[inline(always)]
    pub fn pi_txsr_f1(&self) -> PiTxsrF1R {
        PiTxsrF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
DRAM TXSR value for frequency set 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_txsr_f0(&mut self) -> PiTxsrF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi253Spec> {
        PiTxsrF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
DRAM TXSR value for frequency set 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_txsr_f1(&mut self) -> PiTxsrF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi253Spec> {
        PiTxsrF1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_253\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_253::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_253::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi253Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi253Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_253::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi253Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_253::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi253Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_253 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi253Spec {
    const RESET_VALUE: u32 = 0;
}
