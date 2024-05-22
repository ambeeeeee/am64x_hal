#[doc = "Register `CFG0_JTAG_USER_ID_PROXY` reader"]
pub type R = crate::R<Cfg0JtagUserIdProxySpec>;
#[doc = "Register `CFG0_JTAG_USER_ID_PROXY` writer"]
pub type W = crate::W<Cfg0JtagUserIdProxySpec>;
#[doc = "Field `JTAG_USER_ID_USERCODE_PROXY` reader - 31:0\\]
Device information"]
pub type JtagUserIdUsercodeProxyR = crate::FieldReader<u32>;
#[doc = "Field `JTAG_USER_ID_USERCODE_PROXY` writer - 31:0\\]
Device information"]
pub type JtagUserIdUsercodeProxyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Device information"]
    #[inline(always)]
    pub fn jtag_user_id_usercode_proxy(&self) -> JtagUserIdUsercodeProxyR {
        JtagUserIdUsercodeProxyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Device information"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_user_id_usercode_proxy(
        &mut self,
    ) -> JtagUserIdUsercodeProxyW<Cfg0JtagUserIdProxySpec> {
        JtagUserIdUsercodeProxyW::new(self, 0)
    }
}
#[doc = "CFG0_JTAG_USER_ID_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_jtag_user_id_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_jtag_user_id_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0JtagUserIdProxySpec;
impl crate::RegisterSpec for Cfg0JtagUserIdProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_jtag_user_id_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0JtagUserIdProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_jtag_user_id_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0JtagUserIdProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_JTAG_USER_ID_PROXY to value 0"]
impl crate::Resettable for Cfg0JtagUserIdProxySpec {
    const RESET_VALUE: u32 = 0;
}
