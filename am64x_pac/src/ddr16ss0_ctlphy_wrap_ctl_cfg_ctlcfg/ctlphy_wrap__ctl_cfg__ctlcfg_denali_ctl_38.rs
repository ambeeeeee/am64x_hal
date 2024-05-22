#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_38` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl38Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_38` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl38Spec>;
#[doc = "Field `CASLAT_LIN_F0` reader - 6:0\\]
Sets latency from read command send to data receive from/to controller. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller. FC=0"]
pub type CaslatLinF0R = crate::FieldReader;
#[doc = "Field `CASLAT_LIN_F0` writer - 6:0\\]
Sets latency from read command send to data receive from/to controller. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller. FC=0"]
pub type CaslatLinF0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRLAT_F0` reader - 14:8\\]
DRAM WRLAT value in cycles. FC=0"]
pub type WrlatF0R = crate::FieldReader;
#[doc = "Field `WRLAT_F0` writer - 14:8\\]
DRAM WRLAT value in cycles. FC=0"]
pub type WrlatF0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADDITIVE_LAT_F0` reader - 21:16\\]
DRAM additive latency value in cycles. FC=0"]
pub type AdditiveLatF0R = crate::FieldReader;
#[doc = "Field `ADDITIVE_LAT_F0` writer - 21:16\\]
DRAM additive latency value in cycles. FC=0"]
pub type AdditiveLatF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CA_PARITY_LAT_F0` reader - 27:24\\]
DRAM CA parity latency value in cycles. FC=0"]
pub type CaParityLatF0R = crate::FieldReader;
#[doc = "Field `CA_PARITY_LAT_F0` writer - 27:24\\]
DRAM CA parity latency value in cycles. FC=0"]
pub type CaParityLatF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Sets latency from read command send to data receive from/to controller. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller. FC=0"]
    #[inline(always)]
    pub fn caslat_lin_f0(&self) -> CaslatLinF0R {
        CaslatLinF0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
DRAM WRLAT value in cycles. FC=0"]
    #[inline(always)]
    pub fn wrlat_f0(&self) -> WrlatF0R {
        WrlatF0R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
DRAM additive latency value in cycles. FC=0"]
    #[inline(always)]
    pub fn additive_lat_f0(&self) -> AdditiveLatF0R {
        AdditiveLatF0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
DRAM CA parity latency value in cycles. FC=0"]
    #[inline(always)]
    pub fn ca_parity_lat_f0(&self) -> CaParityLatF0R {
        CaParityLatF0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Sets latency from read command send to data receive from/to controller. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn caslat_lin_f0(&mut self) -> CaslatLinF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl38Spec> {
        CaslatLinF0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
DRAM WRLAT value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn wrlat_f0(&mut self) -> WrlatF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl38Spec> {
        WrlatF0W::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
DRAM additive latency value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn additive_lat_f0(&mut self) -> AdditiveLatF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl38Spec> {
        AdditiveLatF0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
DRAM CA parity latency value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn ca_parity_lat_f0(&mut self) -> CaParityLatF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl38Spec> {
        CaParityLatF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_38::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_38::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl38Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_38::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl38Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_38::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl38Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_38 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl38Spec {
    const RESET_VALUE: u32 = 0;
}
