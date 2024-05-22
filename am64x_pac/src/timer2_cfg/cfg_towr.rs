#[doc = "Register `CFG_TOWR` reader"]
pub type R = crate::R<CfgTowrSpec>;
#[doc = "Register `CFG_TOWR` writer"]
pub type W = crate::W<CfgTowrSpec>;
#[doc = "Field `OVF_WRAPPING_VALUE` reader - 23:0\\]
The number of masked interrupts"]
pub type OvfWrappingValueR = crate::FieldReader<u32>;
#[doc = "Field `OVF_WRAPPING_VALUE` writer - 23:0\\]
The number of masked interrupts"]
pub type OvfWrappingValueW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
The number of masked interrupts"]
    #[inline(always)]
    pub fn ovf_wrapping_value(&self) -> OvfWrappingValueR {
        OvfWrappingValueR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
The number of masked interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn ovf_wrapping_value(&mut self) -> OvfWrappingValueW<CfgTowrSpec> {
        OvfWrappingValueW::new(self, 0)
    }
}
#[doc = "This register holds the number of masked overflow interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_towr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_towr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTowrSpec;
impl crate::RegisterSpec for CfgTowrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_towr::R`](R) reader structure"]
impl crate::Readable for CfgTowrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_towr::W`](W) writer structure"]
impl crate::Writable for CfgTowrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TOWR to value 0"]
impl crate::Resettable for CfgTowrSpec {
    const RESET_VALUE: u32 = 0;
}
