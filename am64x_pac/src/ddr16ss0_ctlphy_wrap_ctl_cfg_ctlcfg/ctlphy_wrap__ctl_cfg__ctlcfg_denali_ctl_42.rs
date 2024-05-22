#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_42` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl42Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_42` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl42Spec>;
#[doc = "Field `CASLAT_LIN_F2` reader - 6:0\\]
Sets latency from read command send to data receive from/to controller. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller. FC=2"]
pub type CaslatLinF2R = crate::FieldReader;
#[doc = "Field `CASLAT_LIN_F2` writer - 6:0\\]
Sets latency from read command send to data receive from/to controller. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller. FC=2"]
pub type CaslatLinF2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRLAT_F2` reader - 14:8\\]
DRAM WRLAT value in cycles. FC=2"]
pub type WrlatF2R = crate::FieldReader;
#[doc = "Field `WRLAT_F2` writer - 14:8\\]
DRAM WRLAT value in cycles. FC=2"]
pub type WrlatF2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADDITIVE_LAT_F2` reader - 21:16\\]
DRAM additive latency value in cycles. FC=2"]
pub type AdditiveLatF2R = crate::FieldReader;
#[doc = "Field `ADDITIVE_LAT_F2` writer - 21:16\\]
DRAM additive latency value in cycles. FC=2"]
pub type AdditiveLatF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CA_PARITY_LAT_F2` reader - 27:24\\]
DRAM CA parity latency value in cycles. FC=2"]
pub type CaParityLatF2R = crate::FieldReader;
#[doc = "Field `CA_PARITY_LAT_F2` writer - 27:24\\]
DRAM CA parity latency value in cycles. FC=2"]
pub type CaParityLatF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Sets latency from read command send to data receive from/to controller. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller. FC=2"]
    #[inline(always)]
    pub fn caslat_lin_f2(&self) -> CaslatLinF2R {
        CaslatLinF2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
DRAM WRLAT value in cycles. FC=2"]
    #[inline(always)]
    pub fn wrlat_f2(&self) -> WrlatF2R {
        WrlatF2R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
DRAM additive latency value in cycles. FC=2"]
    #[inline(always)]
    pub fn additive_lat_f2(&self) -> AdditiveLatF2R {
        AdditiveLatF2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
DRAM CA parity latency value in cycles. FC=2"]
    #[inline(always)]
    pub fn ca_parity_lat_f2(&self) -> CaParityLatF2R {
        CaParityLatF2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Sets latency from read command send to data receive from/to controller. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn caslat_lin_f2(&mut self) -> CaslatLinF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl42Spec> {
        CaslatLinF2W::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
DRAM WRLAT value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn wrlat_f2(&mut self) -> WrlatF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl42Spec> {
        WrlatF2W::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
DRAM additive latency value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn additive_lat_f2(&mut self) -> AdditiveLatF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl42Spec> {
        AdditiveLatF2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
DRAM CA parity latency value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn ca_parity_lat_f2(&mut self) -> CaParityLatF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl42Spec> {
        CaParityLatF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_42::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_42::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl42Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl42Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_42::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl42Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_42::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl42Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_42 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl42Spec {
    const RESET_VALUE: u32 = 0;
}
