#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_95` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl95Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_95` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl95Spec>;
#[doc = "Field `CA_DEFAULT_VAL_F2` reader - 0:0\\]
Defines how unused address/command bits are driven. Set to 1 to use last value or clear to 0 to drive low. FC=2"]
pub type CaDefaultValF2R = crate::BitReader;
#[doc = "Field `CA_DEFAULT_VAL_F2` writer - 0:0\\]
Defines how unused address/command bits are driven. Set to 1 to use last value or clear to 0 to drive low. FC=2"]
pub type CaDefaultValF2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSR_F0` reader - 23:8\\]
DRAM TXSR value in cycles. FC=0"]
pub type TxsrF0R = crate::FieldReader<u16>;
#[doc = "Field `TXSR_F0` writer - 23:8\\]
DRAM TXSR value in cycles. FC=0"]
pub type TxsrF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Defines how unused address/command bits are driven. Set to 1 to use last value or clear to 0 to drive low. FC=2"]
    #[inline(always)]
    pub fn ca_default_val_f2(&self) -> CaDefaultValF2R {
        CaDefaultValF2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
DRAM TXSR value in cycles. FC=0"]
    #[inline(always)]
    pub fn txsr_f0(&self) -> TxsrF0R {
        TxsrF0R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Defines how unused address/command bits are driven. Set to 1 to use last value or clear to 0 to drive low. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn ca_default_val_f2(
        &mut self,
    ) -> CaDefaultValF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl95Spec> {
        CaDefaultValF2W::new(self, 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
DRAM TXSR value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn txsr_f0(&mut self) -> TxsrF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl95Spec> {
        TxsrF0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_95\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_95::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_95::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl95Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl95Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_95::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl95Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_95::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl95Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_95 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl95Spec {
    const RESET_VALUE: u32 = 0;
}
