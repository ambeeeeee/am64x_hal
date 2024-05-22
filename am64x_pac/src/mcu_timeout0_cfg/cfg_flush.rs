#[doc = "Register `CFG_FLUSH` reader"]
pub type R = crate::R<CfgFlushSpec>;
#[doc = "Register `CFG_FLUSH` writer"]
pub type W = crate::W<CfgFlushSpec>;
#[doc = "Field `FL` reader - 3:0\\]
Enable. 4'b1111 - Flush, All other values - Normal."]
pub type FlR = crate::FieldReader;
#[doc = "Field `FL` writer - 3:0\\]
Enable. 4'b1111 - Flush, All other values - Normal."]
pub type FlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXT_FL` reader - 31:31\\]
External Flush Value"]
pub type ExtFlR = crate::BitReader;
#[doc = "Field `EXT_FL` writer - 31:31\\]
External Flush Value"]
pub type ExtFlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Enable. 4'b1111 - Flush, All other values - Normal."]
    #[inline(always)]
    pub fn fl(&self) -> FlR {
        FlR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
External Flush Value"]
    #[inline(always)]
    pub fn ext_fl(&self) -> ExtFlR {
        ExtFlR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Enable. 4'b1111 - Flush, All other values - Normal."]
    #[inline(always)]
    #[must_use]
    pub fn fl(&mut self) -> FlW<CfgFlushSpec> {
        FlW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
External Flush Value"]
    #[inline(always)]
    #[must_use]
    pub fn ext_fl(&mut self) -> ExtFlW<CfgFlushSpec> {
        ExtFlW::new(self, 31)
    }
}
#[doc = "The Flush Register contains software flush control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_flush::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_flush::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgFlushSpec;
impl crate::RegisterSpec for CfgFlushSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_flush::R`](R) reader structure"]
impl crate::Readable for CfgFlushSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_flush::W`](W) writer structure"]
impl crate::Writable for CfgFlushSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_FLUSH to value 0"]
impl crate::Resettable for CfgFlushSpec {
    const RESET_VALUE: u32 = 0;
}
