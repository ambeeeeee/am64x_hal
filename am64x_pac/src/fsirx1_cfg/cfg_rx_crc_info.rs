#[doc = "Register `CFG_RX_CRC_INFO` reader"]
pub type R = crate::R<CfgRxCrcInfoSpec>;
#[doc = "Register `CFG_RX_CRC_INFO` writer"]
pub type W = crate::W<CfgRxCrcInfoSpec>;
#[doc = "Field `RX_CRC` reader - 7:0\\]
Received CRC ValueThis bitfield contains the CRC value that was last received a frame. The contents of this bitfield are valid only when data frames are received. Note: The contents of this bitfield are invalid for ping and error frames."]
pub type RxCrcR = crate::FieldReader;
#[doc = "Field `RX_CRC` writer - 7:0\\]
Received CRC ValueThis bitfield contains the CRC value that was last received a frame. The contents of this bitfield are valid only when data frames are received. Note: The contents of this bitfield are invalid for ping and error frames."]
pub type RxCrcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CALC_CRC` reader - 15:8\\]
Harware Calculated CRC ValueThis bitfield contains the CRC value that was calculated on the last received data. The contents of this bitfield are valid only when data frames are received. Note: The contents of this bitfield are invalid for ping and error frames."]
pub type CalcCrcR = crate::FieldReader;
#[doc = "Field `CALC_CRC` writer - 15:8\\]
Harware Calculated CRC ValueThis bitfield contains the CRC value that was calculated on the last received data. The contents of this bitfield are valid only when data frames are received. Note: The contents of this bitfield are invalid for ping and error frames."]
pub type CalcCrcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Received CRC ValueThis bitfield contains the CRC value that was last received a frame. The contents of this bitfield are valid only when data frames are received. Note: The contents of this bitfield are invalid for ping and error frames."]
    #[inline(always)]
    pub fn rx_crc(&self) -> RxCrcR {
        RxCrcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Harware Calculated CRC ValueThis bitfield contains the CRC value that was calculated on the last received data. The contents of this bitfield are valid only when data frames are received. Note: The contents of this bitfield are invalid for ping and error frames."]
    #[inline(always)]
    pub fn calc_crc(&self) -> CalcCrcR {
        CalcCrcR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Received CRC ValueThis bitfield contains the CRC value that was last received a frame. The contents of this bitfield are valid only when data frames are received. Note: The contents of this bitfield are invalid for ping and error frames."]
    #[inline(always)]
    #[must_use]
    pub fn rx_crc(&mut self) -> RxCrcW<CfgRxCrcInfoSpec> {
        RxCrcW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Harware Calculated CRC ValueThis bitfield contains the CRC value that was calculated on the last received data. The contents of this bitfield are valid only when data frames are received. Note: The contents of this bitfield are invalid for ping and error frames."]
    #[inline(always)]
    #[must_use]
    pub fn calc_crc(&mut self) -> CalcCrcW<CfgRxCrcInfoSpec> {
        CalcCrcW::new(self, 8)
    }
}
#[doc = "Receive CRC info of received and computed CRC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_crc_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_crc_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxCrcInfoSpec;
impl crate::RegisterSpec for CfgRxCrcInfoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_crc_info::R`](R) reader structure"]
impl crate::Readable for CfgRxCrcInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_crc_info::W`](W) writer structure"]
impl crate::Writable for CfgRxCrcInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_CRC_INFO to value 0"]
impl crate::Resettable for CfgRxCrcInfoSpec {
    const RESET_VALUE: u16 = 0;
}
