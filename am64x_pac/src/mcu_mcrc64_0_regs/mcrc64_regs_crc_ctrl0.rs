#[doc = "Register `MCRC64_REGS_CRC_CTRL0` reader"]
pub type R = crate::R<Mcrc64RegsCrcCtrl0Spec>;
#[doc = "Register `MCRC64_REGS_CRC_CTRL0` writer"]
pub type W = crate::W<Mcrc64RegsCrcCtrl0Spec>;
#[doc = "Field `CH1_PSA_SWRE` reader - 0:0\\]
Channel 1 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a 0 . 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
pub type Ch1PsaSwreR = crate::BitReader;
#[doc = "Field `CH1_PSA_SWRE` writer - 0:0\\]
Channel 1 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a 0 . 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
pub type Ch1PsaSwreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_PSA_SWRE` reader - 8:8\\]
Channel 2 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a 0 . 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
pub type Ch2PsaSwreR = crate::BitReader;
#[doc = "Field `CH2_PSA_SWRE` writer - 8:8\\]
Channel 2 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a 0 . 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
pub type Ch2PsaSwreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_PSA_SWRE` reader - 16:16\\]
Channel 3 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a 0 . 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
pub type Ch3PsaSwreR = crate::BitReader;
#[doc = "Field `CH3_PSA_SWRE` writer - 16:16\\]
Channel 3 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a 0 . 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
pub type Ch3PsaSwreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_PSA_SWRE` reader - 24:24\\]
Channel 4 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a 0 . 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
pub type Ch4PsaSwreR = crate::BitReader;
#[doc = "Field `CH4_PSA_SWRE` writer - 24:24\\]
Channel 4 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a 0 . 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
pub type Ch4PsaSwreW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Channel 1 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a 0 . 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
    #[inline(always)]
    pub fn ch1_psa_swre(&self) -> Ch1PsaSwreR {
        Ch1PsaSwreR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel 2 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a 0 . 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
    #[inline(always)]
    pub fn ch2_psa_swre(&self) -> Ch2PsaSwreR {
        Ch2PsaSwreR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Channel 3 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a 0 . 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
    #[inline(always)]
    pub fn ch3_psa_swre(&self) -> Ch3PsaSwreR {
        Ch3PsaSwreR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Channel 4 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a 0 . 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
    #[inline(always)]
    pub fn ch4_psa_swre(&self) -> Ch4PsaSwreR {
        Ch4PsaSwreR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Channel 1 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a 0 . 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_psa_swre(&mut self) -> Ch1PsaSwreW<Mcrc64RegsCrcCtrl0Spec> {
        Ch1PsaSwreW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel 2 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a 0 . 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_psa_swre(&mut self) -> Ch2PsaSwreW<Mcrc64RegsCrcCtrl0Spec> {
        Ch2PsaSwreW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Channel 3 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a 0 . 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_psa_swre(&mut self) -> Ch3PsaSwreW<Mcrc64RegsCrcCtrl0Spec> {
        Ch3PsaSwreW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Channel 4 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a 0 . 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_psa_swre(&mut self) -> Ch4PsaSwreW<Mcrc64RegsCrcCtrl0Spec> {
        Ch4PsaSwreW::new(self, 24)
    }
}
#[doc = "CRC Global Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcCtrl0Spec;
impl crate::RegisterSpec for Mcrc64RegsCrcCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_ctrl0::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_ctrl0::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcCtrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_CTRL0 to value 0"]
impl crate::Resettable for Mcrc64RegsCrcCtrl0Spec {
    const RESET_VALUE: u32 = 0;
}
