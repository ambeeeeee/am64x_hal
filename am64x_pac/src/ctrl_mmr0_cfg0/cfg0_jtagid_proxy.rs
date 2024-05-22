#[doc = "Register `CFG0_JTAGID_PROXY` reader"]
pub type R = crate::R<Cfg0JtagidProxySpec>;
#[doc = "Register `CFG0_JTAGID_PROXY` writer"]
pub type W = crate::W<Cfg0JtagidProxySpec>;
#[doc = "Field `JTAGID_LSB_PROXY` reader - 0:0\\]
Always 1"]
pub type JtagidLsbProxyR = crate::BitReader;
#[doc = "Field `JTAGID_LSB_PROXY` writer - 0:0\\]
Always 1"]
pub type JtagidLsbProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAGID_MFG_PROXY` reader - 11:1\\]
Indicates manufacturer"]
pub type JtagidMfgProxyR = crate::FieldReader<u16>;
#[doc = "Field `JTAGID_MFG_PROXY` writer - 11:1\\]
Indicates manufacturer"]
pub type JtagidMfgProxyW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `JTAGID_PARTNO_PROXY` reader - 27:12\\]
Part number for boundary scan"]
pub type JtagidPartnoProxyR = crate::FieldReader<u16>;
#[doc = "Field `JTAGID_PARTNO_PROXY` writer - 27:12\\]
Part number for boundary scan"]
pub type JtagidPartnoProxyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `JTAGID_VARIANT_PROXY` reader - 31:28\\]
Used to indicate new PGs"]
pub type JtagidVariantProxyR = crate::FieldReader;
#[doc = "Field `JTAGID_VARIANT_PROXY` writer - 31:28\\]
Used to indicate new PGs"]
pub type JtagidVariantProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Always 1"]
    #[inline(always)]
    pub fn jtagid_lsb_proxy(&self) -> JtagidLsbProxyR {
        JtagidLsbProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:11 - 11:1\\]
Indicates manufacturer"]
    #[inline(always)]
    pub fn jtagid_mfg_proxy(&self) -> JtagidMfgProxyR {
        JtagidMfgProxyR::new(((self.bits >> 1) & 0x07ff) as u16)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Part number for boundary scan"]
    #[inline(always)]
    pub fn jtagid_partno_proxy(&self) -> JtagidPartnoProxyR {
        JtagidPartnoProxyR::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Used to indicate new PGs"]
    #[inline(always)]
    pub fn jtagid_variant_proxy(&self) -> JtagidVariantProxyR {
        JtagidVariantProxyR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Always 1"]
    #[inline(always)]
    #[must_use]
    pub fn jtagid_lsb_proxy(&mut self) -> JtagidLsbProxyW<Cfg0JtagidProxySpec> {
        JtagidLsbProxyW::new(self, 0)
    }
    #[doc = "Bits 1:11 - 11:1\\]
Indicates manufacturer"]
    #[inline(always)]
    #[must_use]
    pub fn jtagid_mfg_proxy(&mut self) -> JtagidMfgProxyW<Cfg0JtagidProxySpec> {
        JtagidMfgProxyW::new(self, 1)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Part number for boundary scan"]
    #[inline(always)]
    #[must_use]
    pub fn jtagid_partno_proxy(&mut self) -> JtagidPartnoProxyW<Cfg0JtagidProxySpec> {
        JtagidPartnoProxyW::new(self, 12)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Used to indicate new PGs"]
    #[inline(always)]
    #[must_use]
    pub fn jtagid_variant_proxy(&mut self) -> JtagidVariantProxyW<Cfg0JtagidProxySpec> {
        JtagidVariantProxyW::new(self, 28)
    }
}
#[doc = "CFG0_JTAGID_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_jtagid_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_jtagid_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0JtagidProxySpec;
impl crate::RegisterSpec for Cfg0JtagidProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_jtagid_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0JtagidProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_jtagid_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0JtagidProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_JTAGID_PROXY to value 0x5792_8047"]
impl crate::Resettable for Cfg0JtagidProxySpec {
    const RESET_VALUE: u32 = 0x5792_8047;
}
