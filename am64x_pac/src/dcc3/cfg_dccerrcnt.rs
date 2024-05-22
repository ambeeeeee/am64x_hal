#[doc = "Register `CFG_DCCERRCNT` reader"]
pub type R = crate::R<CfgDccerrcntSpec>;
#[doc = "Register `CFG_DCCERRCNT` writer"]
pub type W = crate::W<CfgDccerrcntSpec>;
#[doc = "Field `ERRCNT` reader - 9:0\\]
Counts the number of errors after the last write to this register or reset. If reached terminal count the count freezes. User needs to clear it."]
pub type ErrcntR = crate::FieldReader<u16>;
#[doc = "Field `ERRCNT` writer - 9:0\\]
Counts the number of errors after the last write to this register or reset. If reached terminal count the count freezes. User needs to clear it."]
pub type ErrcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Counts the number of errors after the last write to this register or reset. If reached terminal count the count freezes. User needs to clear it."]
    #[inline(always)]
    pub fn errcnt(&self) -> ErrcntR {
        ErrcntR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Counts the number of errors after the last write to this register or reset. If reached terminal count the count freezes. User needs to clear it."]
    #[inline(always)]
    #[must_use]
    pub fn errcnt(&mut self) -> ErrcntW<CfgDccerrcntSpec> {
        ErrcntW::new(self, 0)
    }
}
#[doc = "Counts number of errors since last clear.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccerrcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccerrcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDccerrcntSpec;
impl crate::RegisterSpec for CfgDccerrcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dccerrcnt::R`](R) reader structure"]
impl crate::Readable for CfgDccerrcntSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dccerrcnt::W`](W) writer structure"]
impl crate::Writable for CfgDccerrcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DCCERRCNT to value 0"]
impl crate::Resettable for CfgDccerrcntSpec {
    const RESET_VALUE: u32 = 0;
}
