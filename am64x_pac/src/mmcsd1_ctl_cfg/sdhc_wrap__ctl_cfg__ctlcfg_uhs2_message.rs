#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_message` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2MessageSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_message` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2MessageSpec>;
#[doc = "Field `MSG_BYTE0` reader - 7:0\\]
Host Controller holds 4 MSG packets in FIFO buffer. One of 4 MSGs \\[length is 4 bytes\\]
can be read fromthis register by setting UHS-II MSG Select register. Usually 2 duplicate MSG packets are sent from/toUHS-II card. One of these 2 MSG packets which Host Controller recognizes as valid one is stored in theUHS-II MSG Register."]
pub type MsgByte0R = crate::FieldReader;
#[doc = "Field `MSG_BYTE0` writer - 7:0\\]
Host Controller holds 4 MSG packets in FIFO buffer. One of 4 MSGs \\[length is 4 bytes\\]
can be read fromthis register by setting UHS-II MSG Select register. Usually 2 duplicate MSG packets are sent from/toUHS-II card. One of these 2 MSG packets which Host Controller recognizes as valid one is stored in theUHS-II MSG Register."]
pub type MsgByte0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MSG_BYTE1` reader - 15:8\\]
Host Controller holds 4 MSG packets in FIFO buffer. One of 4 MSGs \\[length is 4 bytes\\]
can be read fromthis register by setting UHS-II MSG Select register. Usually 2 duplicate MSG packets are sent from/toUHS-II card. One of these 2 MSG packets which Host Controller recognizes as valid one is stored in theUHS-II MSG Register."]
pub type MsgByte1R = crate::FieldReader;
#[doc = "Field `MSG_BYTE1` writer - 15:8\\]
Host Controller holds 4 MSG packets in FIFO buffer. One of 4 MSGs \\[length is 4 bytes\\]
can be read fromthis register by setting UHS-II MSG Select register. Usually 2 duplicate MSG packets are sent from/toUHS-II card. One of these 2 MSG packets which Host Controller recognizes as valid one is stored in theUHS-II MSG Register."]
pub type MsgByte1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MSG_BYTE2` reader - 23:16\\]
Host Controller holds 4 MSG packets in FIFO buffer. One of 4 MSGs \\[length is 4 bytes\\]
can be read fromthis register by setting UHS-II MSG Select register. Usually 2 duplicate MSG packets are sent from/toUHS-II card. One of these 2 MSG packets which Host Controller recognizes as valid one is stored in theUHS-II MSG Register."]
pub type MsgByte2R = crate::FieldReader;
#[doc = "Field `MSG_BYTE2` writer - 23:16\\]
Host Controller holds 4 MSG packets in FIFO buffer. One of 4 MSGs \\[length is 4 bytes\\]
can be read fromthis register by setting UHS-II MSG Select register. Usually 2 duplicate MSG packets are sent from/toUHS-II card. One of these 2 MSG packets which Host Controller recognizes as valid one is stored in theUHS-II MSG Register."]
pub type MsgByte2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MSG_BYTE3` reader - 31:24\\]
Host Controller holds 4 MSG packets in FIFO buffer. One of 4 MSGs \\[length is 4 bytes\\]
can be read fromthis register by setting UHS-II MSG Select register. Usually 2 duplicate MSG packets are sent from/toUHS-II card. One of these 2 MSG packets which Host Controller recognizes as valid one is stored in theUHS-II MSG Register."]
pub type MsgByte3R = crate::FieldReader;
#[doc = "Field `MSG_BYTE3` writer - 31:24\\]
Host Controller holds 4 MSG packets in FIFO buffer. One of 4 MSGs \\[length is 4 bytes\\]
can be read fromthis register by setting UHS-II MSG Select register. Usually 2 duplicate MSG packets are sent from/toUHS-II card. One of these 2 MSG packets which Host Controller recognizes as valid one is stored in theUHS-II MSG Register."]
pub type MsgByte3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Host Controller holds 4 MSG packets in FIFO buffer. One of 4 MSGs \\[length is 4 bytes\\]
can be read fromthis register by setting UHS-II MSG Select register. Usually 2 duplicate MSG packets are sent from/toUHS-II card. One of these 2 MSG packets which Host Controller recognizes as valid one is stored in theUHS-II MSG Register."]
    #[inline(always)]
    pub fn msg_byte0(&self) -> MsgByte0R {
        MsgByte0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Host Controller holds 4 MSG packets in FIFO buffer. One of 4 MSGs \\[length is 4 bytes\\]
can be read fromthis register by setting UHS-II MSG Select register. Usually 2 duplicate MSG packets are sent from/toUHS-II card. One of these 2 MSG packets which Host Controller recognizes as valid one is stored in theUHS-II MSG Register."]
    #[inline(always)]
    pub fn msg_byte1(&self) -> MsgByte1R {
        MsgByte1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Host Controller holds 4 MSG packets in FIFO buffer. One of 4 MSGs \\[length is 4 bytes\\]
can be read fromthis register by setting UHS-II MSG Select register. Usually 2 duplicate MSG packets are sent from/toUHS-II card. One of these 2 MSG packets which Host Controller recognizes as valid one is stored in theUHS-II MSG Register."]
    #[inline(always)]
    pub fn msg_byte2(&self) -> MsgByte2R {
        MsgByte2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Host Controller holds 4 MSG packets in FIFO buffer. One of 4 MSGs \\[length is 4 bytes\\]
can be read fromthis register by setting UHS-II MSG Select register. Usually 2 duplicate MSG packets are sent from/toUHS-II card. One of these 2 MSG packets which Host Controller recognizes as valid one is stored in theUHS-II MSG Register."]
    #[inline(always)]
    pub fn msg_byte3(&self) -> MsgByte3R {
        MsgByte3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Host Controller holds 4 MSG packets in FIFO buffer. One of 4 MSGs \\[length is 4 bytes\\]
can be read fromthis register by setting UHS-II MSG Select register. Usually 2 duplicate MSG packets are sent from/toUHS-II card. One of these 2 MSG packets which Host Controller recognizes as valid one is stored in theUHS-II MSG Register."]
    #[inline(always)]
    #[must_use]
    pub fn msg_byte0(&mut self) -> MsgByte0W<SdhcWrap_CtlCfg_CtlcfgUhs2MessageSpec> {
        MsgByte0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Host Controller holds 4 MSG packets in FIFO buffer. One of 4 MSGs \\[length is 4 bytes\\]
can be read fromthis register by setting UHS-II MSG Select register. Usually 2 duplicate MSG packets are sent from/toUHS-II card. One of these 2 MSG packets which Host Controller recognizes as valid one is stored in theUHS-II MSG Register."]
    #[inline(always)]
    #[must_use]
    pub fn msg_byte1(&mut self) -> MsgByte1W<SdhcWrap_CtlCfg_CtlcfgUhs2MessageSpec> {
        MsgByte1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Host Controller holds 4 MSG packets in FIFO buffer. One of 4 MSGs \\[length is 4 bytes\\]
can be read fromthis register by setting UHS-II MSG Select register. Usually 2 duplicate MSG packets are sent from/toUHS-II card. One of these 2 MSG packets which Host Controller recognizes as valid one is stored in theUHS-II MSG Register."]
    #[inline(always)]
    #[must_use]
    pub fn msg_byte2(&mut self) -> MsgByte2W<SdhcWrap_CtlCfg_CtlcfgUhs2MessageSpec> {
        MsgByte2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Host Controller holds 4 MSG packets in FIFO buffer. One of 4 MSGs \\[length is 4 bytes\\]
can be read fromthis register by setting UHS-II MSG Select register. Usually 2 duplicate MSG packets are sent from/toUHS-II card. One of these 2 MSG packets which Host Controller recognizes as valid one is stored in theUHS-II MSG Register."]
    #[inline(always)]
    #[must_use]
    pub fn msg_byte3(&mut self) -> MsgByte3W<SdhcWrap_CtlCfg_CtlcfgUhs2MessageSpec> {
        MsgByte3W::new(self, 24)
    }
}
#[doc = "This register is used to access internal buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2MessageSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2MessageSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2MessageSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2MessageSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_message to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2MessageSpec {
    const RESET_VALUE: u32 = 0;
}
