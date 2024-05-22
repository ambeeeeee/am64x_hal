#[doc = "Register `CFG_LOW_PRI` reader"]
pub type R = crate::R<CfgLowPriSpec>;
#[doc = "Register `CFG_LOW_PRI` writer"]
pub type W = crate::W<CfgLowPriSpec>;
#[doc = "Field `LVL` reader - 15:0\\]
This is the highest priority outstanding low priority level interrupt"]
pub type LvlR = crate::FieldReader<u16>;
#[doc = "Field `LVL` writer - 15:0\\]
This is the highest priority outstanding low priority level interrupt"]
pub type LvlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PLS` reader - 31:16\\]
This is the highest priority outstanding low priority pulse interrupt"]
pub type PlsR = crate::FieldReader<u16>;
#[doc = "Field `PLS` writer - 31:16\\]
This is the highest priority outstanding low priority pulse interrupt"]
pub type PlsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
This is the highest priority outstanding low priority level interrupt"]
    #[inline(always)]
    pub fn lvl(&self) -> LvlR {
        LvlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
This is the highest priority outstanding low priority pulse interrupt"]
    #[inline(always)]
    pub fn pls(&self) -> PlsR {
        PlsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
This is the highest priority outstanding low priority level interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lvl(&mut self) -> LvlW<CfgLowPriSpec> {
        LvlW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
This is the highest priority outstanding low priority pulse interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PlsW<CfgLowPriSpec> {
        PlsW::new(self, 16)
    }
}
#[doc = "Shows which is the highest priority outstanding low priority interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_low_pri::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_low_pri::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgLowPriSpec;
impl crate::RegisterSpec for CfgLowPriSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_low_pri::R`](R) reader structure"]
impl crate::Readable for CfgLowPriSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_low_pri::W`](W) writer structure"]
impl crate::Writable for CfgLowPriSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_LOW_PRI to value 0x0006_5535"]
impl crate::Resettable for CfgLowPriSpec {
    const RESET_VALUE: u32 = 0x0006_5535;
}
