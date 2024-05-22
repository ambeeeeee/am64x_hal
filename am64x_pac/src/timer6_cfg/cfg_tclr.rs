#[doc = "Register `CFG_TCLR` reader"]
pub type R = crate::R<CfgTclrSpec>;
#[doc = "Register `CFG_TCLR` writer"]
pub type W = crate::W<CfgTclrSpec>;
#[doc = "Field `ST` reader - 0:0\\]
Start/Stop timer control"]
pub type StR = crate::BitReader;
#[doc = "Field `ST` writer - 0:0\\]
Start/Stop timer control"]
pub type StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AR` reader - 1:1\\]
Auto-reload mode"]
pub type ArR = crate::BitReader;
#[doc = "Field `AR` writer - 1:1\\]
Auto-reload mode"]
pub type ArW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTV` reader - 4:2\\]
Trigger Output Mode. The timer counter is prescaled with the value: 2^PTV"]
pub type PtvR = crate::FieldReader;
#[doc = "Field `PTV` writer - 4:2\\]
Trigger Output Mode. The timer counter is prescaled with the value: 2^PTV"]
pub type PtvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRE` reader - 5:5\\]
Prescaler Enable"]
pub type PreR = crate::BitReader;
#[doc = "Field `PRE` writer - 5:5\\]
Prescaler Enable"]
pub type PreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CE` reader - 6:6\\]
Compare Enable"]
pub type CeR = crate::BitReader;
#[doc = "Field `CE` writer - 6:6\\]
Compare Enable"]
pub type CeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCPWM` reader - 7:7\\]
Pulse Width Modulation output pin default value"]
pub type ScpwmR = crate::BitReader;
#[doc = "Field `SCPWM` writer - 7:7\\]
Pulse Width Modulation output pin default value"]
pub type ScpwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCM` reader - 9:8\\]
Transition Capture Mode"]
pub type TcmR = crate::FieldReader;
#[doc = "Field `TCM` writer - 9:8\\]
Transition Capture Mode"]
pub type TcmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRG` reader - 11:10\\]
Trigger Output Mode"]
pub type TrgR = crate::FieldReader;
#[doc = "Field `TRG` writer - 11:10\\]
Trigger Output Mode"]
pub type TrgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PT` reader - 12:12\\]
Pulse or Toggle select bit"]
pub type PtR = crate::BitReader;
#[doc = "Field `PT` writer - 12:12\\]
Pulse or Toggle select bit"]
pub type PtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPT_MODE` reader - 13:13\\]
Capture mode select bit; First/second"]
pub type CaptModeR = crate::BitReader;
#[doc = "Field `CAPT_MODE` writer - 13:13\\]
Capture mode select bit; First/second"]
pub type CaptModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPO_CFG` reader - 14:14\\]
This field drives directly the timer_gpocfg port"]
pub type GpoCfgR = crate::BitReader;
#[doc = "Field `GPO_CFG` writer - 14:14\\]
This field drives directly the timer_gpocfg port"]
pub type GpoCfgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Start/Stop timer control"]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Auto-reload mode"]
    #[inline(always)]
    pub fn ar(&self) -> ArR {
        ArR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Trigger Output Mode. The timer counter is prescaled with the value: 2^PTV"]
    #[inline(always)]
    pub fn ptv(&self) -> PtvR {
        PtvR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Prescaler Enable"]
    #[inline(always)]
    pub fn pre(&self) -> PreR {
        PreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Compare Enable"]
    #[inline(always)]
    pub fn ce(&self) -> CeR {
        CeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Pulse Width Modulation output pin default value"]
    #[inline(always)]
    pub fn scpwm(&self) -> ScpwmR {
        ScpwmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Transition Capture Mode"]
    #[inline(always)]
    pub fn tcm(&self) -> TcmR {
        TcmR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Trigger Output Mode"]
    #[inline(always)]
    pub fn trg(&self) -> TrgR {
        TrgR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Pulse or Toggle select bit"]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Capture mode select bit; First/second"]
    #[inline(always)]
    pub fn capt_mode(&self) -> CaptModeR {
        CaptModeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
This field drives directly the timer_gpocfg port"]
    #[inline(always)]
    pub fn gpo_cfg(&self) -> GpoCfgR {
        GpoCfgR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Start/Stop timer control"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> StW<CfgTclrSpec> {
        StW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Auto-reload mode"]
    #[inline(always)]
    #[must_use]
    pub fn ar(&mut self) -> ArW<CfgTclrSpec> {
        ArW::new(self, 1)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Trigger Output Mode. The timer counter is prescaled with the value: 2^PTV"]
    #[inline(always)]
    #[must_use]
    pub fn ptv(&mut self) -> PtvW<CfgTclrSpec> {
        PtvW::new(self, 2)
    }
    #[doc = "Bit 5 - 5:5\\]
Prescaler Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PreW<CfgTclrSpec> {
        PreW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ce(&mut self) -> CeW<CfgTclrSpec> {
        CeW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Pulse Width Modulation output pin default value"]
    #[inline(always)]
    #[must_use]
    pub fn scpwm(&mut self) -> ScpwmW<CfgTclrSpec> {
        ScpwmW::new(self, 7)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Transition Capture Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tcm(&mut self) -> TcmW<CfgTclrSpec> {
        TcmW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Trigger Output Mode"]
    #[inline(always)]
    #[must_use]
    pub fn trg(&mut self) -> TrgW<CfgTclrSpec> {
        TrgW::new(self, 10)
    }
    #[doc = "Bit 12 - 12:12\\]
Pulse or Toggle select bit"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PtW<CfgTclrSpec> {
        PtW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Capture mode select bit; First/second"]
    #[inline(always)]
    #[must_use]
    pub fn capt_mode(&mut self) -> CaptModeW<CfgTclrSpec> {
        CaptModeW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
This field drives directly the timer_gpocfg port"]
    #[inline(always)]
    #[must_use]
    pub fn gpo_cfg(&mut self) -> GpoCfgW<CfgTclrSpec> {
        GpoCfgW::new(self, 14)
    }
}
#[doc = "This register controls optional features specific to the timer functionality\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTclrSpec;
impl crate::RegisterSpec for CfgTclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tclr::R`](R) reader structure"]
impl crate::Readable for CfgTclrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tclr::W`](W) writer structure"]
impl crate::Writable for CfgTclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TCLR to value 0"]
impl crate::Resettable for CfgTclrSpec {
    const RESET_VALUE: u32 = 0;
}
