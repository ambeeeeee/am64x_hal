#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_254` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi254Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_254` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi254Spec>;
#[doc = "Field `PI_TXSR_F2` reader - 15:0\\]
DRAM TXSR value for frequency set 2 in cycles."]
pub type PiTxsrF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TXSR_F2` writer - 15:0\\]
DRAM TXSR value for frequency set 2 in cycles."]
pub type PiTxsrF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_TEXCKE_F0` reader - 21:16\\]
DRAM CKE low after SREF command timing for frequency set 0."]
pub type PiTexckeF0R = crate::FieldReader;
#[doc = "Field `PI_TEXCKE_F0` writer - 21:16\\]
DRAM CKE low after SREF command timing for frequency set 0."]
pub type PiTexckeF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_TEXCKE_F1` reader - 29:24\\]
DRAM CKE low after SREF command timing for frequency set 1."]
pub type PiTexckeF1R = crate::FieldReader;
#[doc = "Field `PI_TEXCKE_F1` writer - 29:24\\]
DRAM CKE low after SREF command timing for frequency set 1."]
pub type PiTexckeF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
DRAM TXSR value for frequency set 2 in cycles."]
    #[inline(always)]
    pub fn pi_txsr_f2(&self) -> PiTxsrF2R {
        PiTxsrF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
DRAM CKE low after SREF command timing for frequency set 0."]
    #[inline(always)]
    pub fn pi_texcke_f0(&self) -> PiTexckeF0R {
        PiTexckeF0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
DRAM CKE low after SREF command timing for frequency set 1."]
    #[inline(always)]
    pub fn pi_texcke_f1(&self) -> PiTexckeF1R {
        PiTexckeF1R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
DRAM TXSR value for frequency set 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_txsr_f2(&mut self) -> PiTxsrF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi254Spec> {
        PiTxsrF2W::new(self, 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
DRAM CKE low after SREF command timing for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_texcke_f0(&mut self) -> PiTexckeF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi254Spec> {
        PiTexckeF0W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
DRAM CKE low after SREF command timing for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_texcke_f1(&mut self) -> PiTexckeF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi254Spec> {
        PiTexckeF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_254\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_254::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_254::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi254Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi254Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_254::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi254Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_254::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi254Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_254 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi254Spec {
    const RESET_VALUE: u32 = 0;
}
