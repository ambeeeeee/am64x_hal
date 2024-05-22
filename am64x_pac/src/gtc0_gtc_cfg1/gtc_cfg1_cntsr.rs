#[doc = "Register `GTC_CFG1_CNTSR` reader"]
pub type R = crate::R<GtcCfg1CntsrSpec>;
#[doc = "Register `GTC_CFG1_CNTSR` writer"]
pub type W = crate::W<GtcCfg1CntsrSpec>;
#[doc = "Field `CNTSR_DBGH` reader - 1:1\\]
Debug Halt"]
pub type CntsrDbghR = crate::BitReader;
#[doc = "Field `CNTSR_DBGH` writer - 1:1\\]
Debug Halt"]
pub type CntsrDbghW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTSR_FCACK` reader - 31:8\\]
Frequency Change Ackowledge"]
pub type CntsrFcackR = crate::FieldReader<u32>;
#[doc = "Field `CNTSR_FCACK` writer - 31:8\\]
Frequency Change Ackowledge"]
pub type CntsrFcackW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 1 - 1:1\\]
Debug Halt"]
    #[inline(always)]
    pub fn cntsr_dbgh(&self) -> CntsrDbghR {
        CntsrDbghR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Frequency Change Ackowledge"]
    #[inline(always)]
    pub fn cntsr_fcack(&self) -> CntsrFcackR {
        CntsrFcackR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
Debug Halt"]
    #[inline(always)]
    #[must_use]
    pub fn cntsr_dbgh(&mut self) -> CntsrDbghW<GtcCfg1CntsrSpec> {
        CntsrDbghW::new(self, 1)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Frequency Change Ackowledge"]
    #[inline(always)]
    #[must_use]
    pub fn cntsr_fcack(&mut self) -> CntsrFcackW<GtcCfg1CntsrSpec> {
        CntsrFcackW::new(self, 8)
    }
}
#[doc = "GTC_CFG1_CNTSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg1_cntsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg1_cntsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtcCfg1CntsrSpec;
impl crate::RegisterSpec for GtcCfg1CntsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtc_cfg1_cntsr::R`](R) reader structure"]
impl crate::Readable for GtcCfg1CntsrSpec {}
#[doc = "`write(|w| ..)` method takes [`gtc_cfg1_cntsr::W`](W) writer structure"]
impl crate::Writable for GtcCfg1CntsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTC_CFG1_CNTSR to value 0"]
impl crate::Resettable for GtcCfg1CntsrSpec {
    const RESET_VALUE: u32 = 0;
}
