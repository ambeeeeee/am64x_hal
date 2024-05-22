#[doc = "Register `CFG_SYSCONFIG` reader"]
pub type R = crate::R<CfgSysconfigSpec>;
#[doc = "Register `CFG_SYSCONFIG` writer"]
pub type W = crate::W<CfgSysconfigSpec>;
#[doc = "Field `AUTOIDLE` reader - 0:0\\]
Internal OCP Clock gating strategy"]
pub type AutoidleR = crate::BitReader;
#[doc = "Field `AUTOIDLE` writer - 0:0\\]
Internal OCP Clock gating strategy"]
pub type AutoidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTRESET` reader - 1:1\\]
Software reset During reads it always returns 0"]
pub type SoftresetR = crate::BitReader;
#[doc = "Field `SOFTRESET` writer - 1:1\\]
Software reset During reads it always returns 0"]
pub type SoftresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENAWAKEUP` reader - 2:2\\]
WakeUp feature control"]
pub type EnawakeupR = crate::BitReader;
#[doc = "Field `ENAWAKEUP` writer - 2:2\\]
WakeUp feature control"]
pub type EnawakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIDLEMODE` reader - 4:3\\]
Power management"]
pub type SidlemodeR = crate::FieldReader;
#[doc = "Field `SIDLEMODE` writer - 4:3\\]
Power management"]
pub type SidlemodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED_15` reader - 7:5\\]
Reads returns 0"]
pub type Reserved15R = crate::FieldReader;
#[doc = "Field `RESERVED_15` writer - 7:5\\]
Reads returns 0"]
pub type Reserved15W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLOCKACTIVITY` reader - 9:8\\]
Clocks activity during wake up mode period"]
pub type ClockactivityR = crate::FieldReader;
#[doc = "Field `CLOCKACTIVITY` writer - 9:8\\]
Clocks activity during wake up mode period"]
pub type ClockactivityW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED_14` reader - 31:10\\]
Reads returns 0"]
pub type Reserved14R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED_14` writer - 31:10\\]
Reads returns 0"]
pub type Reserved14W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal OCP Clock gating strategy"]
    #[inline(always)]
    pub fn autoidle(&self) -> AutoidleR {
        AutoidleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software reset During reads it always returns 0"]
    #[inline(always)]
    pub fn softreset(&self) -> SoftresetR {
        SoftresetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
WakeUp feature control"]
    #[inline(always)]
    pub fn enawakeup(&self) -> EnawakeupR {
        EnawakeupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Power management"]
    #[inline(always)]
    pub fn sidlemode(&self) -> SidlemodeR {
        SidlemodeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Reads returns 0"]
    #[inline(always)]
    pub fn reserved_15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Clocks activity during wake up mode period"]
    #[inline(always)]
    pub fn clockactivity(&self) -> ClockactivityR {
        ClockactivityR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Reads returns 0"]
    #[inline(always)]
    pub fn reserved_14(&self) -> Reserved14R {
        Reserved14R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal OCP Clock gating strategy"]
    #[inline(always)]
    #[must_use]
    pub fn autoidle(&mut self) -> AutoidleW<CfgSysconfigSpec> {
        AutoidleW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software reset During reads it always returns 0"]
    #[inline(always)]
    #[must_use]
    pub fn softreset(&mut self) -> SoftresetW<CfgSysconfigSpec> {
        SoftresetW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
WakeUp feature control"]
    #[inline(always)]
    #[must_use]
    pub fn enawakeup(&mut self) -> EnawakeupW<CfgSysconfigSpec> {
        EnawakeupW::new(self, 2)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Power management"]
    #[inline(always)]
    #[must_use]
    pub fn sidlemode(&mut self) -> SidlemodeW<CfgSysconfigSpec> {
        SidlemodeW::new(self, 3)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Reads returns 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_15(&mut self) -> Reserved15W<CfgSysconfigSpec> {
        Reserved15W::new(self, 5)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Clocks activity during wake up mode period"]
    #[inline(always)]
    #[must_use]
    pub fn clockactivity(&mut self) -> ClockactivityW<CfgSysconfigSpec> {
        ClockactivityW::new(self, 8)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Reads returns 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_14(&mut self) -> Reserved14W<CfgSysconfigSpec> {
        Reserved14W::new(self, 10)
    }
}
#[doc = "This register allows controlling various parameters of the OCP interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_sysconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_sysconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSysconfigSpec;
impl crate::RegisterSpec for CfgSysconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_sysconfig::R`](R) reader structure"]
impl crate::Readable for CfgSysconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_sysconfig::W`](W) writer structure"]
impl crate::Writable for CfgSysconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_SYSCONFIG to value 0x15"]
impl crate::Resettable for CfgSysconfigSpec {
    const RESET_VALUE: u32 = 0x15;
}
