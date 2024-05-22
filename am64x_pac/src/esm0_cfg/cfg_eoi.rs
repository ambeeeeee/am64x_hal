#[doc = "Register `CFG_EOI` reader"]
pub type R = crate::R<CfgEoiSpec>;
#[doc = "Register `CFG_EOI` writer"]
pub type W = crate::W<CfgEoiSpec>;
#[doc = "Field `KEY` reader - 10:0\\]
This is the interrupt being serviced"]
pub type KeyR = crate::FieldReader<u16>;
#[doc = "Field `KEY` writer - 10:0\\]
This is the interrupt being serviced"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
This is the interrupt being serviced"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
This is the interrupt being serviced"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CfgEoiSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "End of Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_eoi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_eoi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgEoiSpec;
impl crate::RegisterSpec for CfgEoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_eoi::R`](R) reader structure"]
impl crate::Readable for CfgEoiSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_eoi::W`](W) writer structure"]
impl crate::Writable for CfgEoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_EOI to value 0"]
impl crate::Resettable for CfgEoiSpec {
    const RESET_VALUE: u32 = 0;
}
