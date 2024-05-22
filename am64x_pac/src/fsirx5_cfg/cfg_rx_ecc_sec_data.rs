#[doc = "Register `CFG_RX_ECC_SEC_DATA` reader"]
pub type R = crate::R<CfgRxEccSecDataSpec>;
#[doc = "Register `CFG_RX_ECC_SEC_DATA` writer"]
pub type W = crate::W<CfgRxEccSecDataSpec>;
#[doc = "Field `SEC_DATA` reader - 31:0\\]
ECC Single Error Corrected DataThe ECC corrected data will be available in this register. This value is valid only when there are no bit errors, or a single bit error was detected. Otherwise, the contents of this register are invalid and should not be used."]
pub type SecDataR = crate::FieldReader<u32>;
#[doc = "Field `SEC_DATA` writer - 31:0\\]
ECC Single Error Corrected DataThe ECC corrected data will be available in this register. This value is valid only when there are no bit errors, or a single bit error was detected. Otherwise, the contents of this register are invalid and should not be used."]
pub type SecDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
ECC Single Error Corrected DataThe ECC corrected data will be available in this register. This value is valid only when there are no bit errors, or a single bit error was detected. Otherwise, the contents of this register are invalid and should not be used."]
    #[inline(always)]
    pub fn sec_data(&self) -> SecDataR {
        SecDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
ECC Single Error Corrected DataThe ECC corrected data will be available in this register. This value is valid only when there are no bit errors, or a single bit error was detected. Otherwise, the contents of this register are invalid and should not be used."]
    #[inline(always)]
    #[must_use]
    pub fn sec_data(&mut self) -> SecDataW<CfgRxEccSecDataSpec> {
        SecDataW::new(self, 0)
    }
}
#[doc = "Receive ECC corrected data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_ecc_sec_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_ecc_sec_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxEccSecDataSpec;
impl crate::RegisterSpec for CfgRxEccSecDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_rx_ecc_sec_data::R`](R) reader structure"]
impl crate::Readable for CfgRxEccSecDataSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_ecc_sec_data::W`](W) writer structure"]
impl crate::Writable for CfgRxEccSecDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_RX_ECC_SEC_DATA to value 0"]
impl crate::Resettable for CfgRxEccSecDataSpec {
    const RESET_VALUE: u32 = 0;
}
