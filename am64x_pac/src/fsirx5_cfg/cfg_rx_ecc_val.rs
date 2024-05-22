#[doc = "Register `CFG_RX_ECC_VAL` reader"]
pub type R = crate::R<CfgRxEccValSpec>;
#[doc = "Register `CFG_RX_ECC_VAL` writer"]
pub type W = crate::W<CfgRxEccValSpec>;
#[doc = "Field `ECC_VAL` reader - 6:0\\]
ECC Value for SEC-DED checkThis field contains the ECC value to be used for SEC-DED either for 16-bit or 32-bit data in the RX_ECC_DATA register."]
pub type EccValR = crate::FieldReader;
#[doc = "Field `ECC_VAL` writer - 6:0\\]
ECC Value for SEC-DED checkThis field contains the ECC value to be used for SEC-DED either for 16-bit or 32-bit data in the RX_ECC_DATA register."]
pub type EccValW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
ECC Value for SEC-DED checkThis field contains the ECC value to be used for SEC-DED either for 16-bit or 32-bit data in the RX_ECC_DATA register."]
    #[inline(always)]
    pub fn ecc_val(&self) -> EccValR {
        EccValR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
ECC Value for SEC-DED checkThis field contains the ECC value to be used for SEC-DED either for 16-bit or 32-bit data in the RX_ECC_DATA register."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_val(&mut self) -> EccValW<CfgRxEccValSpec> {
        EccValW::new(self, 0)
    }
}
#[doc = "Receive ECC value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_ecc_val::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_ecc_val::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxEccValSpec;
impl crate::RegisterSpec for CfgRxEccValSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_ecc_val::R`](R) reader structure"]
impl crate::Readable for CfgRxEccValSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_ecc_val::W`](W) writer structure"]
impl crate::Writable for CfgRxEccValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_ECC_VAL to value 0"]
impl crate::Resettable for CfgRxEccValSpec {
    const RESET_VALUE: u16 = 0;
}
