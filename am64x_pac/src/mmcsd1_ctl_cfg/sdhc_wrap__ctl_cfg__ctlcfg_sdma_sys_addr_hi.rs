#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_sdma_sys_addr_hi` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrHiSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_sdma_sys_addr_hi` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrHiSpec>;
#[doc = "Field `SDMA_ADDRESS` reader - 15:0\\]
This register contains the Upper 16-bit of physical system memory address used for DMA transfers or the second argument for the Auto CMD23 in Host version 3.0 and as 32-bit Block Count in Version 4.10."]
pub type SdmaAddressR = crate::FieldReader<u16>;
#[doc = "Field `SDMA_ADDRESS` writer - 15:0\\]
This register contains the Upper 16-bit of physical system memory address used for DMA transfers or the second argument for the Auto CMD23 in Host version 3.0 and as 32-bit Block Count in Version 4.10."]
pub type SdmaAddressW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
This register contains the Upper 16-bit of physical system memory address used for DMA transfers or the second argument for the Auto CMD23 in Host version 3.0 and as 32-bit Block Count in Version 4.10."]
    #[inline(always)]
    pub fn sdma_address(&self) -> SdmaAddressR {
        SdmaAddressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
This register contains the Upper 16-bit of physical system memory address used for DMA transfers or the second argument for the Auto CMD23 in Host version 3.0 and as 32-bit Block Count in Version 4.10."]
    #[inline(always)]
    #[must_use]
    pub fn sdma_address(&mut self) -> SdmaAddressW<SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrHiSpec> {
        SdmaAddressW::new(self, 0)
    }
}
#[doc = "This register contains the Upper 16-bit of physical system memory address used for DMA transfers or the second argument for the Auto CMD23 in Host version 3.0 and as 32-bit Block Count in Version 4.10.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrHiSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrHiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_hi::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrHiSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_hi::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrHiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_sdma_sys_addr_hi to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrHiSpec {
    const RESET_VALUE: u16 = 0;
}
