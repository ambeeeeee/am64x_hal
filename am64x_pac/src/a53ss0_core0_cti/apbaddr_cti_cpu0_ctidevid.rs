#[doc = "Register `APBADDR_CTI_CPU0_CTIDEVID` reader"]
pub type R = crate::R<ApbaddrCtiCpu0CtidevidSpec>;
#[doc = "Register `APBADDR_CTI_CPU0_CTIDEVID` writer"]
pub type W = crate::W<ApbaddrCtiCpu0CtidevidSpec>;
#[doc = "Field `EXTMUXNUM` reader - 4:0\\]
Maximum number of external triggers available for multiplexing into the CTI. This relates only to additional external triggers outside those defined for v8-A."]
pub type ExtmuxnumR = crate::FieldReader;
#[doc = "Field `EXTMUXNUM` writer - 4:0\\]
Maximum number of external triggers available for multiplexing into the CTI. This relates only to additional external triggers outside those defined for v8-A."]
pub type ExtmuxnumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RES0_CTIDEVID_7_5` reader - 7:5\\]
Reserved, RES0."]
pub type Res0Ctidevid7_5R = crate::FieldReader;
#[doc = "Field `RES0_CTIDEVID_7_5` writer - 7:5\\]
Reserved, RES0."]
pub type Res0Ctidevid7_5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NUMTRIG` reader - 13:8\\]
Number of triggers implemented. IMPLEMENTATION DEFINED. This is one more than the index of the largest trigger, rather than the actual number of triggers.For v8-A, valid values are: 000011 Up to 3 triggers \\[0..2\\]
implemented. 001000 Up to 8 triggers \\[0..7\\]
implemented. 001001 Up to 9 triggers \\[0..8\\]
implemented. 001010 Up to 10 triggers \\[0..9\\]
implemented. and so on up to 0b100000, 32 triggers \\[0..31\\]
implemented.All other values are reserved. If the Trace Extension is implemented, this field must be at least 001000. There is no guarantee that any of the implemented triggers, including the highest numbered, are connected to any components."]
pub type NumtrigR = crate::FieldReader;
#[doc = "Field `NUMTRIG` writer - 13:8\\]
Number of triggers implemented. IMPLEMENTATION DEFINED. This is one more than the index of the largest trigger, rather than the actual number of triggers.For v8-A, valid values are: 000011 Up to 3 triggers \\[0..2\\]
implemented. 001000 Up to 8 triggers \\[0..7\\]
implemented. 001001 Up to 9 triggers \\[0..8\\]
implemented. 001010 Up to 10 triggers \\[0..9\\]
implemented. and so on up to 0b100000, 32 triggers \\[0..31\\]
implemented.All other values are reserved. If the Trace Extension is implemented, this field must be at least 001000. There is no guarantee that any of the implemented triggers, including the highest numbered, are connected to any components."]
pub type NumtrigW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RES0_CTIDEVID_15_14` reader - 15:14\\]
Reserved, RES0."]
pub type Res0Ctidevid15_14R = crate::FieldReader;
#[doc = "Field `RES0_CTIDEVID_15_14` writer - 15:14\\]
Reserved, RES0."]
pub type Res0Ctidevid15_14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NUMCHAN` reader - 21:16\\]
Number of ECT channels implemented. IMPLEMENTATION DEFINED. For v8-A, valid values are: 000011 3 channels \\[0..2\\]
implemented. 000100 4 channels \\[0..3\\]
implemented. 000101 5 channels \\[0..4\\]
implemented. 000110 6 channels \\[0..5\\]
implemented. and so on up to 0b100000, 32 channels \\[0..31\\]
implemented.All other values are reserved."]
pub type NumchanR = crate::FieldReader;
#[doc = "Field `NUMCHAN` writer - 21:16\\]
Number of ECT channels implemented. IMPLEMENTATION DEFINED. For v8-A, valid values are: 000011 3 channels \\[0..2\\]
implemented. 000100 4 channels \\[0..3\\]
implemented. 000101 5 channels \\[0..4\\]
implemented. 000110 6 channels \\[0..5\\]
implemented. and so on up to 0b100000, 32 channels \\[0..31\\]
implemented.All other values are reserved."]
pub type NumchanW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RES0_CTIDEVID_23_22` reader - 23:22\\]
Reserved, RES0."]
pub type Res0Ctidevid23_22R = crate::FieldReader;
#[doc = "Field `RES0_CTIDEVID_23_22` writer - 23:22\\]
Reserved, RES0."]
pub type Res0Ctidevid23_22W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INOUT` reader - 25:24\\]
Input/output options. Indicates presence of the input gate. If the CTM is not implemented, this field is RAZ. 00 CTIGATE does not mask propagation of input events from external channels. 01 CTIGATE masks propagation of input events from external channels. All other values are reserved."]
pub type InoutR = crate::FieldReader;
#[doc = "Field `INOUT` writer - 25:24\\]
Input/output options. Indicates presence of the input gate. If the CTM is not implemented, this field is RAZ. 00 CTIGATE does not mask propagation of input events from external channels. 01 CTIGATE masks propagation of input events from external channels. All other values are reserved."]
pub type InoutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES0_CTIDEVID_31_26` reader - 31:26\\]
Reserved, RES0."]
pub type Res0Ctidevid31_26R = crate::FieldReader;
#[doc = "Field `RES0_CTIDEVID_31_26` writer - 31:26\\]
Reserved, RES0."]
pub type Res0Ctidevid31_26W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Maximum number of external triggers available for multiplexing into the CTI. This relates only to additional external triggers outside those defined for v8-A."]
    #[inline(always)]
    pub fn extmuxnum(&self) -> ExtmuxnumR {
        ExtmuxnumR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_ctidevid_7_5(&self) -> Res0Ctidevid7_5R {
        Res0Ctidevid7_5R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Number of triggers implemented. IMPLEMENTATION DEFINED. This is one more than the index of the largest trigger, rather than the actual number of triggers.For v8-A, valid values are: 000011 Up to 3 triggers \\[0..2\\]
implemented. 001000 Up to 8 triggers \\[0..7\\]
implemented. 001001 Up to 9 triggers \\[0..8\\]
implemented. 001010 Up to 10 triggers \\[0..9\\]
implemented. and so on up to 0b100000, 32 triggers \\[0..31\\]
implemented.All other values are reserved. If the Trace Extension is implemented, this field must be at least 001000. There is no guarantee that any of the implemented triggers, including the highest numbered, are connected to any components."]
    #[inline(always)]
    pub fn numtrig(&self) -> NumtrigR {
        NumtrigR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_ctidevid_15_14(&self) -> Res0Ctidevid15_14R {
        Res0Ctidevid15_14R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Number of ECT channels implemented. IMPLEMENTATION DEFINED. For v8-A, valid values are: 000011 3 channels \\[0..2\\]
implemented. 000100 4 channels \\[0..3\\]
implemented. 000101 5 channels \\[0..4\\]
implemented. 000110 6 channels \\[0..5\\]
implemented. and so on up to 0b100000, 32 channels \\[0..31\\]
implemented.All other values are reserved."]
    #[inline(always)]
    pub fn numchan(&self) -> NumchanR {
        NumchanR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_ctidevid_23_22(&self) -> Res0Ctidevid23_22R {
        Res0Ctidevid23_22R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Input/output options. Indicates presence of the input gate. If the CTM is not implemented, this field is RAZ. 00 CTIGATE does not mask propagation of input events from external channels. 01 CTIGATE masks propagation of input events from external channels. All other values are reserved."]
    #[inline(always)]
    pub fn inout(&self) -> InoutR {
        InoutR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_ctidevid_31_26(&self) -> Res0Ctidevid31_26R {
        Res0Ctidevid31_26R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Maximum number of external triggers available for multiplexing into the CTI. This relates only to additional external triggers outside those defined for v8-A."]
    #[inline(always)]
    #[must_use]
    pub fn extmuxnum(&mut self) -> ExtmuxnumW<ApbaddrCtiCpu0CtidevidSpec> {
        ExtmuxnumW::new(self, 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_ctidevid_7_5(&mut self) -> Res0Ctidevid7_5W<ApbaddrCtiCpu0CtidevidSpec> {
        Res0Ctidevid7_5W::new(self, 5)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Number of triggers implemented. IMPLEMENTATION DEFINED. This is one more than the index of the largest trigger, rather than the actual number of triggers.For v8-A, valid values are: 000011 Up to 3 triggers \\[0..2\\]
implemented. 001000 Up to 8 triggers \\[0..7\\]
implemented. 001001 Up to 9 triggers \\[0..8\\]
implemented. 001010 Up to 10 triggers \\[0..9\\]
implemented. and so on up to 0b100000, 32 triggers \\[0..31\\]
implemented.All other values are reserved. If the Trace Extension is implemented, this field must be at least 001000. There is no guarantee that any of the implemented triggers, including the highest numbered, are connected to any components."]
    #[inline(always)]
    #[must_use]
    pub fn numtrig(&mut self) -> NumtrigW<ApbaddrCtiCpu0CtidevidSpec> {
        NumtrigW::new(self, 8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_ctidevid_15_14(&mut self) -> Res0Ctidevid15_14W<ApbaddrCtiCpu0CtidevidSpec> {
        Res0Ctidevid15_14W::new(self, 14)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Number of ECT channels implemented. IMPLEMENTATION DEFINED. For v8-A, valid values are: 000011 3 channels \\[0..2\\]
implemented. 000100 4 channels \\[0..3\\]
implemented. 000101 5 channels \\[0..4\\]
implemented. 000110 6 channels \\[0..5\\]
implemented. and so on up to 0b100000, 32 channels \\[0..31\\]
implemented.All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn numchan(&mut self) -> NumchanW<ApbaddrCtiCpu0CtidevidSpec> {
        NumchanW::new(self, 16)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_ctidevid_23_22(&mut self) -> Res0Ctidevid23_22W<ApbaddrCtiCpu0CtidevidSpec> {
        Res0Ctidevid23_22W::new(self, 22)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Input/output options. Indicates presence of the input gate. If the CTM is not implemented, this field is RAZ. 00 CTIGATE does not mask propagation of input events from external channels. 01 CTIGATE masks propagation of input events from external channels. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn inout(&mut self) -> InoutW<ApbaddrCtiCpu0CtidevidSpec> {
        InoutW::new(self, 24)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_ctidevid_31_26(&mut self) -> Res0Ctidevid31_26W<ApbaddrCtiCpu0CtidevidSpec> {
        Res0Ctidevid31_26W::new(self, 26)
    }
}
#[doc = "CTI Device ID Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctidevid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctidevid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu0CtidevidSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu0CtidevidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu0_ctidevid::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu0CtidevidSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu0_ctidevid::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu0CtidevidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU0_CTIDEVID to value 0x0104_0800"]
impl crate::Resettable for ApbaddrCtiCpu0CtidevidSpec {
    const RESET_VALUE: u32 = 0x0104_0800;
}
