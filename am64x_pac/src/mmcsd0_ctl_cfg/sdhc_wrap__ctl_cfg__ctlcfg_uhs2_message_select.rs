#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_message_select` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2MessageSelectSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_message_select` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2MessageSelectSpec>;
#[doc = "Field `MSG_SEL` reader - 1:0\\]
Host Controller holds 4 MSG packets in FIFO buffer.One of 4 MSGs can be read from the UHS-II MSG register \\[0BB-0B8h\\]
by setting this register.\\[Assumed for debug usage.\\]
'00' The latest MSG '01' One MSG before '10' Two MSGs before '11' Three MSGs before"]
pub type MsgSelR = crate::FieldReader;
#[doc = "Field `MSG_SEL` writer - 1:0\\]
Host Controller holds 4 MSG packets in FIFO buffer.One of 4 MSGs can be read from the UHS-II MSG register \\[0BB-0B8h\\]
by setting this register.\\[Assumed for debug usage.\\]
'00' The latest MSG '01' One MSG before '10' Two MSGs before '11' Three MSGs before"]
pub type MsgSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Host Controller holds 4 MSG packets in FIFO buffer.One of 4 MSGs can be read from the UHS-II MSG register \\[0BB-0B8h\\]
by setting this register.\\[Assumed for debug usage.\\]
'00' The latest MSG '01' One MSG before '10' Two MSGs before '11' Three MSGs before"]
    #[inline(always)]
    pub fn msg_sel(&self) -> MsgSelR {
        MsgSelR::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Host Controller holds 4 MSG packets in FIFO buffer.One of 4 MSGs can be read from the UHS-II MSG register \\[0BB-0B8h\\]
by setting this register.\\[Assumed for debug usage.\\]
'00' The latest MSG '01' One MSG before '10' Two MSGs before '11' Three MSGs before"]
    #[inline(always)]
    #[must_use]
    pub fn msg_sel(&mut self) -> MsgSelW<SdhcWrap_CtlCfg_CtlcfgUhs2MessageSelectSpec> {
        MsgSelW::new(self, 0)
    }
}
#[doc = "This register is used to access internal buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message_select::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message_select::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2MessageSelectSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2MessageSelectSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message_select::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2MessageSelectSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message_select::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2MessageSelectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_message_select to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2MessageSelectSpec {
    const RESET_VALUE: u8 = 0;
}
