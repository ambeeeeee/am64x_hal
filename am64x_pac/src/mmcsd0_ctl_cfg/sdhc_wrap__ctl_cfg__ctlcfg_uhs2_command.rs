#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_command` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2CommandSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_command` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2CommandSpec>;
#[doc = "Field `SUB_COMMAND` reader - 2:2\\]
This bit is added from Version 4.10 to distinguish a main command or sub command \\[Refer to Section 1.17\\].When issuing a main command, this bit is set to 0 and when issuing a sub com-mand, this bit is set to 1. Setting of this bit is checked by Sub Command Status in the Present State register."]
pub type SubCommandR = crate::BitReader;
#[doc = "Field `SUB_COMMAND` writer - 2:2\\]
This bit is added from Version 4.10 to distinguish a main command or sub command \\[Refer to Section 1.17\\].When issuing a main command, this bit is set to 0 and when issuing a sub com-mand, this bit is set to 1. Setting of this bit is checked by Sub Command Status in the Present State register."]
pub type SubCommandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_PRESENT` reader - 5:5\\]
This bit specifies whether the command is accompanied by data packet."]
pub type DataPresentR = crate::BitReader;
#[doc = "Field `DATA_PRESENT` writer - 5:5\\]
This bit specifies whether the command is accompanied by data packet."]
pub type DataPresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_TYPE` reader - 7:6\\]
This field is used to distinguish a spe-cific command like abort command. If this field is set to 00b, the UHS-II RES Packet is stored in UHS-II Response register \\[0B3h-0A0h\\]. To avoid overwrit-ing the UHS-II Response register, when this filed is set to 01b, the RES Packet \\[4 bytes length\\]
of TRANS_ABORT CCMD is stored in the Response regis-ter \\[013h-010h\\]
and when this filed is set to 10b, the RES Packet \\[8 bytes length\\]
of memory or SDIO abort com-mand \\[CMD12 or SDIO Abort com-mand\\]
is stored in the Response register \\[01Fh-018h\\]. When this filed is set to 11b, Host Controller controls lane to go into dormant state. '00' Normal Command '01' TRANS_ABORT CCMD '10' CMD12 or SDIO Abort Command '11' Go Dormant Command"]
pub type CmdTypeR = crate::FieldReader;
#[doc = "Field `CMD_TYPE` writer - 7:6\\]
This field is used to distinguish a spe-cific command like abort command. If this field is set to 00b, the UHS-II RES Packet is stored in UHS-II Response register \\[0B3h-0A0h\\]. To avoid overwrit-ing the UHS-II Response register, when this filed is set to 01b, the RES Packet \\[4 bytes length\\]
of TRANS_ABORT CCMD is stored in the Response regis-ter \\[013h-010h\\]
and when this filed is set to 10b, the RES Packet \\[8 bytes length\\]
of memory or SDIO abort com-mand \\[CMD12 or SDIO Abort com-mand\\]
is stored in the Response register \\[01Fh-018h\\]. When this filed is set to 11b, Host Controller controls lane to go into dormant state. '00' Normal Command '01' TRANS_ABORT CCMD '10' CMD12 or SDIO Abort Command '11' Go Dormant Command"]
pub type CmdTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PKT_LENGTH` reader - 12:8\\]
A command packet length, which is set in the UHS-II Command Packet register, is set to this register. 00011b - 00000b - 3-0 Bytes \\[Not used\\]
00100b - 4 Bytes .......... ...... 10100b - 20 Bytes 11111b 10101b"]
pub type PktLengthR = crate::FieldReader;
#[doc = "Field `PKT_LENGTH` writer - 12:8\\]
A command packet length, which is set in the UHS-II Command Packet register, is set to this register. 00011b - 00000b - 3-0 Bytes \\[Not used\\]
00100b - 4 Bytes .......... ...... 10100b - 20 Bytes 11111b 10101b"]
pub type PktLengthW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 2 - 2:2\\]
This bit is added from Version 4.10 to distinguish a main command or sub command \\[Refer to Section 1.17\\].When issuing a main command, this bit is set to 0 and when issuing a sub com-mand, this bit is set to 1. Setting of this bit is checked by Sub Command Status in the Present State register."]
    #[inline(always)]
    pub fn sub_command(&self) -> SubCommandR {
        SubCommandR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit specifies whether the command is accompanied by data packet."]
    #[inline(always)]
    pub fn data_present(&self) -> DataPresentR {
        DataPresentR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
This field is used to distinguish a spe-cific command like abort command. If this field is set to 00b, the UHS-II RES Packet is stored in UHS-II Response register \\[0B3h-0A0h\\]. To avoid overwrit-ing the UHS-II Response register, when this filed is set to 01b, the RES Packet \\[4 bytes length\\]
of TRANS_ABORT CCMD is stored in the Response regis-ter \\[013h-010h\\]
and when this filed is set to 10b, the RES Packet \\[8 bytes length\\]
of memory or SDIO abort com-mand \\[CMD12 or SDIO Abort com-mand\\]
is stored in the Response register \\[01Fh-018h\\]. When this filed is set to 11b, Host Controller controls lane to go into dormant state. '00' Normal Command '01' TRANS_ABORT CCMD '10' CMD12 or SDIO Abort Command '11' Go Dormant Command"]
    #[inline(always)]
    pub fn cmd_type(&self) -> CmdTypeR {
        CmdTypeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
A command packet length, which is set in the UHS-II Command Packet register, is set to this register. 00011b - 00000b - 3-0 Bytes \\[Not used\\]
00100b - 4 Bytes .......... ...... 10100b - 20 Bytes 11111b 10101b"]
    #[inline(always)]
    pub fn pkt_length(&self) -> PktLengthR {
        PktLengthR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - 2:2\\]
This bit is added from Version 4.10 to distinguish a main command or sub command \\[Refer to Section 1.17\\].When issuing a main command, this bit is set to 0 and when issuing a sub com-mand, this bit is set to 1. Setting of this bit is checked by Sub Command Status in the Present State register."]
    #[inline(always)]
    #[must_use]
    pub fn sub_command(&mut self) -> SubCommandW<SdhcWrap_CtlCfg_CtlcfgUhs2CommandSpec> {
        SubCommandW::new(self, 2)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit specifies whether the command is accompanied by data packet."]
    #[inline(always)]
    #[must_use]
    pub fn data_present(&mut self) -> DataPresentW<SdhcWrap_CtlCfg_CtlcfgUhs2CommandSpec> {
        DataPresentW::new(self, 5)
    }
    #[doc = "Bits 6:7 - 7:6\\]
This field is used to distinguish a spe-cific command like abort command. If this field is set to 00b, the UHS-II RES Packet is stored in UHS-II Response register \\[0B3h-0A0h\\]. To avoid overwrit-ing the UHS-II Response register, when this filed is set to 01b, the RES Packet \\[4 bytes length\\]
of TRANS_ABORT CCMD is stored in the Response regis-ter \\[013h-010h\\]
and when this filed is set to 10b, the RES Packet \\[8 bytes length\\]
of memory or SDIO abort com-mand \\[CMD12 or SDIO Abort com-mand\\]
is stored in the Response register \\[01Fh-018h\\]. When this filed is set to 11b, Host Controller controls lane to go into dormant state. '00' Normal Command '01' TRANS_ABORT CCMD '10' CMD12 or SDIO Abort Command '11' Go Dormant Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_type(&mut self) -> CmdTypeW<SdhcWrap_CtlCfg_CtlcfgUhs2CommandSpec> {
        CmdTypeW::new(self, 6)
    }
    #[doc = "Bits 8:12 - 12:8\\]
A command packet length, which is set in the UHS-II Command Packet register, is set to this register. 00011b - 00000b - 3-0 Bytes \\[Not used\\]
00100b - 4 Bytes .......... ...... 10100b - 20 Bytes 11111b 10101b"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_length(&mut self) -> PktLengthW<SdhcWrap_CtlCfg_CtlcfgUhs2CommandSpec> {
        PktLengthW::new(self, 8)
    }
}
#[doc = "This register is used to program the Command for host controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2CommandSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2CommandSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2CommandSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2CommandSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_command to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2CommandSpec {
    const RESET_VALUE: u16 = 0;
}
