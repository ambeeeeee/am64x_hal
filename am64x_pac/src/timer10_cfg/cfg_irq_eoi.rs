#[doc = "Register `CFG_IRQ_EOI` reader"]
pub type R = crate::R<CfgIrqEoiSpec>;
#[doc = "Register `CFG_IRQ_EOI` writer"]
pub type W = crate::W<CfgIrqEoiSpec>;
#[doc = "Field `LINE_NUMBER` reader - 0:0\\]
Idle Mode"]
pub type LineNumberR = crate::BitReader;
#[doc = "Field `LINE_NUMBER` writer - 0:0\\]
Idle Mode"]
pub type LineNumberW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Idle Mode"]
    #[inline(always)]
    pub fn line_number(&self) -> LineNumberR {
        LineNumberR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Idle Mode"]
    #[inline(always)]
    #[must_use]
    pub fn line_number(&mut self) -> LineNumberW<CfgIrqEoiSpec> {
        LineNumberW::new(self, 0)
    }
}
#[doc = "Allows the generation of further pulses on the interrupt line, if an new interrupt event is pending, when using the pulsed output. Unused when using the level interrupt line\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_irq_eoi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_irq_eoi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgIrqEoiSpec;
impl crate::RegisterSpec for CfgIrqEoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_irq_eoi::R`](R) reader structure"]
impl crate::Readable for CfgIrqEoiSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_irq_eoi::W`](W) writer structure"]
impl crate::Writable for CfgIrqEoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_IRQ_EOI to value 0"]
impl crate::Resettable for CfgIrqEoiSpec {
    const RESET_VALUE: u32 = 0;
}
