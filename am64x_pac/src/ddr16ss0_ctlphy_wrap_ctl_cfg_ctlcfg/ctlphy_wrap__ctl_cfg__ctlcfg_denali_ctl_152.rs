#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_152` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl152Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_152` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl152Spec>;
#[doc = "Field `PPR_ROW_ADDRESS` reader - 16:0\\]
Specifies the encoded row address to be repaired."]
pub type PprRowAddressR = crate::FieldReader<u32>;
#[doc = "Field `PPR_ROW_ADDRESS` writer - 16:0\\]
Specifies the encoded row address to be repaired."]
pub type PprRowAddressW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `PPR_BANK_ADDRESS` reader - 27:24\\]
Specifies the bank for the row to be repaired."]
pub type PprBankAddressR = crate::FieldReader;
#[doc = "Field `PPR_BANK_ADDRESS` writer - 27:24\\]
Specifies the bank for the row to be repaired."]
pub type PprBankAddressW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:16 - 16:0\\]
Specifies the encoded row address to be repaired."]
    #[inline(always)]
    pub fn ppr_row_address(&self) -> PprRowAddressR {
        PprRowAddressR::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Specifies the bank for the row to be repaired."]
    #[inline(always)]
    pub fn ppr_bank_address(&self) -> PprBankAddressR {
        PprBankAddressR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:16 - 16:0\\]
Specifies the encoded row address to be repaired."]
    #[inline(always)]
    #[must_use]
    pub fn ppr_row_address(&mut self) -> PprRowAddressW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl152Spec> {
        PprRowAddressW::new(self, 0)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Specifies the bank for the row to be repaired."]
    #[inline(always)]
    #[must_use]
    pub fn ppr_bank_address(
        &mut self,
    ) -> PprBankAddressW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl152Spec> {
        PprBankAddressW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_152\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_152::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_152::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl152Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl152Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_152::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl152Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_152::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl152Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_152 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl152Spec {
    const RESET_VALUE: u32 = 0;
}
