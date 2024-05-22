#[doc = "Register `CFG_I2C_SYSC` reader"]
pub type R = crate::R<CfgI2cSyscSpec>;
#[doc = "Register `CFG_I2C_SYSC` writer"]
pub type W = crate::W<CfgI2cSyscSpec>;
#[doc = "Field `AUTOIDLE` reader - 0:0\\]
Autoidle bit"]
pub type AutoidleR = crate::BitReader;
#[doc = "Field `AUTOIDLE` writer - 0:0\\]
Autoidle bit"]
pub type AutoidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRST` reader - 1:1\\]
SoftReset bit"]
pub type SrstR = crate::BitReader;
#[doc = "Field `SRST` writer - 1:1\\]
SoftReset bit"]
pub type SrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENAWAKEUP` reader - 2:2\\]
Enable Wakeup control bit"]
pub type EnawakeupR = crate::BitReader;
#[doc = "Field `ENAWAKEUP` writer - 2:2\\]
Enable Wakeup control bit"]
pub type EnawakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEMODE` reader - 4:3\\]
Idle Mode selection bits"]
pub type IdlemodeR = crate::FieldReader;
#[doc = "Field `IDLEMODE` writer - 4:3\\]
Idle Mode selection bits"]
pub type IdlemodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLKACTIVITY` reader - 9:8\\]
Clock Activity selection bits"]
pub type ClkactivityR = crate::FieldReader;
#[doc = "Field `CLKACTIVITY` writer - 9:8\\]
Clock Activity selection bits"]
pub type ClkactivityW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Autoidle bit"]
    #[inline(always)]
    pub fn autoidle(&self) -> AutoidleR {
        AutoidleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SoftReset bit"]
    #[inline(always)]
    pub fn srst(&self) -> SrstR {
        SrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable Wakeup control bit"]
    #[inline(always)]
    pub fn enawakeup(&self) -> EnawakeupR {
        EnawakeupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Idle Mode selection bits"]
    #[inline(always)]
    pub fn idlemode(&self) -> IdlemodeR {
        IdlemodeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Clock Activity selection bits"]
    #[inline(always)]
    pub fn clkactivity(&self) -> ClkactivityR {
        ClkactivityR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Autoidle bit"]
    #[inline(always)]
    #[must_use]
    pub fn autoidle(&mut self) -> AutoidleW<CfgI2cSyscSpec> {
        AutoidleW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SoftReset bit"]
    #[inline(always)]
    #[must_use]
    pub fn srst(&mut self) -> SrstW<CfgI2cSyscSpec> {
        SrstW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable Wakeup control bit"]
    #[inline(always)]
    #[must_use]
    pub fn enawakeup(&mut self) -> EnawakeupW<CfgI2cSyscSpec> {
        EnawakeupW::new(self, 2)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Idle Mode selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn idlemode(&mut self) -> IdlemodeW<CfgI2cSyscSpec> {
        IdlemodeW::new(self, 3)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Clock Activity selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn clkactivity(&mut self) -> ClkactivityW<CfgI2cSyscSpec> {
        ClkactivityW::new(self, 8)
    }
}
#[doc = "System Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_sysc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_sysc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cSyscSpec;
impl crate::RegisterSpec for CfgI2cSyscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_sysc::R`](R) reader structure"]
impl crate::Readable for CfgI2cSyscSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_sysc::W`](W) writer structure"]
impl crate::Writable for CfgI2cSyscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_SYSC to value 0x01"]
impl crate::Resettable for CfgI2cSyscSpec {
    const RESET_VALUE: u32 = 0x01;
}
