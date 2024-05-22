#[doc = "Register `CFG_GPMC_CONFIG` reader"]
pub type R = crate::R<CfgGpmcConfigSpec>;
#[doc = "Register `CFG_GPMC_CONFIG` writer"]
pub type W = crate::W<CfgGpmcConfigSpec>;
#[doc = "Field `NANDFORCEPOSTEDWRITE` reader - 0:0\\]
Enables the Force Posted Write feature to NAND Cmd/Add/Data location"]
pub type NandforcepostedwriteR = crate::BitReader;
#[doc = "Field `NANDFORCEPOSTEDWRITE` writer - 0:0\\]
Enables the Force Posted Write feature to NAND Cmd/Add/Data location"]
pub type NandforcepostedwriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LIMITEDADDRESS` reader - 1:1\\]
Limited Address device support"]
pub type LimitedaddressR = crate::BitReader;
#[doc = "Field `LIMITEDADDRESS` writer - 1:1\\]
Limited Address device support"]
pub type LimitedaddressW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITEPROTECT` reader - 4:4\\]
Controls the WP output pin level"]
pub type WriteprotectR = crate::BitReader;
#[doc = "Field `WRITEPROTECT` writer - 4:4\\]
Controls the WP output pin level"]
pub type WriteprotectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT0PINPOLARITY` reader - 8:8\\]
Selects the polarity of input pin WAIT0"]
pub type Wait0pinpolarityR = crate::BitReader;
#[doc = "Field `WAIT0PINPOLARITY` writer - 8:8\\]
Selects the polarity of input pin WAIT0"]
pub type Wait0pinpolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT1PINPOLARITY` reader - 9:9\\]
Selects the polarity of input pin WAIT1"]
pub type Wait1pinpolarityR = crate::BitReader;
#[doc = "Field `WAIT1PINPOLARITY` writer - 9:9\\]
Selects the polarity of input pin WAIT1"]
pub type Wait1pinpolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT2PINPOLARITY` reader - 10:10\\]
Selects the polarity of input pin WAIT2"]
pub type Wait2pinpolarityR = crate::BitReader;
#[doc = "Field `WAIT2PINPOLARITY` writer - 10:10\\]
Selects the polarity of input pin WAIT2"]
pub type Wait2pinpolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT3PINPOLARITY` reader - 11:11\\]
Selects the polarity of input pin WAIT3"]
pub type Wait3pinpolarityR = crate::BitReader;
#[doc = "Field `WAIT3PINPOLARITY` writer - 11:11\\]
Selects the polarity of input pin WAIT3"]
pub type Wait3pinpolarityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the Force Posted Write feature to NAND Cmd/Add/Data location"]
    #[inline(always)]
    pub fn nandforcepostedwrite(&self) -> NandforcepostedwriteR {
        NandforcepostedwriteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Limited Address device support"]
    #[inline(always)]
    pub fn limitedaddress(&self) -> LimitedaddressR {
        LimitedaddressR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Controls the WP output pin level"]
    #[inline(always)]
    pub fn writeprotect(&self) -> WriteprotectR {
        WriteprotectR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Selects the polarity of input pin WAIT0"]
    #[inline(always)]
    pub fn wait0pinpolarity(&self) -> Wait0pinpolarityR {
        Wait0pinpolarityR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Selects the polarity of input pin WAIT1"]
    #[inline(always)]
    pub fn wait1pinpolarity(&self) -> Wait1pinpolarityR {
        Wait1pinpolarityR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Selects the polarity of input pin WAIT2"]
    #[inline(always)]
    pub fn wait2pinpolarity(&self) -> Wait2pinpolarityR {
        Wait2pinpolarityR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Selects the polarity of input pin WAIT3"]
    #[inline(always)]
    pub fn wait3pinpolarity(&self) -> Wait3pinpolarityR {
        Wait3pinpolarityR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the Force Posted Write feature to NAND Cmd/Add/Data location"]
    #[inline(always)]
    #[must_use]
    pub fn nandforcepostedwrite(&mut self) -> NandforcepostedwriteW<CfgGpmcConfigSpec> {
        NandforcepostedwriteW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Limited Address device support"]
    #[inline(always)]
    #[must_use]
    pub fn limitedaddress(&mut self) -> LimitedaddressW<CfgGpmcConfigSpec> {
        LimitedaddressW::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
Controls the WP output pin level"]
    #[inline(always)]
    #[must_use]
    pub fn writeprotect(&mut self) -> WriteprotectW<CfgGpmcConfigSpec> {
        WriteprotectW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Selects the polarity of input pin WAIT0"]
    #[inline(always)]
    #[must_use]
    pub fn wait0pinpolarity(&mut self) -> Wait0pinpolarityW<CfgGpmcConfigSpec> {
        Wait0pinpolarityW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Selects the polarity of input pin WAIT1"]
    #[inline(always)]
    #[must_use]
    pub fn wait1pinpolarity(&mut self) -> Wait1pinpolarityW<CfgGpmcConfigSpec> {
        Wait1pinpolarityW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Selects the polarity of input pin WAIT2"]
    #[inline(always)]
    #[must_use]
    pub fn wait2pinpolarity(&mut self) -> Wait2pinpolarityW<CfgGpmcConfigSpec> {
        Wait2pinpolarityW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Selects the polarity of input pin WAIT3"]
    #[inline(always)]
    #[must_use]
    pub fn wait3pinpolarity(&mut self) -> Wait3pinpolarityW<CfgGpmcConfigSpec> {
        Wait3pinpolarityW::new(self, 11)
    }
}
#[doc = "The configuration register allows global configuration of the GPMC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcConfigSpec;
impl crate::RegisterSpec for CfgGpmcConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_config::R`](R) reader structure"]
impl crate::Readable for CfgGpmcConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_config::W`](W) writer structure"]
impl crate::Writable for CfgGpmcConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_CONFIG to value 0x0a00"]
impl crate::Resettable for CfgGpmcConfigSpec {
    const RESET_VALUE: u32 = 0x0a00;
}
