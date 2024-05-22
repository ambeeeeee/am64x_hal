#[doc = "Register `CFG0_intr_raw_status` reader"]
pub type R = crate::R<Cfg0IntrRawStatusSpec>;
#[doc = "Register `CFG0_intr_raw_status` writer"]
pub type W = crate::W<Cfg0IntrRawStatusSpec>;
#[doc = "Field `PROT_ERR` reader - 0:0\\]
Protection violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type ProtErrR = crate::BitReader;
#[doc = "Field `PROT_ERR` writer - 0:0\\]
Protection violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type ProtErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_ERR` reader - 1:1\\]
Addressing violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type AddrErrR = crate::BitReader;
#[doc = "Field `ADDR_ERR` writer - 1:1\\]
Addressing violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type AddrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KICK_ERR` reader - 2:2\\]
Kick access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type KickErrR = crate::BitReader;
#[doc = "Field `KICK_ERR` writer - 2:2\\]
Kick access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type KickErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROXY_ERR` reader - 3:3\\]
Proxy0 access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type ProxyErrR = crate::BitReader;
#[doc = "Field `PROXY_ERR` writer - 3:3\\]
Proxy0 access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type ProxyErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn prot_err(&self) -> ProtErrR {
        ProtErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn addr_err(&self) -> AddrErrR {
        AddrErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn kick_err(&self) -> KickErrR {
        KickErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn proxy_err(&self) -> ProxyErrR {
        ProxyErrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn prot_err(&mut self) -> ProtErrW<Cfg0IntrRawStatusSpec> {
        ProtErrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn addr_err(&mut self) -> AddrErrW<Cfg0IntrRawStatusSpec> {
        AddrErrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn kick_err(&mut self) -> KickErrW<Cfg0IntrRawStatusSpec> {
        KickErrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn proxy_err(&mut self) -> ProxyErrW<Cfg0IntrRawStatusSpec> {
        ProxyErrW::new(self, 3)
    }
}
#[doc = "CFG0_intr_raw_status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_intr_raw_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_intr_raw_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0IntrRawStatusSpec;
impl crate::RegisterSpec for Cfg0IntrRawStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_intr_raw_status::R`](R) reader structure"]
impl crate::Readable for Cfg0IntrRawStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_intr_raw_status::W`](W) writer structure"]
impl crate::Writable for Cfg0IntrRawStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_intr_raw_status to value 0"]
impl crate::Resettable for Cfg0IntrRawStatusSpec {
    const RESET_VALUE: u32 = 0;
}
