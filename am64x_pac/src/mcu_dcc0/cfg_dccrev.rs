#[doc = "Register `CFG_DCCREV` reader"]
pub type R = crate::R<CfgDccrevSpec>;
#[doc = "Register `CFG_DCCREV` writer"]
pub type W = crate::W<CfgDccrevSpec>;
#[doc = "Field `MINOR` reader - 5:0\\]
Represents minor changes to the module (e.g. enhancements to existing features). The minor revision number for this module. User, privilege, and debug mode (read): 0x4 Privilege and debug mode (write): Writes have no effect."]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MINOR` writer - 5:0\\]
Represents minor changes to the module (e.g. enhancements to existing features). The minor revision number for this module. User, privilege, and debug mode (read): 0x4 Privilege and debug mode (write): Writes have no effect."]
pub type MinorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
Indicates a special version of the module. May not be supported by standard software. User, privilege, and debug mode (read): 0x0 Privilege and debug mode (write): Writes have no effect."]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
Indicates a special version of the module. May not be supported by standard software. User, privilege, and debug mode (read): 0x0 Privilege and debug mode (write): Writes have no effect."]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAJOR` reader - 10:8\\]
Represents major changes to the module (e.g. entirely new features are added/changed). The major revision number for this module. User, privilege, and debug mode (read): 0x2 Privilege and debug mode (write): Writes have no effect."]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - 10:8\\]
Represents major changes to the module (e.g. entirely new features are added/changed). The major revision number for this module. User, privilege, and debug mode (read): 0x2 Privilege and debug mode (write): Writes have no effect."]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL` reader - 15:11\\]
Incremented for releases due to spec changes or post-release design changes. Reset to 0 when either MAJOR or MINOR is incremented. User, privilege, and debug mode (read): 0x0 Privilege and debug mode (write): Writes have no effect."]
pub type RtlR = crate::FieldReader;
#[doc = "Field `RTL` writer - 15:11\\]
Incremented for releases due to spec changes or post-release design changes. Reset to 0 when either MAJOR or MINOR is incremented. User, privilege, and debug mode (read): 0x0 Privilege and debug mode (write): Writes have no effect."]
pub type RtlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FUNC` reader - 27:16\\]
Reflects software-compatability. If there is no level of software compatability, a unique func number is assigned; for compatible modules, the same number is maintained. User, privilege, and debug mode (read): 0x0 Privilege and debug mode (write): Writes have no effect."]
pub type FuncR = crate::FieldReader<u16>;
#[doc = "Field `FUNC` writer - 27:16\\]
Reflects software-compatability. If there is no level of software compatability, a unique func number is assigned; for compatible modules, the same number is maintained. User, privilege, and debug mode (read): 0x0 Privilege and debug mode (write): Writes have no effect."]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SCHEME` reader - 31:30\\]
User, privilege, and debug mode (read): Returns 01. Privilege and debug mode (write): Writes have no effect."]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - 31:30\\]
User, privilege, and debug mode (read): Returns 01. Privilege and debug mode (write): Writes have no effect."]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Represents minor changes to the module (e.g. enhancements to existing features). The minor revision number for this module. User, privilege, and debug mode (read): 0x4 Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Indicates a special version of the module. May not be supported by standard software. User, privilege, and debug mode (read): 0x0 Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Represents major changes to the module (e.g. entirely new features are added/changed). The major revision number for this module. User, privilege, and debug mode (read): 0x2 Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Incremented for releases due to spec changes or post-release design changes. Reset to 0 when either MAJOR or MINOR is incremented. User, privilege, and debug mode (read): 0x0 Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    pub fn rtl(&self) -> RtlR {
        RtlR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Reflects software-compatability. If there is no level of software compatability, a unique func number is assigned; for compatible modules, the same number is maintained. User, privilege, and debug mode (read): 0x0 Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31 - 31:30\\]
User, privilege, and debug mode (read): Returns 01. Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Represents minor changes to the module (e.g. enhancements to existing features). The minor revision number for this module. User, privilege, and debug mode (read): 0x4 Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn minor(&mut self) -> MinorW<CfgDccrevSpec> {
        MinorW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Indicates a special version of the module. May not be supported by standard software. User, privilege, and debug mode (read): 0x0 Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<CfgDccrevSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Represents major changes to the module (e.g. entirely new features are added/changed). The major revision number for this module. User, privilege, and debug mode (read): 0x2 Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MajorW<CfgDccrevSpec> {
        MajorW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Incremented for releases due to spec changes or post-release design changes. Reset to 0 when either MAJOR or MINOR is incremented. User, privilege, and debug mode (read): 0x0 Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rtl(&mut self) -> RtlW<CfgDccrevSpec> {
        RtlW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Reflects software-compatability. If there is no level of software compatability, a unique func number is assigned; for compatible modules, the same number is maintained. User, privilege, and debug mode (read): 0x0 Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FuncW<CfgDccrevSpec> {
        FuncW::new(self, 16)
    }
    #[doc = "Bits 30:31 - 31:30\\]
User, privilege, and debug mode (read): Returns 01. Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<CfgDccrevSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "Specifies the module version.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccrev::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccrev::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDccrevSpec;
impl crate::RegisterSpec for CfgDccrevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dccrev::R`](R) reader structure"]
impl crate::Readable for CfgDccrevSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dccrev::W`](W) writer structure"]
impl crate::Writable for CfgDccrevSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DCCREV to value 0x4001_0300"]
impl crate::Resettable for CfgDccrevSpec {
    const RESET_VALUE: u32 = 0x4001_0300;
}
