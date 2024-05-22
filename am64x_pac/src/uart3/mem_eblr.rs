#[doc = "Register `MEM_EBLR` reader"]
pub type R = crate::R<MemEblrSpec>;
#[doc = "Register `MEM_EBLR` writer"]
pub type W = crate::W<MemEblrSpec>;
#[doc = "Field `EBLR` reader - 7:0\\]
IR-IRDA mode: This register allows to define up to 176 xBOFs, the maximum required by IrDA specification. IR-CIR mode: This register specifies the number of consecutive zeros to be received before generating the RX_STOP interrupt \\[IIR\\[2\\]\\]. 0x00: feature disabled. 0x01: generate RX_STOP interrupt after receiving one zero bit. ... 0xFF: generate RX_STOP interrupt after receiving 255 zero bits."]
pub type EblrR = crate::FieldReader;
#[doc = "Field `EBLR` writer - 7:0\\]
IR-IRDA mode: This register allows to define up to 176 xBOFs, the maximum required by IrDA specification. IR-CIR mode: This register specifies the number of consecutive zeros to be received before generating the RX_STOP interrupt \\[IIR\\[2\\]\\]. 0x00: feature disabled. 0x01: generate RX_STOP interrupt after receiving one zero bit. ... 0xFF: generate RX_STOP interrupt after receiving 255 zero bits."]
pub type EblrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
IR-IRDA mode: This register allows to define up to 176 xBOFs, the maximum required by IrDA specification. IR-CIR mode: This register specifies the number of consecutive zeros to be received before generating the RX_STOP interrupt \\[IIR\\[2\\]\\]. 0x00: feature disabled. 0x01: generate RX_STOP interrupt after receiving one zero bit. ... 0xFF: generate RX_STOP interrupt after receiving 255 zero bits."]
    #[inline(always)]
    pub fn eblr(&self) -> EblrR {
        EblrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
IR-IRDA mode: This register allows to define up to 176 xBOFs, the maximum required by IrDA specification. IR-CIR mode: This register specifies the number of consecutive zeros to be received before generating the RX_STOP interrupt \\[IIR\\[2\\]\\]. 0x00: feature disabled. 0x01: generate RX_STOP interrupt after receiving one zero bit. ... 0xFF: generate RX_STOP interrupt after receiving 255 zero bits."]
    #[inline(always)]
    #[must_use]
    pub fn eblr(&mut self) -> EblrW<MemEblrSpec> {
        EblrW::new(self, 0)
    }
}
#[doc = "IR-IrDA and IR-CIR modes only. In IR-IrDA SIR operation, this register specifies the number of BOF + xBOFs to transmit. Value set into this register must take into account the BOF character, therefore to only sent one BOF with no XBOF this register must be set to 1. To send one BOF with N XBOF this register must be set to N+1. Furthermore, the value 0 will send 1 BOF plus 255 XBOF. In IR-IrDA MIR mode, this register specifies the number of additional start flags (MIR protocol mandates a minimum of 2 start flags). In IR-CIR mode, this register specifies the number of consecutive zeros to be received before generating the RX_STOP interrupt (IIR\\[2\\]). All the received zeros are stored in the RX FIFO. When the register is set to 0, this feature is de-activated and always in reception state which can be disabled by setting the ACREG\\[5\\]
to\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_eblr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_eblr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemEblrSpec;
impl crate::RegisterSpec for MemEblrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_eblr::R`](R) reader structure"]
impl crate::Readable for MemEblrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_eblr::W`](W) writer structure"]
impl crate::Writable for MemEblrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_EBLR to value 0"]
impl crate::Resettable for MemEblrSpec {
    const RESET_VALUE: u32 = 0;
}
