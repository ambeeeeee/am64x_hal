#[doc = "Register `CFG_INFO` reader"]
pub type R = crate::R<CfgInfoSpec>;
#[doc = "Register `CFG_INFO` writer"]
pub type W = crate::W<CfgInfoSpec>;
#[doc = "Field `GROUPS` reader - 7:0\\]
Total number of Error Groups"]
pub type GroupsR = crate::FieldReader;
#[doc = "Field `GROUPS` writer - 7:0\\]
Total number of Error Groups"]
pub type GroupsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PULSE_GROUPS` reader - 15:8\\]
Number of Pulse Error Groups"]
pub type PulseGroupsR = crate::FieldReader;
#[doc = "Field `PULSE_GROUPS` writer - 15:8\\]
Number of Pulse Error Groups"]
pub type PulseGroupsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LAST_RESET` reader - 31:31\\]
Indicates the Source of the last Reset"]
pub type LastResetR = crate::BitReader;
#[doc = "Field `LAST_RESET` writer - 31:31\\]
Indicates the Source of the last Reset"]
pub type LastResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Total number of Error Groups"]
    #[inline(always)]
    pub fn groups(&self) -> GroupsR {
        GroupsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of Pulse Error Groups"]
    #[inline(always)]
    pub fn pulse_groups(&self) -> PulseGroupsR {
        PulseGroupsR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Indicates the Source of the last Reset"]
    #[inline(always)]
    pub fn last_reset(&self) -> LastResetR {
        LastResetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Total number of Error Groups"]
    #[inline(always)]
    #[must_use]
    pub fn groups(&mut self) -> GroupsW<CfgInfoSpec> {
        GroupsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of Pulse Error Groups"]
    #[inline(always)]
    #[must_use]
    pub fn pulse_groups(&mut self) -> PulseGroupsW<CfgInfoSpec> {
        PulseGroupsW::new(self, 8)
    }
    #[doc = "Bit 31 - 31:31\\]
Indicates the Source of the last Reset"]
    #[inline(always)]
    #[must_use]
    pub fn last_reset(&mut self) -> LastResetW<CfgInfoSpec> {
        LastResetW::new(self, 31)
    }
}
#[doc = "The Info Register gives the configuration Inforrmation of this ESM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgInfoSpec;
impl crate::RegisterSpec for CfgInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_info::R`](R) reader structure"]
impl crate::Readable for CfgInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_info::W`](W) writer structure"]
impl crate::Writable for CfgInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_INFO to value 0x0103"]
impl crate::Resettable for CfgInfoSpec {
    const RESET_VALUE: u32 = 0x0103;
}
