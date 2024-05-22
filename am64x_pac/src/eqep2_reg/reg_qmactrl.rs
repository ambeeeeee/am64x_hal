#[doc = "Register `REG_QMACTRL` reader"]
pub type R = crate::R<RegQmactrlSpec>;
#[doc = "Register `REG_QMACTRL` writer"]
pub type W = crate::W<RegQmactrlSpec>;
#[doc = "Field `MODE` reader - 2:0\\]
Select Mode for QMA mode:000 : QMA Module is bypassed. 001 : QMA Mode-1 operation selected010 : QMA Mode-2 operation selected011 : QMA Module is bypassed \\[reserved\\]1xx : QMA Module is bypassed \\[reserved\\]"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - 2:0\\]
Select Mode for QMA mode:000 : QMA Module is bypassed. 001 : QMA Mode-1 operation selected010 : QMA Mode-2 operation selected011 : QMA Module is bypassed \\[reserved\\]1xx : QMA Module is bypassed \\[reserved\\]"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Select Mode for QMA mode:000 : QMA Module is bypassed. 001 : QMA Mode-1 operation selected010 : QMA Mode-2 operation selected011 : QMA Module is bypassed \\[reserved\\]1xx : QMA Module is bypassed \\[reserved\\]"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Select Mode for QMA mode:000 : QMA Module is bypassed. 001 : QMA Mode-1 operation selected010 : QMA Mode-2 operation selected011 : QMA Module is bypassed \\[reserved\\]1xx : QMA Module is bypassed \\[reserved\\]"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<RegQmactrlSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "QMA Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qmactrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qmactrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQmactrlSpec;
impl crate::RegisterSpec for RegQmactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_qmactrl::R`](R) reader structure"]
impl crate::Readable for RegQmactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qmactrl::W`](W) writer structure"]
impl crate::Writable for RegQmactrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_QMACTRL to value 0"]
impl crate::Resettable for RegQmactrlSpec {
    const RESET_VALUE: u32 = 0;
}
