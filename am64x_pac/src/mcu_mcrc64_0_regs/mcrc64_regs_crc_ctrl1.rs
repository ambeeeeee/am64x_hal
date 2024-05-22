#[doc = "Register `MCRC64_REGS_CRC_CTRL1` reader"]
pub type R = crate::R<Mcrc64RegsCrcCtrl1Spec>;
#[doc = "Register `MCRC64_REGS_CRC_CTRL1` writer"]
pub type W = crate::W<Mcrc64RegsCrcCtrl1Spec>;
#[doc = "Field `PWDN` reader - 0:0\\]
Power Down. When set, MCRC moduleMCRC Module is put in power down mode. 0 = MCRC is not in power down mode. 1 = MCRC is in power down mode."]
pub type PwdnR = crate::BitReader;
#[doc = "Field `PWDN` writer - 0:0\\]
Power Down. When set, MCRC moduleMCRC Module is put in power down mode. 0 = MCRC is not in power down mode. 1 = MCRC is in power down mode."]
pub type PwdnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Power Down. When set, MCRC moduleMCRC Module is put in power down mode. 0 = MCRC is not in power down mode. 1 = MCRC is in power down mode."]
    #[inline(always)]
    pub fn pwdn(&self) -> PwdnR {
        PwdnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Power Down. When set, MCRC moduleMCRC Module is put in power down mode. 0 = MCRC is not in power down mode. 1 = MCRC is in power down mode."]
    #[inline(always)]
    #[must_use]
    pub fn pwdn(&mut self) -> PwdnW<Mcrc64RegsCrcCtrl1Spec> {
        PwdnW::new(self, 0)
    }
}
#[doc = "CRC Global Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcCtrl1Spec;
impl crate::RegisterSpec for Mcrc64RegsCrcCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_ctrl1::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_ctrl1::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_CTRL1 to value 0"]
impl crate::Resettable for Mcrc64RegsCrcCtrl1Spec {
    const RESET_VALUE: u32 = 0;
}
