#[doc = "Register `CFG_TX_ECC_VAL` reader"]
pub type R = crate::R<CfgTxEccValSpec>;
#[doc = "Register `CFG_TX_ECC_VAL` writer"]
pub type W = crate::W<CfgTxEccValSpec>;
#[doc = "Field `ECC_VAL` reader - 6:0\\]
Computed ECC ValueThis field contains the ECC value computed using SEC-DED either for 16-bit or 32-bit data in the TX_ECC_DATA register."]
pub type EccValR = crate::FieldReader;
#[doc = "Field `ECC_VAL` writer - 6:0\\]
Computed ECC ValueThis field contains the ECC value computed using SEC-DED either for 16-bit or 32-bit data in the TX_ECC_DATA register."]
pub type EccValW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Computed ECC ValueThis field contains the ECC value computed using SEC-DED either for 16-bit or 32-bit data in the TX_ECC_DATA register."]
    #[inline(always)]
    pub fn ecc_val(&self) -> EccValR {
        EccValR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Computed ECC ValueThis field contains the ECC value computed using SEC-DED either for 16-bit or 32-bit data in the TX_ECC_DATA register."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_val(&mut self) -> EccValW<CfgTxEccValSpec> {
        EccValW::new(self, 0)
    }
}
#[doc = "Transmit ECC value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_ecc_val::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_ecc_val::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxEccValSpec;
impl crate::RegisterSpec for CfgTxEccValSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_tx_ecc_val::R`](R) reader structure"]
impl crate::Readable for CfgTxEccValSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_ecc_val::W`](W) writer structure"]
impl crate::Writable for CfgTxEccValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_TX_ECC_VAL to value 0x12"]
impl crate::Resettable for CfgTxEccValSpec {
    const RESET_VALUE: u16 = 0x12;
}
