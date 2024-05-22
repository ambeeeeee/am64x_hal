#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_332` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl332Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_332` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl332Spec>;
#[doc = "Field `WR_DBI_EN` reader - 0:0\\]
Enables controller support of DRAM DBI feature for write data with DDR4 devices. Set to 1 to enable."]
pub type WrDbiEnR = crate::BitReader;
#[doc = "Field `WR_DBI_EN` writer - 0:0\\]
Enables controller support of DRAM DBI feature for write data with DDR4 devices. Set to 1 to enable."]
pub type WrDbiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_DBI_EN` reader - 8:8\\]
Enables controller support of DRAM DBI feature for read data with DDR4 devices. Set to 1 to enable."]
pub type RdDbiEnR = crate::BitReader;
#[doc = "Field `RD_DBI_EN` writer - 8:8\\]
Enables controller support of DRAM DBI feature for read data with DDR4 devices. Set to 1 to enable."]
pub type RdDbiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFI_ERROR` reader - 18:16\\]
Indicates that the DFI error flag has been asserted. READ-ONLY"]
pub type DfiErrorR = crate::FieldReader;
#[doc = "Field `DFI_ERROR` writer - 18:16\\]
Indicates that the DFI error flag has been asserted. READ-ONLY"]
pub type DfiErrorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables controller support of DRAM DBI feature for write data with DDR4 devices. Set to 1 to enable."]
    #[inline(always)]
    pub fn wr_dbi_en(&self) -> WrDbiEnR {
        WrDbiEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables controller support of DRAM DBI feature for read data with DDR4 devices. Set to 1 to enable."]
    #[inline(always)]
    pub fn rd_dbi_en(&self) -> RdDbiEnR {
        RdDbiEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Indicates that the DFI error flag has been asserted. READ-ONLY"]
    #[inline(always)]
    pub fn dfi_error(&self) -> DfiErrorR {
        DfiErrorR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables controller support of DRAM DBI feature for write data with DDR4 devices. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn wr_dbi_en(&mut self) -> WrDbiEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl332Spec> {
        WrDbiEnW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables controller support of DRAM DBI feature for read data with DDR4 devices. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn rd_dbi_en(&mut self) -> RdDbiEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl332Spec> {
        RdDbiEnW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Indicates that the DFI error flag has been asserted. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_error(&mut self) -> DfiErrorW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl332Spec> {
        DfiErrorW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_332\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_332::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_332::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl332Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl332Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_332::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl332Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_332::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl332Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_332 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl332Spec {
    const RESET_VALUE: u32 = 0;
}
