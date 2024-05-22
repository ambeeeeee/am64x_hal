#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_CONTROL_REG` reader"]
pub type R = crate::R<Pr1MdioV1p7_Mdio_RegsControlRegSpec>;
#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_CONTROL_REG` writer"]
pub type W = crate::W<Pr1MdioV1p7_Mdio_RegsControlRegSpec>;
#[doc = "Field `CLKDIV` reader - 15:0\\]
Clock divider"]
pub type ClkdivR = crate::FieldReader<u16>;
#[doc = "Field `CLKDIV` writer - 15:0\\]
Clock divider"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INT_TEST_ENABLE` reader - 17:17\\]
Interrupt test enable"]
pub type IntTestEnableR = crate::BitReader;
#[doc = "Field `INT_TEST_ENABLE` writer - 17:17\\]
Interrupt test enable"]
pub type IntTestEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT_DETECT_ENABLE` reader - 18:18\\]
Fault detect enable"]
pub type FaultDetectEnableR = crate::BitReader;
#[doc = "Field `FAULT_DETECT_ENABLE` writer - 18:18\\]
Fault detect enable"]
pub type FaultDetectEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT` reader - 19:19\\]
Fault indicator"]
pub type FaultR = crate::BitReader;
#[doc = "Field `FAULT` writer - 19:19\\]
Fault indicator"]
pub type FaultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREAMBLE` reader - 20:20\\]
Preamble disable"]
pub type PreambleR = crate::BitReader;
#[doc = "Field `PREAMBLE` writer - 20:20\\]
Preamble disable"]
pub type PreambleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGHEST_USER_CHANNEL` reader - 28:24\\]
Highest user channel"]
pub type HighestUserChannelR = crate::FieldReader;
#[doc = "Field `HIGHEST_USER_CHANNEL` writer - 28:24\\]
Highest user channel"]
pub type HighestUserChannelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ENABLE` reader - 30:30\\]
Enable control"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - 30:30\\]
Enable control"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE` reader - 31:31\\]
MDIO state machine idle"]
pub type IdleR = crate::BitReader;
#[doc = "Field `IDLE` writer - 31:31\\]
MDIO state machine idle"]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Clock divider"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt test enable"]
    #[inline(always)]
    pub fn int_test_enable(&self) -> IntTestEnableR {
        IntTestEnableR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Fault detect enable"]
    #[inline(always)]
    pub fn fault_detect_enable(&self) -> FaultDetectEnableR {
        FaultDetectEnableR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Fault indicator"]
    #[inline(always)]
    pub fn fault(&self) -> FaultR {
        FaultR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Preamble disable"]
    #[inline(always)]
    pub fn preamble(&self) -> PreambleR {
        PreambleR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Highest user channel"]
    #[inline(always)]
    pub fn highest_user_channel(&self) -> HighestUserChannelR {
        HighestUserChannelR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Enable control"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
MDIO state machine idle"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> ClkdivW<Pr1MdioV1p7_Mdio_RegsControlRegSpec> {
        ClkdivW::new(self, 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt test enable"]
    #[inline(always)]
    #[must_use]
    pub fn int_test_enable(&mut self) -> IntTestEnableW<Pr1MdioV1p7_Mdio_RegsControlRegSpec> {
        IntTestEnableW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Fault detect enable"]
    #[inline(always)]
    #[must_use]
    pub fn fault_detect_enable(
        &mut self,
    ) -> FaultDetectEnableW<Pr1MdioV1p7_Mdio_RegsControlRegSpec> {
        FaultDetectEnableW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Fault indicator"]
    #[inline(always)]
    #[must_use]
    pub fn fault(&mut self) -> FaultW<Pr1MdioV1p7_Mdio_RegsControlRegSpec> {
        FaultW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Preamble disable"]
    #[inline(always)]
    #[must_use]
    pub fn preamble(&mut self) -> PreambleW<Pr1MdioV1p7_Mdio_RegsControlRegSpec> {
        PreambleW::new(self, 20)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Highest user channel"]
    #[inline(always)]
    #[must_use]
    pub fn highest_user_channel(
        &mut self,
    ) -> HighestUserChannelW<Pr1MdioV1p7_Mdio_RegsControlRegSpec> {
        HighestUserChannelW::new(self, 24)
    }
    #[doc = "Bit 30 - 30:30\\]
Enable control"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<Pr1MdioV1p7_Mdio_RegsControlRegSpec> {
        EnableW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
MDIO state machine idle"]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IdleW<Pr1MdioV1p7_Mdio_RegsControlRegSpec> {
        IdleW::new(self, 31)
    }
}
#[doc = "control_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_control_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_control_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MdioV1p7_Mdio_RegsControlRegSpec;
impl crate::RegisterSpec for Pr1MdioV1p7_Mdio_RegsControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mdio_v1p7__mdio__regs_control_reg::R`](R) reader structure"]
impl crate::Readable for Pr1MdioV1p7_Mdio_RegsControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mdio_v1p7__mdio__regs_control_reg::W`](W) writer structure"]
impl crate::Writable for Pr1MdioV1p7_Mdio_RegsControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MDIO_V1P7__MDIO__REGS_CONTROL_REG to value 0x8100_0255"]
impl crate::Resettable for Pr1MdioV1p7_Mdio_RegsControlRegSpec {
    const RESET_VALUE: u32 = 0x8100_0255;
}
