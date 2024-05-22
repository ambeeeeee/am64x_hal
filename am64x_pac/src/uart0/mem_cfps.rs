#[doc = "Register `MEM_CFPS` reader"]
pub type R = crate::R<MemCfpsSpec>;
#[doc = "Register `MEM_CFPS` writer"]
pub type W = crate::W<MemCfpsSpec>;
#[doc = "Field `CFPS` reader - 7:0\\]
System clock frequency prescaler at \\[12x multiple\\]. Examples for CFPS values are given in the table below. Target Freq \\[KHz\\]
CFPS \\[decimal\\]
Actual Freq\\[KHz\\]
30 133 30.08 32.75 122 32.79 36 111 36.04 36.7 109 36.69 38* 105 38.1 40 100 40 56.8 70 57.14 * configured at reset to this value Note: CFPS = 0 is not supported."]
pub type CfpsR = crate::FieldReader;
#[doc = "Field `CFPS` writer - 7:0\\]
System clock frequency prescaler at \\[12x multiple\\]. Examples for CFPS values are given in the table below. Target Freq \\[KHz\\]
CFPS \\[decimal\\]
Actual Freq\\[KHz\\]
30 133 30.08 32.75 122 32.79 36 111 36.04 36.7 109 36.69 38* 105 38.1 40 100 40 56.8 70 57.14 * configured at reset to this value Note: CFPS = 0 is not supported."]
pub type CfpsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
System clock frequency prescaler at \\[12x multiple\\]. Examples for CFPS values are given in the table below. Target Freq \\[KHz\\]
CFPS \\[decimal\\]
Actual Freq\\[KHz\\]
30 133 30.08 32.75 122 32.79 36 111 36.04 36.7 109 36.69 38* 105 38.1 40 100 40 56.8 70 57.14 * configured at reset to this value Note: CFPS = 0 is not supported."]
    #[inline(always)]
    pub fn cfps(&self) -> CfpsR {
        CfpsR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
System clock frequency prescaler at \\[12x multiple\\]. Examples for CFPS values are given in the table below. Target Freq \\[KHz\\]
CFPS \\[decimal\\]
Actual Freq\\[KHz\\]
30 133 30.08 32.75 122 32.79 36 111 36.04 36.7 109 36.69 38* 105 38.1 40 100 40 56.8 70 57.14 * configured at reset to this value Note: CFPS = 0 is not supported."]
    #[inline(always)]
    #[must_use]
    pub fn cfps(&mut self) -> CfpsW<MemCfpsSpec> {
        CfpsW::new(self, 0)
    }
}
#[doc = "Since the Consumer IR works at modulation rates of 30 56.8 KHz, the 48 MHz clock must be pre scaled before the clock can drive the IR logic. This register sets the divisor rate to give a range to accommodate the remote control requirements in BAUD multiples of 12x. The value of the CFPS at reset is 0105 decimal which equates to a 38.1 KHz output from starting conditions. The 48 MHz carrier is prescaled by the CFPS which is then divided by the 12x BAUD multiple.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_cfps::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_cfps::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemCfpsSpec;
impl crate::RegisterSpec for MemCfpsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_cfps::R`](R) reader structure"]
impl crate::Readable for MemCfpsSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_cfps::W`](W) writer structure"]
impl crate::Writable for MemCfpsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CFPS to value 0x0105"]
impl crate::Resettable for MemCfpsSpec {
    const RESET_VALUE: u32 = 0x0105;
}
