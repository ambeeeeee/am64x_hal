#[doc = "Register `CPSW_NUSS_VBUSP_THRU_RATE_REG` reader"]
pub type R = crate::R<CpswNussVbuspThruRateRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_THRU_RATE_REG` writer"]
pub type W = crate::W<CpswNussVbuspThruRateRegSpec>;
#[doc = "Field `P0_RX_THRU_RATE` reader - 3:0\\]
CPPI FIFO receive through rate"]
pub type P0RxThruRateR = crate::FieldReader;
#[doc = "Field `P0_RX_THRU_RATE` writer - 3:0\\]
CPPI FIFO receive through rate"]
pub type P0RxThruRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SL_RX_THRU_RATE` reader - 15:12\\]
Switch FIFO receive through rate"]
pub type SlRxThruRateR = crate::FieldReader;
#[doc = "Field `SL_RX_THRU_RATE` writer - 15:12\\]
Switch FIFO receive through rate"]
pub type SlRxThruRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
CPPI FIFO receive through rate"]
    #[inline(always)]
    pub fn p0_rx_thru_rate(&self) -> P0RxThruRateR {
        P0RxThruRateR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Switch FIFO receive through rate"]
    #[inline(always)]
    pub fn sl_rx_thru_rate(&self) -> SlRxThruRateR {
        SlRxThruRateR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
CPPI FIFO receive through rate"]
    #[inline(always)]
    #[must_use]
    pub fn p0_rx_thru_rate(&mut self) -> P0RxThruRateW<CpswNussVbuspThruRateRegSpec> {
        P0RxThruRateW::new(self, 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Switch FIFO receive through rate"]
    #[inline(always)]
    #[must_use]
    pub fn sl_rx_thru_rate(&mut self) -> SlRxThruRateW<CpswNussVbuspThruRateRegSpec> {
        SlRxThruRateW::new(self, 12)
    }
}
#[doc = "CPSW Thru Rate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_thru_rate_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_thru_rate_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspThruRateRegSpec;
impl crate::RegisterSpec for CpswNussVbuspThruRateRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_thru_rate_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspThruRateRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_thru_rate_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspThruRateRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_THRU_RATE_REG to value 0x3001"]
impl crate::Resettable for CpswNussVbuspThruRateRegSpec {
    const RESET_VALUE: u32 = 0x3001;
}
