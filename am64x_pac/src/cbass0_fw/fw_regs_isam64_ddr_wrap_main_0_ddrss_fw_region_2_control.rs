#[doc = "Register `FW_REGS_Isam64_ddr_wrap_main_0_ddrss_fw_region_2_control` reader"]
pub type R = crate::R<FwRegsIsam64DdrWrapMain0DdrssFwRegion2ControlSpec>;
#[doc = "Register `FW_REGS_Isam64_ddr_wrap_main_0_ddrss_fw_region_2_control` writer"]
pub type W = crate::W<FwRegsIsam64DdrWrapMain0DdrssFwRegion2ControlSpec>;
#[doc = "Field `ENABLE` reader - 3:0\\]
Enable region. A value of 0xA enables, others disable."]
pub type EnableR = crate::FieldReader;
#[doc = "Field `ENABLE` writer - 3:0\\]
Enable region. A value of 0xA enables, others disable."]
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LOCK` reader - 4:4\\]
Lock region. Once set region values cannot be modified."]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - 4:4\\]
Lock region. Once set region values cannot be modified."]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BACKGROUND` reader - 8:8\\]
Background enable for region. There can be 1 backgroun region per FW and foreground regions can have overlapping addresses only with the background region."]
pub type BackgroundR = crate::BitReader;
#[doc = "Field `BACKGROUND` writer - 8:8\\]
Background enable for region. There can be 1 backgroun region per FW and foreground regions can have overlapping addresses only with the background region."]
pub type BackgroundW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_MODE` reader - 9:9\\]
Cache mode for region. Set to 1 to check cache permissions. Set to 0 to ignore cache permissions."]
pub type CacheModeR = crate::BitReader;
#[doc = "Field `CACHE_MODE` writer - 9:9\\]
Cache mode for region. Set to 1 to check cache permissions. Set to 0 to ignore cache permissions."]
pub type CacheModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Enable region. A value of 0xA enables, others disable."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Lock region. Once set region values cannot be modified."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Background enable for region. There can be 1 backgroun region per FW and foreground regions can have overlapping addresses only with the background region."]
    #[inline(always)]
    pub fn background(&self) -> BackgroundR {
        BackgroundR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Cache mode for region. Set to 1 to check cache permissions. Set to 0 to ignore cache permissions."]
    #[inline(always)]
    pub fn cache_mode(&self) -> CacheModeR {
        CacheModeR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Enable region. A value of 0xA enables, others disable."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<FwRegsIsam64DdrWrapMain0DdrssFwRegion2ControlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Lock region. Once set region values cannot be modified."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<FwRegsIsam64DdrWrapMain0DdrssFwRegion2ControlSpec> {
        LockW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Background enable for region. There can be 1 backgroun region per FW and foreground regions can have overlapping addresses only with the background region."]
    #[inline(always)]
    #[must_use]
    pub fn background(&mut self) -> BackgroundW<FwRegsIsam64DdrWrapMain0DdrssFwRegion2ControlSpec> {
        BackgroundW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Cache mode for region. Set to 1 to check cache permissions. Set to 0 to ignore cache permissions."]
    #[inline(always)]
    #[must_use]
    pub fn cache_mode(&mut self) -> CacheModeW<FwRegsIsam64DdrWrapMain0DdrssFwRegion2ControlSpec> {
        CacheModeW::new(self, 9)
    }
}
#[doc = "The FW Region 2 Control Register defines the control fields for the slave Isam64_ddr_wrap_main_0.ddrss region 2 firewall.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fw_regs_isam64_ddr_wrap_main_0_ddrss_fw_region_2_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fw_regs_isam64_ddr_wrap_main_0_ddrss_fw_region_2_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwRegsIsam64DdrWrapMain0DdrssFwRegion2ControlSpec;
impl crate::RegisterSpec for FwRegsIsam64DdrWrapMain0DdrssFwRegion2ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fw_regs_isam64_ddr_wrap_main_0_ddrss_fw_region_2_control::R`](R) reader structure"]
impl crate::Readable for FwRegsIsam64DdrWrapMain0DdrssFwRegion2ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`fw_regs_isam64_ddr_wrap_main_0_ddrss_fw_region_2_control::W`](W) writer structure"]
impl crate::Writable for FwRegsIsam64DdrWrapMain0DdrssFwRegion2ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FW_REGS_Isam64_ddr_wrap_main_0_ddrss_fw_region_2_control to value 0"]
impl crate::Resettable for FwRegsIsam64DdrWrapMain0DdrssFwRegion2ControlSpec {
    const RESET_VALUE: u32 = 0;
}
