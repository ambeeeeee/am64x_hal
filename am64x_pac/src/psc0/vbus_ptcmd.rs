#[doc = "Register `VBUS_PTCMD` reader"]
pub type R = crate::R<VbusPtcmdSpec>;
#[doc = "Register `VBUS_PTCMD` writer"]
pub type W = crate::W<VbusPtcmdSpec>;
#[doc = "Field `GO` reader - 31:0\\]
Power Domain n GO Transition"]
pub type GoR = crate::FieldReader<u32>;
#[doc = "Field `GO` writer - 31:0\\]
Power Domain n GO Transition"]
pub type GoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Power Domain n GO Transition"]
    #[inline(always)]
    pub fn go(&self) -> GoR {
        GoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Power Domain n GO Transition"]
    #[inline(always)]
    #[must_use]
    pub fn go(&mut self) -> GoW<VbusPtcmdSpec> {
        GoW::new(self, 0)
    }
}
#[doc = "This is a pseudo-command register with no actual storage. Reads return 0. One bit for each power domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_ptcmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_ptcmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusPtcmdSpec;
impl crate::RegisterSpec for VbusPtcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_ptcmd::R`](R) reader structure"]
impl crate::Readable for VbusPtcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_ptcmd::W`](W) writer structure"]
impl crate::Writable for VbusPtcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_PTCMD to value 0"]
impl crate::Resettable for VbusPtcmdSpec {
    const RESET_VALUE: u32 = 0;
}
