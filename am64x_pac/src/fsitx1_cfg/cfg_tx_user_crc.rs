#[doc = "Register `CFG_TX_USER_CRC` reader"]
pub type R = crate::R<CfgTxUserCrcSpec>;
#[doc = "Register `CFG_TX_USER_CRC` writer"]
pub type W = crate::W<CfgTxUserCrcSpec>;
#[doc = "Field `USER_CRC` reader - 7:0\\]
User-defined CRCThis register contains the 8-bit CRC value to be transmitted in the next frame if the transmission is set for user-defined CRC option \\[TX_OPER_CTRL_LO.SW_CRC = 1\\]. This register is ignored if the hardware CRC generation is enabled."]
pub type UserCrcR = crate::FieldReader;
#[doc = "Field `USER_CRC` writer - 7:0\\]
User-defined CRCThis register contains the 8-bit CRC value to be transmitted in the next frame if the transmission is set for user-defined CRC option \\[TX_OPER_CTRL_LO.SW_CRC = 1\\]. This register is ignored if the hardware CRC generation is enabled."]
pub type UserCrcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
User-defined CRCThis register contains the 8-bit CRC value to be transmitted in the next frame if the transmission is set for user-defined CRC option \\[TX_OPER_CTRL_LO.SW_CRC = 1\\]. This register is ignored if the hardware CRC generation is enabled."]
    #[inline(always)]
    pub fn user_crc(&self) -> UserCrcR {
        UserCrcR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
User-defined CRCThis register contains the 8-bit CRC value to be transmitted in the next frame if the transmission is set for user-defined CRC option \\[TX_OPER_CTRL_LO.SW_CRC = 1\\]. This register is ignored if the hardware CRC generation is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn user_crc(&mut self) -> UserCrcW<CfgTxUserCrcSpec> {
        UserCrcW::new(self, 0)
    }
}
#[doc = "Transmit user-defined CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_user_crc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_user_crc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxUserCrcSpec;
impl crate::RegisterSpec for CfgTxUserCrcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_tx_user_crc::R`](R) reader structure"]
impl crate::Readable for CfgTxUserCrcSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_user_crc::W`](W) writer structure"]
impl crate::Writable for CfgTxUserCrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_TX_USER_CRC to value 0"]
impl crate::Resettable for CfgTxUserCrcSpec {
    const RESET_VALUE: u16 = 0;
}
