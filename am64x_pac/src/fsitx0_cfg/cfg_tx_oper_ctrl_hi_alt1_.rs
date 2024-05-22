#[doc = "Register `CFG_TX_OPER_CTRL_HI_ALT1_` reader"]
pub type R = crate::R<CfgTxOperCtrlHiAlt1_Spec>;
#[doc = "Register `CFG_TX_OPER_CTRL_HI_ALT1_` writer"]
pub type W = crate::W<CfgTxOperCtrlHiAlt1_Spec>;
#[doc = "Field `FORCE_ERR` reader - 5:5\\]
Error Frame Force bitThis bit will force the the CRC value of the transmitted data frame to 0 whenever there is a buffer overrun or underrun condition. This can be used to force a corrupted CRC as the data is not guaranteed to be reliable. The receiver will treat the data as invalid and can handle this as needed.Note: DO NOT use FORCE_ERR if using the SW CRC mode \\[FSI Transmit\\]. 0h \\[R/W\\]
= The CRC will not be forced to 0.1h \\[R/W\\]
= The CRC will be forced to 0 in a buffer overrun or underrun condition."]
pub type ForceErrR = crate::BitReader;
#[doc = "Field `FORCE_ERR` writer - 5:5\\]
Error Frame Force bitThis bit will force the the CRC value of the transmitted data frame to 0 whenever there is a buffer overrun or underrun condition. This can be used to force a corrupted CRC as the data is not guaranteed to be reliable. The receiver will treat the data as invalid and can handle this as needed.Note: DO NOT use FORCE_ERR if using the SW CRC mode \\[FSI Transmit\\]. 0h \\[R/W\\]
= The CRC will not be forced to 0.1h \\[R/W\\]
= The CRC will be forced to 0 in a buffer overrun or underrun condition."]
pub type ForceErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_SEL` reader - 6:6\\]
ECC Data Width Select bitThis bit selects between 16-bit and 32-bit ECC computation. 0h \\[R/W\\]
= 32-bit ECC is used.1h \\[R/W\\]
= 16-bit ECC is used."]
pub type EccSelR = crate::BitReader;
#[doc = "Field `ECC_SEL` writer - 6:6\\]
ECC Data Width Select bitThis bit selects between 16-bit and 32-bit ECC computation. 0h \\[R/W\\]
= 32-bit ECC is used.1h \\[R/W\\]
= 16-bit ECC is used."]
pub type EccSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_TRIG_SEL` reader - 12:7\\]
External Trigger Select bitThese bits define which of the 32 external inputs will be used as the source for the external input trigger. 00h \\[R/W\\]
= Trigger 1 is the source.01h \\[R/W\\]
= Trigger 2 is the source.02h \\[R/W\\]
= Trigger 3 is the source....3Fh \\[R/W\\]
= Trigger 64 is the source."]
pub type ExtTrigSelR = crate::FieldReader;
#[doc = "Field `EXT_TRIG_SEL` writer - 12:7\\]
External Trigger Select bitThese bits define which of the 32 external inputs will be used as the source for the external input trigger. 00h \\[R/W\\]
= Trigger 1 is the source.01h \\[R/W\\]
= Trigger 2 is the source.02h \\[R/W\\]
= Trigger 3 is the source....3Fh \\[R/W\\]
= Trigger 64 is the source."]
pub type ExtTrigSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 5 - 5:5\\]
Error Frame Force bitThis bit will force the the CRC value of the transmitted data frame to 0 whenever there is a buffer overrun or underrun condition. This can be used to force a corrupted CRC as the data is not guaranteed to be reliable. The receiver will treat the data as invalid and can handle this as needed.Note: DO NOT use FORCE_ERR if using the SW CRC mode \\[FSI Transmit\\]. 0h \\[R/W\\]
= The CRC will not be forced to 0.1h \\[R/W\\]
= The CRC will be forced to 0 in a buffer overrun or underrun condition."]
    #[inline(always)]
    pub fn force_err(&self) -> ForceErrR {
        ForceErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
ECC Data Width Select bitThis bit selects between 16-bit and 32-bit ECC computation. 0h \\[R/W\\]
= 32-bit ECC is used.1h \\[R/W\\]
= 16-bit ECC is used."]
    #[inline(always)]
    pub fn ecc_sel(&self) -> EccSelR {
        EccSelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:12 - 12:7\\]
External Trigger Select bitThese bits define which of the 32 external inputs will be used as the source for the external input trigger. 00h \\[R/W\\]
= Trigger 1 is the source.01h \\[R/W\\]
= Trigger 2 is the source.02h \\[R/W\\]
= Trigger 3 is the source....3Fh \\[R/W\\]
= Trigger 64 is the source."]
    #[inline(always)]
    pub fn ext_trig_sel(&self) -> ExtTrigSelR {
        ExtTrigSelR::new(((self.bits >> 7) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - 5:5\\]
Error Frame Force bitThis bit will force the the CRC value of the transmitted data frame to 0 whenever there is a buffer overrun or underrun condition. This can be used to force a corrupted CRC as the data is not guaranteed to be reliable. The receiver will treat the data as invalid and can handle this as needed.Note: DO NOT use FORCE_ERR if using the SW CRC mode \\[FSI Transmit\\]. 0h \\[R/W\\]
= The CRC will not be forced to 0.1h \\[R/W\\]
= The CRC will be forced to 0 in a buffer overrun or underrun condition."]
    #[inline(always)]
    #[must_use]
    pub fn force_err(&mut self) -> ForceErrW<CfgTxOperCtrlHiAlt1_Spec> {
        ForceErrW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
ECC Data Width Select bitThis bit selects between 16-bit and 32-bit ECC computation. 0h \\[R/W\\]
= 32-bit ECC is used.1h \\[R/W\\]
= 16-bit ECC is used."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_sel(&mut self) -> EccSelW<CfgTxOperCtrlHiAlt1_Spec> {
        EccSelW::new(self, 6)
    }
    #[doc = "Bits 7:12 - 12:7\\]
External Trigger Select bitThese bits define which of the 32 external inputs will be used as the source for the external input trigger. 00h \\[R/W\\]
= Trigger 1 is the source.01h \\[R/W\\]
= Trigger 2 is the source.02h \\[R/W\\]
= Trigger 3 is the source....3Fh \\[R/W\\]
= Trigger 64 is the source."]
    #[inline(always)]
    #[must_use]
    pub fn ext_trig_sel(&mut self) -> ExtTrigSelW<CfgTxOperCtrlHiAlt1_Spec> {
        ExtTrigSelW::new(self, 7)
    }
}
#[doc = "Transmit operation control register high. Protected by LOCK field in TX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_oper_ctrl_hi_alt1_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_oper_ctrl_hi_alt1_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxOperCtrlHiAlt1_Spec;
impl crate::RegisterSpec for CfgTxOperCtrlHiAlt1_Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_tx_oper_ctrl_hi_alt1_::R`](R) reader structure"]
impl crate::Readable for CfgTxOperCtrlHiAlt1_Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_oper_ctrl_hi_alt1_::W`](W) writer structure"]
impl crate::Writable for CfgTxOperCtrlHiAlt1_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_TX_OPER_CTRL_HI_ALT1_ to value 0"]
impl crate::Resettable for CfgTxOperCtrlHiAlt1_Spec {
    const RESET_VALUE: u16 = 0;
}
