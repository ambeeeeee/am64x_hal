#[doc = "Register `CONFIG_TM_ID` reader"]
pub type R = crate::R<ConfigTmIdSpec>;
#[doc = "Register `CONFIG_TM_ID` writer"]
pub type W = crate::W<ConfigTmIdSpec>;
#[doc = "Field `MINOR_REV` reader - 5:0\\]
PID Minor revision number"]
pub type MinorRevR = crate::FieldReader;
#[doc = "Field `MINOR_REV` writer - 5:0\\]
PID Minor revision number"]
pub type MinorRevW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
PID custom"]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
PID custom"]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAJOR_REV` reader - 10:8\\]
PID Major revision number"]
pub type MajorRevR = crate::FieldReader;
#[doc = "Field `MAJOR_REV` writer - 10:8\\]
PID Major revision number"]
pub type MajorRevW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL_VER` reader - 15:11\\]
PID RTL version number"]
pub type RtlVerR = crate::FieldReader;
#[doc = "Field `RTL_VER` writer - 15:11\\]
PID RTL version number"]
pub type RtlVerW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FUNCTION` reader - 27:16\\]
PID function identifier"]
pub type FunctionR = crate::FieldReader<u16>;
#[doc = "Field `FUNCTION` writer - 27:16\\]
PID function identifier"]
pub type FunctionW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BU` reader - 29:28\\]
PID bu identifier"]
pub type BuR = crate::FieldReader;
#[doc = "Field `BU` writer - 29:28\\]
PID bu identifier"]
pub type BuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCHEME` reader - 31:30\\]
PID scheme"]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - 31:30\\]
PID scheme"]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
PID Minor revision number"]
    #[inline(always)]
    pub fn minor_rev(&self) -> MinorRevR {
        MinorRevR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
PID custom"]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
PID Major revision number"]
    #[inline(always)]
    pub fn major_rev(&self) -> MajorRevR {
        MajorRevR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
PID RTL version number"]
    #[inline(always)]
    pub fn rtl_ver(&self) -> RtlVerR {
        RtlVerR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
PID function identifier"]
    #[inline(always)]
    pub fn function(&self) -> FunctionR {
        FunctionR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
PID bu identifier"]
    #[inline(always)]
    pub fn bu(&self) -> BuR {
        BuR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
PID scheme"]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
PID Minor revision number"]
    #[inline(always)]
    #[must_use]
    pub fn minor_rev(&mut self) -> MinorRevW<ConfigTmIdSpec> {
        MinorRevW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
PID custom"]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<ConfigTmIdSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
PID Major revision number"]
    #[inline(always)]
    #[must_use]
    pub fn major_rev(&mut self) -> MajorRevW<ConfigTmIdSpec> {
        MajorRevW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
PID RTL version number"]
    #[inline(always)]
    #[must_use]
    pub fn rtl_ver(&mut self) -> RtlVerW<ConfigTmIdSpec> {
        RtlVerW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
PID function identifier"]
    #[inline(always)]
    #[must_use]
    pub fn function(&mut self) -> FunctionW<ConfigTmIdSpec> {
        FunctionW::new(self, 16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
PID bu identifier"]
    #[inline(always)]
    #[must_use]
    pub fn bu(&mut self) -> BuW<ConfigTmIdSpec> {
        BuW::new(self, 28)
    }
    #[doc = "Bits 30:31 - 31:30\\]
PID scheme"]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<ConfigTmIdSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "This is the standard TI peripheral ID register that exists at address 0 in the peripheral space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_tm_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_tm_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigTmIdSpec;
impl crate::RegisterSpec for ConfigTmIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_tm_id::R`](R) reader structure"]
impl crate::Readable for ConfigTmIdSpec {}
#[doc = "`write(|w| ..)` method takes [`config_tm_id::W`](W) writer structure"]
impl crate::Writable for ConfigTmIdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_TM_ID to value 0x7784_2901"]
impl crate::Resettable for ConfigTmIdSpec {
    const RESET_VALUE: u32 = 0x7784_2901;
}
