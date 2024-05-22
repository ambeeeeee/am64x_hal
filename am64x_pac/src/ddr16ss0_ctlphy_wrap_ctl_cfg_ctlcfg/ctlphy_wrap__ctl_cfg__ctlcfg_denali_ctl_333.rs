#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_333` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl333Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_333` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl333Spec>;
#[doc = "Field `DFI_ERROR_INFO` reader - 11:0\\]
Holds the encoded DFI error type associated with the DFI_ERROR parameter assertion. READ-ONLY"]
pub type DfiErrorInfoR = crate::FieldReader<u16>;
#[doc = "Field `DFI_ERROR_INFO` writer - 11:0\\]
Holds the encoded DFI error type associated with the DFI_ERROR parameter assertion. READ-ONLY"]
pub type DfiErrorInfoW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BG_ROTATE_EN` reader - 16:16\\]
Enable bank group rotation. Set to 1 to enable."]
pub type BgRotateEnR = crate::BitReader;
#[doc = "Field `BG_ROTATE_EN` writer - 16:16\\]
Enable bank group rotation. Set to 1 to enable."]
pub type BgRotateEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Holds the encoded DFI error type associated with the DFI_ERROR parameter assertion. READ-ONLY"]
    #[inline(always)]
    pub fn dfi_error_info(&self) -> DfiErrorInfoR {
        DfiErrorInfoR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable bank group rotation. Set to 1 to enable."]
    #[inline(always)]
    pub fn bg_rotate_en(&self) -> BgRotateEnR {
        BgRotateEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Holds the encoded DFI error type associated with the DFI_ERROR parameter assertion. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_error_info(&mut self) -> DfiErrorInfoW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl333Spec> {
        DfiErrorInfoW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable bank group rotation. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn bg_rotate_en(&mut self) -> BgRotateEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl333Spec> {
        BgRotateEnW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_333\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_333::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_333::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl333Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl333Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_333::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl333Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_333::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl333Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_333 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl333Spec {
    const RESET_VALUE: u32 = 0;
}
