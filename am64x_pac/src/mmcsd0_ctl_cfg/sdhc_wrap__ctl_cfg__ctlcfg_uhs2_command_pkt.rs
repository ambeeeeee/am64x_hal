#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_command_pkt` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2CommandPktSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_command_pkt` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2CommandPktSpec>;
#[doc = "Field `CMD_PKT_BYTE` reader - 7:0\\]
UHS-II Command Packet image is set to this register.The command length varies depends on a Command Packet type."]
pub type CmdPktByteR = crate::FieldReader;
#[doc = "Field `CMD_PKT_BYTE` writer - 7:0\\]
UHS-II Command Packet image is set to this register.The command length varies depends on a Command Packet type."]
pub type CmdPktByteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
UHS-II Command Packet image is set to this register.The command length varies depends on a Command Packet type."]
    #[inline(always)]
    pub fn cmd_pkt_byte(&self) -> CmdPktByteR {
        CmdPktByteR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
UHS-II Command Packet image is set to this register.The command length varies depends on a Command Packet type."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_pkt_byte(&mut self) -> CmdPktByteW<SdhcWrap_CtlCfg_CtlcfgUhs2CommandPktSpec> {
        CmdPktByteW::new(self, 0)
    }
}
#[doc = "UHS-II Command Packet image is set to this register. The maximum length is 20 bytes. The command length varies depends on a Command Packet type. The length is specified by the UHS-II Command register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command_pkt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command_pkt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2CommandPktSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2CommandPktSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command_pkt::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2CommandPktSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command_pkt::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2CommandPktSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_command_pkt to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2CommandPktSpec {
    const RESET_VALUE: u8 = 0;
}
