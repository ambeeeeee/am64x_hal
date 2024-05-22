#[doc = "Register `CFG_RX_ECC_LOG` reader"]
pub type R = crate::R<CfgRxEccLogSpec>;
#[doc = "Register `CFG_RX_ECC_LOG` writer"]
pub type W = crate::W<CfgRxEccLogSpec>;
#[doc = "Field `SBE` reader - 0:0\\]
Single Bit Error DetectedThis bit indicates the occurrence of a single bit error in the data. The data is autocorrected and placed into the RX_ECC_SEC_DATA register. This bit is valid only if MBE is 0. 0h \\[R\\]
No bit errors were detected. The value in RX_ECC_SEC_DATA is correct. 1h \\[R\\]
A single bit error was detected and corrected. The corrected data is present in RX_ECC_SEC_DATA."]
pub type SbeR = crate::BitReader;
#[doc = "Field `SBE` writer - 0:0\\]
Single Bit Error DetectedThis bit indicates the occurrence of a single bit error in the data. The data is autocorrected and placed into the RX_ECC_SEC_DATA register. This bit is valid only if MBE is 0. 0h \\[R\\]
No bit errors were detected. The value in RX_ECC_SEC_DATA is correct. 1h \\[R\\]
A single bit error was detected and corrected. The corrected data is present in RX_ECC_SEC_DATA."]
pub type SbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBE` reader - 1:1\\]
Multiple Bit Errors DetectedThis bit indicates the occurrence of multiple bit errors.The data is corrupted and cannot be corrected. If this bit is set, the data present in RX_ECC_SEC_DATA is invalid and should not be used. 0h \\[R\\]
Multiple Bit Errors were not detected. Check the SBE bit for single bit errors.1h \\[R\\]
Multiple Bit Errors were detected. The data is not able to be corrected. The value present in RX_ECC_SEC_DATA is invalid and should not be used."]
pub type MbeR = crate::BitReader;
#[doc = "Field `MBE` writer - 1:1\\]
Multiple Bit Errors DetectedThis bit indicates the occurrence of multiple bit errors.The data is corrupted and cannot be corrected. If this bit is set, the data present in RX_ECC_SEC_DATA is invalid and should not be used. 0h \\[R\\]
Multiple Bit Errors were not detected. Check the SBE bit for single bit errors.1h \\[R\\]
Multiple Bit Errors were detected. The data is not able to be corrected. The value present in RX_ECC_SEC_DATA is invalid and should not be used."]
pub type MbeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Single Bit Error DetectedThis bit indicates the occurrence of a single bit error in the data. The data is autocorrected and placed into the RX_ECC_SEC_DATA register. This bit is valid only if MBE is 0. 0h \\[R\\]
No bit errors were detected. The value in RX_ECC_SEC_DATA is correct. 1h \\[R\\]
A single bit error was detected and corrected. The corrected data is present in RX_ECC_SEC_DATA."]
    #[inline(always)]
    pub fn sbe(&self) -> SbeR {
        SbeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Multiple Bit Errors DetectedThis bit indicates the occurrence of multiple bit errors.The data is corrupted and cannot be corrected. If this bit is set, the data present in RX_ECC_SEC_DATA is invalid and should not be used. 0h \\[R\\]
Multiple Bit Errors were not detected. Check the SBE bit for single bit errors.1h \\[R\\]
Multiple Bit Errors were detected. The data is not able to be corrected. The value present in RX_ECC_SEC_DATA is invalid and should not be used."]
    #[inline(always)]
    pub fn mbe(&self) -> MbeR {
        MbeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Single Bit Error DetectedThis bit indicates the occurrence of a single bit error in the data. The data is autocorrected and placed into the RX_ECC_SEC_DATA register. This bit is valid only if MBE is 0. 0h \\[R\\]
No bit errors were detected. The value in RX_ECC_SEC_DATA is correct. 1h \\[R\\]
A single bit error was detected and corrected. The corrected data is present in RX_ECC_SEC_DATA."]
    #[inline(always)]
    #[must_use]
    pub fn sbe(&mut self) -> SbeW<CfgRxEccLogSpec> {
        SbeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Multiple Bit Errors DetectedThis bit indicates the occurrence of multiple bit errors.The data is corrupted and cannot be corrected. If this bit is set, the data present in RX_ECC_SEC_DATA is invalid and should not be used. 0h \\[R\\]
Multiple Bit Errors were not detected. Check the SBE bit for single bit errors.1h \\[R\\]
Multiple Bit Errors were detected. The data is not able to be corrected. The value present in RX_ECC_SEC_DATA is invalid and should not be used."]
    #[inline(always)]
    #[must_use]
    pub fn mbe(&mut self) -> MbeW<CfgRxEccLogSpec> {
        MbeW::new(self, 1)
    }
}
#[doc = "Receive ECC log and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_ecc_log::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_ecc_log::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxEccLogSpec;
impl crate::RegisterSpec for CfgRxEccLogSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_ecc_log::R`](R) reader structure"]
impl crate::Readable for CfgRxEccLogSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_ecc_log::W`](W) writer structure"]
impl crate::Writable for CfgRxEccLogSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_ECC_LOG to value 0x03"]
impl crate::Resettable for CfgRxEccLogSpec {
    const RESET_VALUE: u16 = 0x03;
}
