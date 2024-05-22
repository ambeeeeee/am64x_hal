#[doc = "Register `CFG0_JTAG_USER_ID` reader"]
pub type R = crate::R<Cfg0JtagUserIdSpec>;
#[doc = "Register `CFG0_JTAG_USER_ID` writer"]
pub type W = crate::W<Cfg0JtagUserIdSpec>;
#[doc = "Field `JTAG_USER_ID_USERCODE` reader - 31:0\\]
Device information"]
pub type JtagUserIdUsercodeR = crate::FieldReader<u32>;
#[doc = "Field `JTAG_USER_ID_USERCODE` writer - 31:0\\]
Device information"]
pub type JtagUserIdUsercodeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Device information"]
    #[inline(always)]
    pub fn jtag_user_id_usercode(&self) -> JtagUserIdUsercodeR {
        JtagUserIdUsercodeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Device information"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_user_id_usercode(&mut self) -> JtagUserIdUsercodeW<Cfg0JtagUserIdSpec> {
        JtagUserIdUsercodeW::new(self, 0)
    }
}
#[doc = "CFG0_JTAG_USER_ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_jtag_user_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_jtag_user_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0JtagUserIdSpec;
impl crate::RegisterSpec for Cfg0JtagUserIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_jtag_user_id::R`](R) reader structure"]
impl crate::Readable for Cfg0JtagUserIdSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_jtag_user_id::W`](W) writer structure"]
impl crate::Writable for Cfg0JtagUserIdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_JTAG_USER_ID to value 0"]
impl crate::Resettable for Cfg0JtagUserIdSpec {
    const RESET_VALUE: u32 = 0;
}
