#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_software_reset` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2SoftwareResetSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_software_reset` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2SoftwareResetSpec>;
#[doc = "Field `HOST_FULL_RESET` reader - 0:0\\]
On issuing FULL_RESET CCMD, Host Driver set this bit to 1 to reset Host Controller. This bit is cleared auto-matically at completion of Host Controller reset. Initial- ization sequence from PHY Initialization is required to use UHS-II mode. Assuming that bus power is maintained. Host Controller requires to do followings: \\[1\\]
SD Clock Enable is cleared. \\[Internal Clock is still synchronized\\]. \\[2\\]
All setting register is cleared. \\[3\\]
Internal sequencers are reset to just after power on. \\[4\\]
All Interrupt Status, Status Enable and Signal Enable are cleared."]
pub type HostFullResetR = crate::BitReader;
#[doc = "Field `HOST_FULL_RESET` writer - 0:0\\]
On issuing FULL_RESET CCMD, Host Driver set this bit to 1 to reset Host Controller. This bit is cleared auto-matically at completion of Host Controller reset. Initial- ization sequence from PHY Initialization is required to use UHS-II mode. Assuming that bus power is maintained. Host Controller requires to do followings: \\[1\\]
SD Clock Enable is cleared. \\[Internal Clock is still synchronized\\]. \\[2\\]
All setting register is cleared. \\[3\\]
Internal sequencers are reset to just after power on. \\[4\\]
All Interrupt Status, Status Enable and Signal Enable are cleared."]
pub type HostFullResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SDTRAN_RESET` reader - 1:1\\]
Host Driver set this bit to 1 to reset SD-TRAN layer when CMD0 is issued to Device or data transfer error occurs. This bit is cleared automatically at completionof SD-TRAN reset. If CMD0 is issued, SD-TRAN Initial- ization sequence from CMD8 is required to use UHS-II mode. Assuming that bus power is maintained and CM-TRAN Initialization is not required. '0' Not Affected '1' Reset SD TRAN Host Controller requires to do followings: \\[1\\]
SD Clock Enable is maintained.\\[Continue to provide RCLK\\]. \\[2\\]
All setting register is maintained. \\[3\\]
Internal sequencers are reset to just after power on.be able to issue a command. \\[4\\]
All Interrupt Status, Status Enable and Signal Enable are cleared. \\[5\\]
Data transfer is terminated and data in buffer is dis-carded."]
pub type HostSdtranResetR = crate::BitReader;
#[doc = "Field `HOST_SDTRAN_RESET` writer - 1:1\\]
Host Driver set this bit to 1 to reset SD-TRAN layer when CMD0 is issued to Device or data transfer error occurs. This bit is cleared automatically at completionof SD-TRAN reset. If CMD0 is issued, SD-TRAN Initial- ization sequence from CMD8 is required to use UHS-II mode. Assuming that bus power is maintained and CM-TRAN Initialization is not required. '0' Not Affected '1' Reset SD TRAN Host Controller requires to do followings: \\[1\\]
SD Clock Enable is maintained.\\[Continue to provide RCLK\\]. \\[2\\]
All setting register is maintained. \\[3\\]
Internal sequencers are reset to just after power on.be able to issue a command. \\[4\\]
All Interrupt Status, Status Enable and Signal Enable are cleared. \\[5\\]
Data transfer is terminated and data in buffer is dis-carded."]
pub type HostSdtranResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
On issuing FULL_RESET CCMD, Host Driver set this bit to 1 to reset Host Controller. This bit is cleared auto-matically at completion of Host Controller reset. Initial- ization sequence from PHY Initialization is required to use UHS-II mode. Assuming that bus power is maintained. Host Controller requires to do followings: \\[1\\]
SD Clock Enable is cleared. \\[Internal Clock is still synchronized\\]. \\[2\\]
All setting register is cleared. \\[3\\]
Internal sequencers are reset to just after power on. \\[4\\]
All Interrupt Status, Status Enable and Signal Enable are cleared."]
    #[inline(always)]
    pub fn host_full_reset(&self) -> HostFullResetR {
        HostFullResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Host Driver set this bit to 1 to reset SD-TRAN layer when CMD0 is issued to Device or data transfer error occurs. This bit is cleared automatically at completionof SD-TRAN reset. If CMD0 is issued, SD-TRAN Initial- ization sequence from CMD8 is required to use UHS-II mode. Assuming that bus power is maintained and CM-TRAN Initialization is not required. '0' Not Affected '1' Reset SD TRAN Host Controller requires to do followings: \\[1\\]
SD Clock Enable is maintained.\\[Continue to provide RCLK\\]. \\[2\\]
All setting register is maintained. \\[3\\]
Internal sequencers are reset to just after power on.be able to issue a command. \\[4\\]
All Interrupt Status, Status Enable and Signal Enable are cleared. \\[5\\]
Data transfer is terminated and data in buffer is dis-carded."]
    #[inline(always)]
    pub fn host_sdtran_reset(&self) -> HostSdtranResetR {
        HostSdtranResetR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
On issuing FULL_RESET CCMD, Host Driver set this bit to 1 to reset Host Controller. This bit is cleared auto-matically at completion of Host Controller reset. Initial- ization sequence from PHY Initialization is required to use UHS-II mode. Assuming that bus power is maintained. Host Controller requires to do followings: \\[1\\]
SD Clock Enable is cleared. \\[Internal Clock is still synchronized\\]. \\[2\\]
All setting register is cleared. \\[3\\]
Internal sequencers are reset to just after power on. \\[4\\]
All Interrupt Status, Status Enable and Signal Enable are cleared."]
    #[inline(always)]
    #[must_use]
    pub fn host_full_reset(
        &mut self,
    ) -> HostFullResetW<SdhcWrap_CtlCfg_CtlcfgUhs2SoftwareResetSpec> {
        HostFullResetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Host Driver set this bit to 1 to reset SD-TRAN layer when CMD0 is issued to Device or data transfer error occurs. This bit is cleared automatically at completionof SD-TRAN reset. If CMD0 is issued, SD-TRAN Initial- ization sequence from CMD8 is required to use UHS-II mode. Assuming that bus power is maintained and CM-TRAN Initialization is not required. '0' Not Affected '1' Reset SD TRAN Host Controller requires to do followings: \\[1\\]
SD Clock Enable is maintained.\\[Continue to provide RCLK\\]. \\[2\\]
All setting register is maintained. \\[3\\]
Internal sequencers are reset to just after power on.be able to issue a command. \\[4\\]
All Interrupt Status, Status Enable and Signal Enable are cleared. \\[5\\]
Data transfer is terminated and data in buffer is dis-carded."]
    #[inline(always)]
    #[must_use]
    pub fn host_sdtran_reset(
        &mut self,
    ) -> HostSdtranResetW<SdhcWrap_CtlCfg_CtlcfgUhs2SoftwareResetSpec> {
        HostSdtranResetW::new(self, 1)
    }
}
#[doc = "UHS-II Software Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_software_reset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_software_reset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2SoftwareResetSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2SoftwareResetSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_software_reset::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2SoftwareResetSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_software_reset::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2SoftwareResetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_software_reset to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2SoftwareResetSpec {
    const RESET_VALUE: u16 = 0;
}
