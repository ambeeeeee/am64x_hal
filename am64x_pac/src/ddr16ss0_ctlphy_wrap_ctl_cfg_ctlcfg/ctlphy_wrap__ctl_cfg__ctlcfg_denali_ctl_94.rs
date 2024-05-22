#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_94` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl94Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_94` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl94Spec>;
#[doc = "Field `CA_DEFAULT_VAL_F0` reader - 0:0\\]
Defines how unused address/command bits are driven. Set to 1 to use last value or clear to 0 to drive low. FC=0"]
pub type CaDefaultValF0R = crate::BitReader;
#[doc = "Field `CA_DEFAULT_VAL_F0` writer - 0:0\\]
Defines how unused address/command bits are driven. Set to 1 to use last value or clear to 0 to drive low. FC=0"]
pub type CaDefaultValF0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCSCKE_F1` reader - 12:8\\]
DRAM TCSCKE value in cycles. FC=1"]
pub type TcsckeF1R = crate::FieldReader;
#[doc = "Field `TCSCKE_F1` writer - 12:8\\]
DRAM TCSCKE value in cycles. FC=1"]
pub type TcsckeF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CA_DEFAULT_VAL_F1` reader - 16:16\\]
Defines how unused address/command bits are driven. Set to 1 to use last value or clear to 0 to drive low. FC=1"]
pub type CaDefaultValF1R = crate::BitReader;
#[doc = "Field `CA_DEFAULT_VAL_F1` writer - 16:16\\]
Defines how unused address/command bits are driven. Set to 1 to use last value or clear to 0 to drive low. FC=1"]
pub type CaDefaultValF1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCSCKE_F2` reader - 28:24\\]
DRAM TCSCKE value in cycles. FC=2"]
pub type TcsckeF2R = crate::FieldReader;
#[doc = "Field `TCSCKE_F2` writer - 28:24\\]
DRAM TCSCKE value in cycles. FC=2"]
pub type TcsckeF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Defines how unused address/command bits are driven. Set to 1 to use last value or clear to 0 to drive low. FC=0"]
    #[inline(always)]
    pub fn ca_default_val_f0(&self) -> CaDefaultValF0R {
        CaDefaultValF0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TCSCKE value in cycles. FC=1"]
    #[inline(always)]
    pub fn tcscke_f1(&self) -> TcsckeF1R {
        TcsckeF1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Defines how unused address/command bits are driven. Set to 1 to use last value or clear to 0 to drive low. FC=1"]
    #[inline(always)]
    pub fn ca_default_val_f1(&self) -> CaDefaultValF1R {
        CaDefaultValF1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCSCKE value in cycles. FC=2"]
    #[inline(always)]
    pub fn tcscke_f2(&self) -> TcsckeF2R {
        TcsckeF2R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Defines how unused address/command bits are driven. Set to 1 to use last value or clear to 0 to drive low. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn ca_default_val_f0(
        &mut self,
    ) -> CaDefaultValF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl94Spec> {
        CaDefaultValF0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TCSCKE value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tcscke_f1(&mut self) -> TcsckeF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl94Spec> {
        TcsckeF1W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Defines how unused address/command bits are driven. Set to 1 to use last value or clear to 0 to drive low. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn ca_default_val_f1(
        &mut self,
    ) -> CaDefaultValF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl94Spec> {
        CaDefaultValF1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCSCKE value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tcscke_f2(&mut self) -> TcsckeF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl94Spec> {
        TcsckeF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_94\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_94::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_94::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl94Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl94Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_94::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl94Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_94::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl94Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_94 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl94Spec {
    const RESET_VALUE: u32 = 0;
}
