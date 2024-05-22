#[doc = "Register `INTR_ROUTER_CFG_PID` reader"]
pub type R = crate::R<IntrRouterCfgPidSpec>;
#[doc = "Register `INTR_ROUTER_CFG_PID` writer"]
pub type W = crate::W<IntrRouterCfgPidSpec>;
#[doc = "Field `MINREV` reader - 5:0\\]
minor version"]
pub type MinrevR = crate::FieldReader;
#[doc = "Field `MINREV` writer - 5:0\\]
minor version"]
pub type MinrevW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
custom id"]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
custom id"]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAJREV` reader - 10:8\\]
major version"]
pub type MajrevR = crate::FieldReader;
#[doc = "Field `MAJREV` writer - 10:8\\]
major version"]
pub type MajrevW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTLVER` reader - 15:11\\]
rtl version"]
pub type RtlverR = crate::FieldReader;
#[doc = "Field `RTLVER` writer - 15:11\\]
rtl version"]
pub type RtlverW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FUNCTION` reader - 27:16\\]
function"]
pub type FunctionR = crate::FieldReader<u16>;
#[doc = "Field `FUNCTION` writer - 27:16\\]
function"]
pub type FunctionW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BU` reader - 29:28\\]
bu"]
pub type BuR = crate::FieldReader;
#[doc = "Field `BU` writer - 29:28\\]
bu"]
pub type BuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCHEME` reader - 31:30\\]
scheme"]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - 31:30\\]
scheme"]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
minor version"]
    #[inline(always)]
    pub fn minrev(&self) -> MinrevR {
        MinrevR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
custom id"]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
major version"]
    #[inline(always)]
    pub fn majrev(&self) -> MajrevR {
        MajrevR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
rtl version"]
    #[inline(always)]
    pub fn rtlver(&self) -> RtlverR {
        RtlverR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
function"]
    #[inline(always)]
    pub fn function(&self) -> FunctionR {
        FunctionR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
bu"]
    #[inline(always)]
    pub fn bu(&self) -> BuR {
        BuR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
scheme"]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
minor version"]
    #[inline(always)]
    #[must_use]
    pub fn minrev(&mut self) -> MinrevW<IntrRouterCfgPidSpec> {
        MinrevW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
custom id"]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<IntrRouterCfgPidSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
major version"]
    #[inline(always)]
    #[must_use]
    pub fn majrev(&mut self) -> MajrevW<IntrRouterCfgPidSpec> {
        MajrevW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
rtl version"]
    #[inline(always)]
    #[must_use]
    pub fn rtlver(&mut self) -> RtlverW<IntrRouterCfgPidSpec> {
        RtlverW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
function"]
    #[inline(always)]
    #[must_use]
    pub fn function(&mut self) -> FunctionW<IntrRouterCfgPidSpec> {
        FunctionW::new(self, 16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
bu"]
    #[inline(always)]
    #[must_use]
    pub fn bu(&mut self) -> BuW<IntrRouterCfgPidSpec> {
        BuW::new(self, 28)
    }
    #[doc = "Bits 30:31 - 31:30\\]
scheme"]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<IntrRouterCfgPidSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "Identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_router_cfg_pid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_router_cfg_pid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrRouterCfgPidSpec;
impl crate::RegisterSpec for IntrRouterCfgPidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_router_cfg_pid::R`](R) reader structure"]
impl crate::Readable for IntrRouterCfgPidSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_router_cfg_pid::W`](W) writer structure"]
impl crate::Writable for IntrRouterCfgPidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_ROUTER_CFG_PID to value 0x7684_b100"]
impl crate::Resettable for IntrRouterCfgPidSpec {
    const RESET_VALUE: u32 = 0x7684_b100;
}
