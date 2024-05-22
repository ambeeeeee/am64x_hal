#[doc = "Register `CFG_TX_ECC_DATA` reader"]
pub type R = crate::R<CfgTxEccDataSpec>;
#[doc = "Register `CFG_TX_ECC_DATA` writer"]
pub type W = crate::W<CfgTxEccDataSpec>;
#[doc = "Field `DATA_LOW` reader - 15:0\\]
Lower 16 bits of ECC DataWriting to this bitfield will cause the ECC logic to compute the ECC\\[SEC-DED\\]
for these 16 bits and update the TX_ECC_VAL register with the results. Software should write to these register bits as a 16-bit write when needing to compute ECC for 16-bits."]
pub type DataLowR = crate::FieldReader<u16>;
#[doc = "Field `DATA_LOW` writer - 15:0\\]
Lower 16 bits of ECC DataWriting to this bitfield will cause the ECC logic to compute the ECC\\[SEC-DED\\]
for these 16 bits and update the TX_ECC_VAL register with the results. Software should write to these register bits as a 16-bit write when needing to compute ECC for 16-bits."]
pub type DataLowW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DATA_HIGH` reader - 31:16\\]
Upper 16 bits of ECC DataWriting to this bitfield will cause the ECC logic to compute the ECC\\[SEC-DED\\]
the entire 32-bit register and update TX_ECC_VAL register with the results. Software should write to these 16 bits of the register in a 32-bit write when needing to compute ECC for 32-bits for the full TX_ECC_DATA register."]
pub type DataHighR = crate::FieldReader<u16>;
#[doc = "Field `DATA_HIGH` writer - 31:16\\]
Upper 16 bits of ECC DataWriting to this bitfield will cause the ECC logic to compute the ECC\\[SEC-DED\\]
the entire 32-bit register and update TX_ECC_VAL register with the results. Software should write to these 16 bits of the register in a 32-bit write when needing to compute ECC for 32-bits for the full TX_ECC_DATA register."]
pub type DataHighW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Lower 16 bits of ECC DataWriting to this bitfield will cause the ECC logic to compute the ECC\\[SEC-DED\\]
for these 16 bits and update the TX_ECC_VAL register with the results. Software should write to these register bits as a 16-bit write when needing to compute ECC for 16-bits."]
    #[inline(always)]
    pub fn data_low(&self) -> DataLowR {
        DataLowR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Upper 16 bits of ECC DataWriting to this bitfield will cause the ECC logic to compute the ECC\\[SEC-DED\\]
the entire 32-bit register and update TX_ECC_VAL register with the results. Software should write to these 16 bits of the register in a 32-bit write when needing to compute ECC for 32-bits for the full TX_ECC_DATA register."]
    #[inline(always)]
    pub fn data_high(&self) -> DataHighR {
        DataHighR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Lower 16 bits of ECC DataWriting to this bitfield will cause the ECC logic to compute the ECC\\[SEC-DED\\]
for these 16 bits and update the TX_ECC_VAL register with the results. Software should write to these register bits as a 16-bit write when needing to compute ECC for 16-bits."]
    #[inline(always)]
    #[must_use]
    pub fn data_low(&mut self) -> DataLowW<CfgTxEccDataSpec> {
        DataLowW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Upper 16 bits of ECC DataWriting to this bitfield will cause the ECC logic to compute the ECC\\[SEC-DED\\]
the entire 32-bit register and update TX_ECC_VAL register with the results. Software should write to these 16 bits of the register in a 32-bit write when needing to compute ECC for 32-bits for the full TX_ECC_DATA register."]
    #[inline(always)]
    #[must_use]
    pub fn data_high(&mut self) -> DataHighW<CfgTxEccDataSpec> {
        DataHighW::new(self, 16)
    }
}
#[doc = "Transmit ECC data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_ecc_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_ecc_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxEccDataSpec;
impl crate::RegisterSpec for CfgTxEccDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tx_ecc_data::R`](R) reader structure"]
impl crate::Readable for CfgTxEccDataSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_ecc_data::W`](W) writer structure"]
impl crate::Writable for CfgTxEccDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TX_ECC_DATA to value 0"]
impl crate::Resettable for CfgTxEccDataSpec {
    const RESET_VALUE: u32 = 0;
}
