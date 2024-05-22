#[doc = "Register `APBADDR_ETM_CPU1_TRCACATR4` reader"]
pub type R = crate::R<ApbaddrEtmCpu1Trcacatr4Spec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCACATR4` writer"]
pub type W = crate::W<ApbaddrEtmCpu1Trcacatr4Spec>;
#[doc = "Field `TYPE` reader - 1:0\\]
Controls what type of comparison the trace unit performs: 00 Instruction address. 01 Data load address. 10 Data store address. 11 Data load address or data store address. If TRCIDR4.SUPPDAC does not indicate that data address comparisons are implemented, then this field is RES0. This means that any comparison performed by this address comparator is an instruction address comparison."]
pub type TypeR = crate::FieldReader;
#[doc = "Field `TYPE` writer - 1:0\\]
Controls what type of comparison the trace unit performs: 00 Instruction address. 01 Data load address. 10 Data store address. 11 Data load address or data store address. If TRCIDR4.SUPPDAC does not indicate that data address comparisons are implemented, then this field is RES0. This means that any comparison performed by this address comparator is an instruction address comparison."]
pub type TypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CONTEXTTYPE` reader - 3:2\\]
If TRCIDR4.NUMVMIDC>0 and TRCIDR4.NUMCIDC>0, this field controls whether the trace unit performs a Context ID comparison, a virtual machine identifier \\[VMID\\]
comparison, or both comparisons: 00 The trace unit does not perform a Context ID or VMID comparison. 01 The trace unit performs a Context ID comparison using the Context ID comparator that the CONTEXT field specifies, and signals a match if both the Context ID comparator matches and the address comparator match. 10 The trace unit performs a VMID comparison using the VMID comparator that the CONTEXT field specifies, and signals a match if both the VMID comparator and the address comparator match. 11 The trace unit performs a Context ID comparison and a VMID comparison using the comparators that the CONTEXT field specifies, and signals a match if the Context ID comparator matches, the VMID comparator matches, and the address comparator matches. If TRCIDR4.NUMVMIDC==0 and TRCIDR4.NUMCIDC>0, bit \\[3\\]
is RES0 and bit\\[2\\]
controls whether the trace unit performs a Context ID comparison, as with cases 0b00 and 0b01 above.If TRCIDR4.NUMVMIDC==0 and TRCIDR4.NUMCIDC==0, both bits are RES0."]
pub type ContexttypeR = crate::FieldReader;
#[doc = "Field `CONTEXTTYPE` writer - 3:2\\]
If TRCIDR4.NUMVMIDC>0 and TRCIDR4.NUMCIDC>0, this field controls whether the trace unit performs a Context ID comparison, a virtual machine identifier \\[VMID\\]
comparison, or both comparisons: 00 The trace unit does not perform a Context ID or VMID comparison. 01 The trace unit performs a Context ID comparison using the Context ID comparator that the CONTEXT field specifies, and signals a match if both the Context ID comparator matches and the address comparator match. 10 The trace unit performs a VMID comparison using the VMID comparator that the CONTEXT field specifies, and signals a match if both the VMID comparator and the address comparator match. 11 The trace unit performs a Context ID comparison and a VMID comparison using the comparators that the CONTEXT field specifies, and signals a match if the Context ID comparator matches, the VMID comparator matches, and the address comparator matches. If TRCIDR4.NUMVMIDC==0 and TRCIDR4.NUMCIDC>0, bit \\[3\\]
is RES0 and bit\\[2\\]
controls whether the trace unit performs a Context ID comparison, as with cases 0b00 and 0b01 above.If TRCIDR4.NUMVMIDC==0 and TRCIDR4.NUMCIDC==0, both bits are RES0."]
pub type ContexttypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CONTEXT` reader - 6:4\\]
If TRCIDR4.NUMCIDFC > 0 or TRCIDR4.NUMVMIDC > 0, selects a Context ID comparator or VMID comparator: 000 Comparator 0. 001 Comparator 1. 010 Comparator 2. and so on up to 0b111, which indicates comparator 7.The implemented width of this field is determined by the size of whichever of TRCIDR4.NUMVMIDC and TRCIDR4.NUMCIDC is larger. If the largest field is one bit long, then this field implements one bit, bit\\[4\\]. If the largest field is four bits long, then this field implements two bits, bits\\[5:4\\]. Unimplemented bits within the field are RAZ/WI.If TRCIDR4.NUMCIDFC==0 and TRCIDR4.NUMVMIDC==0, this field is RES0."]
pub type ContextR = crate::FieldReader;
#[doc = "Field `CONTEXT` writer - 6:4\\]
If TRCIDR4.NUMCIDFC > 0 or TRCIDR4.NUMVMIDC > 0, selects a Context ID comparator or VMID comparator: 000 Comparator 0. 001 Comparator 1. 010 Comparator 2. and so on up to 0b111, which indicates comparator 7.The implemented width of this field is determined by the size of whichever of TRCIDR4.NUMVMIDC and TRCIDR4.NUMCIDC is larger. If the largest field is one bit long, then this field implements one bit, bit\\[4\\]. If the largest field is four bits long, then this field implements two bits, bits\\[5:4\\]. Unimplemented bits within the field are RAZ/WI.If TRCIDR4.NUMCIDFC==0 and TRCIDR4.NUMVMIDC==0, this field is RES0."]
pub type ContextW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES0_TRCACATR4_7_7` reader - 7:7\\]
Reserved, RES0."]
pub type Res0Trcacatr4_7_7R = crate::BitReader;
#[doc = "Field `RES0_TRCACATR4_7_7` writer - 7:7\\]
Reserved, RES0."]
pub type Res0Trcacatr4_7_7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXLEVEL_S` reader - 11:8\\]
In Secure state, each bit controls whether a comparison can occur for the corresponding exception level: 0 The trace unit can perform a comparison, in Secure state, for exception level n. 1 The trace unit does not perform a comparison, in Secure state, for exception level n. The exception levels are:Bit\\[8\\]Exception level 0.Bit\\[9\\]Exception level 1.Bit\\[10\\]RAZ/WI. EXLEVEL_S\\[2\\]
is never implemented.Bit\\[11\\]Exception level 3.The content of the field is IMPLEMENTATION DEFINED and is set by the value of TRCIDR3.EXLEVEL_S. Unimplemented bits are RAZ/WI."]
pub type ExlevelSR = crate::FieldReader;
#[doc = "Field `EXLEVEL_S` writer - 11:8\\]
In Secure state, each bit controls whether a comparison can occur for the corresponding exception level: 0 The trace unit can perform a comparison, in Secure state, for exception level n. 1 The trace unit does not perform a comparison, in Secure state, for exception level n. The exception levels are:Bit\\[8\\]Exception level 0.Bit\\[9\\]Exception level 1.Bit\\[10\\]RAZ/WI. EXLEVEL_S\\[2\\]
is never implemented.Bit\\[11\\]Exception level 3.The content of the field is IMPLEMENTATION DEFINED and is set by the value of TRCIDR3.EXLEVEL_S. Unimplemented bits are RAZ/WI."]
pub type ExlevelSW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXLEVEL_NS` reader - 15:12\\]
In Non-secure state, each bit controls whether a comparison can occur for the corresponding exception level: 0 The trace unit can perform a comparison, in Non-secure state, for exception level n. 1 The trace unit does not perform a comparison, in Non-secure state, for exception level n. The exception levels are:Bit\\[12\\]Exception level 0.Bit\\[13\\]Exception level 1.Bit\\[14\\]Exception level 2.Bit\\[15\\]RAZ/WI. EXLEVEL_NS\\[3\\]
is never implemented.The content of the field is IMPLEMENTATION DEFINED and is set by the value of TRCIDR3.EXLEVEL_NS. Unimplemented bits are RAZ/WI."]
pub type ExlevelNsR = crate::FieldReader;
#[doc = "Field `EXLEVEL_NS` writer - 15:12\\]
In Non-secure state, each bit controls whether a comparison can occur for the corresponding exception level: 0 The trace unit can perform a comparison, in Non-secure state, for exception level n. 1 The trace unit does not perform a comparison, in Non-secure state, for exception level n. The exception levels are:Bit\\[12\\]Exception level 0.Bit\\[13\\]Exception level 1.Bit\\[14\\]Exception level 2.Bit\\[15\\]RAZ/WI. EXLEVEL_NS\\[3\\]
is never implemented.The content of the field is IMPLEMENTATION DEFINED and is set by the value of TRCIDR3.EXLEVEL_NS. Unimplemented bits are RAZ/WI."]
pub type ExlevelNsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATAMATCH` reader - 17:16\\]
Controls how the trace unit performs a data value comparison: 00 The trace unit does not perform a data value comparison. 01 The trace unit performs a data value comparison and signals a match if both values are identical. 10 Reserved. 11 The trace unit performs a data value comparison and signals a match if both values are different. Supported only if the corresponding data value comparator is supported, otherwise this field is RES0."]
pub type DatamatchR = crate::FieldReader;
#[doc = "Field `DATAMATCH` writer - 17:16\\]
Controls how the trace unit performs a data value comparison: 00 The trace unit does not perform a data value comparison. 01 The trace unit performs a data value comparison and signals a match if both values are identical. 10 Reserved. 11 The trace unit performs a data value comparison and signals a match if both values are different. Supported only if the corresponding data value comparator is supported, otherwise this field is RES0."]
pub type DatamatchW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATASIZE` reader - 19:18\\]
Controls the width of the data value comparison: 00 Byte. 01 Halfword. 10 Word. 11 Doubleword. Supported only if the corresponding data value comparator is supported, otherwise this field is RES0.The doubleword width is supported only if TRCIDR2.DVSIZE indicates that 64-bit values are supported. If 64-bit values are not supported, 0b11 is reserved."]
pub type DatasizeR = crate::FieldReader;
#[doc = "Field `DATASIZE` writer - 19:18\\]
Controls the width of the data value comparison: 00 Byte. 01 Halfword. 10 Word. 11 Doubleword. Supported only if the corresponding data value comparator is supported, otherwise this field is RES0.The doubleword width is supported only if TRCIDR2.DVSIZE indicates that 64-bit values are supported. If 64-bit values are not supported, 0b11 is reserved."]
pub type DatasizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATARANGE` reader - 20:20\\]
Controls whether a data value comparison uses the single address comparator or the address range comparator: 0 The trace unit uses the single address comparator for data value comparisons. The behavior of the address range comparator is UNPREDICTABLE. 1 The trace unit uses the address range comparator for data value comparisons. The behavior of the single address comparators in this pair is UNPREDICTABLE. The trace unit ignores this field when DATAMATCH==0b00.Supported only if the corresponding data value comparator is supported, otherwise this bit is RES0."]
pub type DatarangeR = crate::BitReader;
#[doc = "Field `DATARANGE` writer - 20:20\\]
Controls whether a data value comparison uses the single address comparator or the address range comparator: 0 The trace unit uses the single address comparator for data value comparisons. The behavior of the address range comparator is UNPREDICTABLE. 1 The trace unit uses the address range comparator for data value comparisons. The behavior of the single address comparators in this pair is UNPREDICTABLE. The trace unit ignores this field when DATAMATCH==0b00.Supported only if the corresponding data value comparator is supported, otherwise this bit is RES0."]
pub type DatarangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTBM` reader - 21:21\\]
Controls whether data address comparisons use the data address \\[63:56\\]
bits: 0 The trace unit ignores the data address \\[63:56\\]
bits for data address comparisons. 1 The trace unit uses the data address \\[63:56\\]
bits for data address comparisons. Supported only if TRCIDR2.DASIZE indicates that the data address size is 64 bits, otherwise this bit is RES0."]
pub type DtbmR = crate::BitReader;
#[doc = "Field `DTBM` writer - 21:21\\]
Controls whether data address comparisons use the data address \\[63:56\\]
bits: 0 The trace unit ignores the data address \\[63:56\\]
bits for data address comparisons. 1 The trace unit uses the data address \\[63:56\\]
bits for data address comparisons. Supported only if TRCIDR2.DASIZE indicates that the data address size is 64 bits, otherwise this bit is RES0."]
pub type DtbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCACATR4_31_22` reader - 31:22\\]
Reserved, RES0."]
pub type Res0Trcacatr4_31_22R = crate::FieldReader<u16>;
#[doc = "Field `RES0_TRCACATR4_31_22` writer - 31:22\\]
Reserved, RES0."]
pub type Res0Trcacatr4_31_22W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Controls what type of comparison the trace unit performs: 00 Instruction address. 01 Data load address. 10 Data store address. 11 Data load address or data store address. If TRCIDR4.SUPPDAC does not indicate that data address comparisons are implemented, then this field is RES0. This means that any comparison performed by this address comparator is an instruction address comparison."]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
If TRCIDR4.NUMVMIDC>0 and TRCIDR4.NUMCIDC>0, this field controls whether the trace unit performs a Context ID comparison, a virtual machine identifier \\[VMID\\]
comparison, or both comparisons: 00 The trace unit does not perform a Context ID or VMID comparison. 01 The trace unit performs a Context ID comparison using the Context ID comparator that the CONTEXT field specifies, and signals a match if both the Context ID comparator matches and the address comparator match. 10 The trace unit performs a VMID comparison using the VMID comparator that the CONTEXT field specifies, and signals a match if both the VMID comparator and the address comparator match. 11 The trace unit performs a Context ID comparison and a VMID comparison using the comparators that the CONTEXT field specifies, and signals a match if the Context ID comparator matches, the VMID comparator matches, and the address comparator matches. If TRCIDR4.NUMVMIDC==0 and TRCIDR4.NUMCIDC>0, bit \\[3\\]
is RES0 and bit\\[2\\]
controls whether the trace unit performs a Context ID comparison, as with cases 0b00 and 0b01 above.If TRCIDR4.NUMVMIDC==0 and TRCIDR4.NUMCIDC==0, both bits are RES0."]
    #[inline(always)]
    pub fn contexttype(&self) -> ContexttypeR {
        ContexttypeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
If TRCIDR4.NUMCIDFC > 0 or TRCIDR4.NUMVMIDC > 0, selects a Context ID comparator or VMID comparator: 000 Comparator 0. 001 Comparator 1. 010 Comparator 2. and so on up to 0b111, which indicates comparator 7.The implemented width of this field is determined by the size of whichever of TRCIDR4.NUMVMIDC and TRCIDR4.NUMCIDC is larger. If the largest field is one bit long, then this field implements one bit, bit\\[4\\]. If the largest field is four bits long, then this field implements two bits, bits\\[5:4\\]. Unimplemented bits within the field are RAZ/WI.If TRCIDR4.NUMCIDFC==0 and TRCIDR4.NUMVMIDC==0, this field is RES0."]
    #[inline(always)]
    pub fn context(&self) -> ContextR {
        ContextR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcacatr4_7_7(&self) -> Res0Trcacatr4_7_7R {
        Res0Trcacatr4_7_7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
In Secure state, each bit controls whether a comparison can occur for the corresponding exception level: 0 The trace unit can perform a comparison, in Secure state, for exception level n. 1 The trace unit does not perform a comparison, in Secure state, for exception level n. The exception levels are:Bit\\[8\\]Exception level 0.Bit\\[9\\]Exception level 1.Bit\\[10\\]RAZ/WI. EXLEVEL_S\\[2\\]
is never implemented.Bit\\[11\\]Exception level 3.The content of the field is IMPLEMENTATION DEFINED and is set by the value of TRCIDR3.EXLEVEL_S. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    pub fn exlevel_s(&self) -> ExlevelSR {
        ExlevelSR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
In Non-secure state, each bit controls whether a comparison can occur for the corresponding exception level: 0 The trace unit can perform a comparison, in Non-secure state, for exception level n. 1 The trace unit does not perform a comparison, in Non-secure state, for exception level n. The exception levels are:Bit\\[12\\]Exception level 0.Bit\\[13\\]Exception level 1.Bit\\[14\\]Exception level 2.Bit\\[15\\]RAZ/WI. EXLEVEL_NS\\[3\\]
is never implemented.The content of the field is IMPLEMENTATION DEFINED and is set by the value of TRCIDR3.EXLEVEL_NS. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    pub fn exlevel_ns(&self) -> ExlevelNsR {
        ExlevelNsR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Controls how the trace unit performs a data value comparison: 00 The trace unit does not perform a data value comparison. 01 The trace unit performs a data value comparison and signals a match if both values are identical. 10 Reserved. 11 The trace unit performs a data value comparison and signals a match if both values are different. Supported only if the corresponding data value comparator is supported, otherwise this field is RES0."]
    #[inline(always)]
    pub fn datamatch(&self) -> DatamatchR {
        DatamatchR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Controls the width of the data value comparison: 00 Byte. 01 Halfword. 10 Word. 11 Doubleword. Supported only if the corresponding data value comparator is supported, otherwise this field is RES0.The doubleword width is supported only if TRCIDR2.DVSIZE indicates that 64-bit values are supported. If 64-bit values are not supported, 0b11 is reserved."]
    #[inline(always)]
    pub fn datasize(&self) -> DatasizeR {
        DatasizeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Controls whether a data value comparison uses the single address comparator or the address range comparator: 0 The trace unit uses the single address comparator for data value comparisons. The behavior of the address range comparator is UNPREDICTABLE. 1 The trace unit uses the address range comparator for data value comparisons. The behavior of the single address comparators in this pair is UNPREDICTABLE. The trace unit ignores this field when DATAMATCH==0b00.Supported only if the corresponding data value comparator is supported, otherwise this bit is RES0."]
    #[inline(always)]
    pub fn datarange(&self) -> DatarangeR {
        DatarangeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Controls whether data address comparisons use the data address \\[63:56\\]
bits: 0 The trace unit ignores the data address \\[63:56\\]
bits for data address comparisons. 1 The trace unit uses the data address \\[63:56\\]
bits for data address comparisons. Supported only if TRCIDR2.DASIZE indicates that the data address size is 64 bits, otherwise this bit is RES0."]
    #[inline(always)]
    pub fn dtbm(&self) -> DtbmR {
        DtbmR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcacatr4_31_22(&self) -> Res0Trcacatr4_31_22R {
        Res0Trcacatr4_31_22R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Controls what type of comparison the trace unit performs: 00 Instruction address. 01 Data load address. 10 Data store address. 11 Data load address or data store address. If TRCIDR4.SUPPDAC does not indicate that data address comparisons are implemented, then this field is RES0. This means that any comparison performed by this address comparator is an instruction address comparison."]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TypeW<ApbaddrEtmCpu1Trcacatr4Spec> {
        TypeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
If TRCIDR4.NUMVMIDC>0 and TRCIDR4.NUMCIDC>0, this field controls whether the trace unit performs a Context ID comparison, a virtual machine identifier \\[VMID\\]
comparison, or both comparisons: 00 The trace unit does not perform a Context ID or VMID comparison. 01 The trace unit performs a Context ID comparison using the Context ID comparator that the CONTEXT field specifies, and signals a match if both the Context ID comparator matches and the address comparator match. 10 The trace unit performs a VMID comparison using the VMID comparator that the CONTEXT field specifies, and signals a match if both the VMID comparator and the address comparator match. 11 The trace unit performs a Context ID comparison and a VMID comparison using the comparators that the CONTEXT field specifies, and signals a match if the Context ID comparator matches, the VMID comparator matches, and the address comparator matches. If TRCIDR4.NUMVMIDC==0 and TRCIDR4.NUMCIDC>0, bit \\[3\\]
is RES0 and bit\\[2\\]
controls whether the trace unit performs a Context ID comparison, as with cases 0b00 and 0b01 above.If TRCIDR4.NUMVMIDC==0 and TRCIDR4.NUMCIDC==0, both bits are RES0."]
    #[inline(always)]
    #[must_use]
    pub fn contexttype(&mut self) -> ContexttypeW<ApbaddrEtmCpu1Trcacatr4Spec> {
        ContexttypeW::new(self, 2)
    }
    #[doc = "Bits 4:6 - 6:4\\]
If TRCIDR4.NUMCIDFC > 0 or TRCIDR4.NUMVMIDC > 0, selects a Context ID comparator or VMID comparator: 000 Comparator 0. 001 Comparator 1. 010 Comparator 2. and so on up to 0b111, which indicates comparator 7.The implemented width of this field is determined by the size of whichever of TRCIDR4.NUMVMIDC and TRCIDR4.NUMCIDC is larger. If the largest field is one bit long, then this field implements one bit, bit\\[4\\]. If the largest field is four bits long, then this field implements two bits, bits\\[5:4\\]. Unimplemented bits within the field are RAZ/WI.If TRCIDR4.NUMCIDFC==0 and TRCIDR4.NUMVMIDC==0, this field is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn context(&mut self) -> ContextW<ApbaddrEtmCpu1Trcacatr4Spec> {
        ContextW::new(self, 4)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcacatr4_7_7(&mut self) -> Res0Trcacatr4_7_7W<ApbaddrEtmCpu1Trcacatr4Spec> {
        Res0Trcacatr4_7_7W::new(self, 7)
    }
    #[doc = "Bits 8:11 - 11:8\\]
In Secure state, each bit controls whether a comparison can occur for the corresponding exception level: 0 The trace unit can perform a comparison, in Secure state, for exception level n. 1 The trace unit does not perform a comparison, in Secure state, for exception level n. The exception levels are:Bit\\[8\\]Exception level 0.Bit\\[9\\]Exception level 1.Bit\\[10\\]RAZ/WI. EXLEVEL_S\\[2\\]
is never implemented.Bit\\[11\\]Exception level 3.The content of the field is IMPLEMENTATION DEFINED and is set by the value of TRCIDR3.EXLEVEL_S. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    #[must_use]
    pub fn exlevel_s(&mut self) -> ExlevelSW<ApbaddrEtmCpu1Trcacatr4Spec> {
        ExlevelSW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
In Non-secure state, each bit controls whether a comparison can occur for the corresponding exception level: 0 The trace unit can perform a comparison, in Non-secure state, for exception level n. 1 The trace unit does not perform a comparison, in Non-secure state, for exception level n. The exception levels are:Bit\\[12\\]Exception level 0.Bit\\[13\\]Exception level 1.Bit\\[14\\]Exception level 2.Bit\\[15\\]RAZ/WI. EXLEVEL_NS\\[3\\]
is never implemented.The content of the field is IMPLEMENTATION DEFINED and is set by the value of TRCIDR3.EXLEVEL_NS. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    #[must_use]
    pub fn exlevel_ns(&mut self) -> ExlevelNsW<ApbaddrEtmCpu1Trcacatr4Spec> {
        ExlevelNsW::new(self, 12)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Controls how the trace unit performs a data value comparison: 00 The trace unit does not perform a data value comparison. 01 The trace unit performs a data value comparison and signals a match if both values are identical. 10 Reserved. 11 The trace unit performs a data value comparison and signals a match if both values are different. Supported only if the corresponding data value comparator is supported, otherwise this field is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn datamatch(&mut self) -> DatamatchW<ApbaddrEtmCpu1Trcacatr4Spec> {
        DatamatchW::new(self, 16)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Controls the width of the data value comparison: 00 Byte. 01 Halfword. 10 Word. 11 Doubleword. Supported only if the corresponding data value comparator is supported, otherwise this field is RES0.The doubleword width is supported only if TRCIDR2.DVSIZE indicates that 64-bit values are supported. If 64-bit values are not supported, 0b11 is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn datasize(&mut self) -> DatasizeW<ApbaddrEtmCpu1Trcacatr4Spec> {
        DatasizeW::new(self, 18)
    }
    #[doc = "Bit 20 - 20:20\\]
Controls whether a data value comparison uses the single address comparator or the address range comparator: 0 The trace unit uses the single address comparator for data value comparisons. The behavior of the address range comparator is UNPREDICTABLE. 1 The trace unit uses the address range comparator for data value comparisons. The behavior of the single address comparators in this pair is UNPREDICTABLE. The trace unit ignores this field when DATAMATCH==0b00.Supported only if the corresponding data value comparator is supported, otherwise this bit is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn datarange(&mut self) -> DatarangeW<ApbaddrEtmCpu1Trcacatr4Spec> {
        DatarangeW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Controls whether data address comparisons use the data address \\[63:56\\]
bits: 0 The trace unit ignores the data address \\[63:56\\]
bits for data address comparisons. 1 The trace unit uses the data address \\[63:56\\]
bits for data address comparisons. Supported only if TRCIDR2.DASIZE indicates that the data address size is 64 bits, otherwise this bit is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn dtbm(&mut self) -> DtbmW<ApbaddrEtmCpu1Trcacatr4Spec> {
        DtbmW::new(self, 21)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcacatr4_31_22(&mut self) -> Res0Trcacatr4_31_22W<ApbaddrEtmCpu1Trcacatr4Spec> {
        Res0Trcacatr4_31_22W::new(self, 22)
    }
}
#[doc = "Address Comparator Access Type Registers 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacatr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacatr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1Trcacatr4Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu1Trcacatr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcacatr4::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1Trcacatr4Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcacatr4::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1Trcacatr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCACATR4 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1Trcacatr4Spec {
    const RESET_VALUE: u32 = 0;
}
