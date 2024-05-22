#[doc = "Register `CPSW_NUSS_VBUSP_SUBSSYSTEM_STATUS_REG` reader"]
pub type R = crate::R<CpswNussVbuspSubssystemStatusRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_SUBSSYSTEM_STATUS_REG` writer"]
pub type W = crate::W<CpswNussVbuspSubssystemStatusRegSpec>;
#[doc = "Field `EEE_CLKSTOP_ACK` reader - 0:0\\]
Energy Efficient Ethernet clockstop acknowledge from CPSW"]
pub type EeeClkstopAckR = crate::BitReader;
#[doc = "Field `EEE_CLKSTOP_ACK` writer - 0:0\\]
Energy Efficient Ethernet clockstop acknowledge from CPSW"]
pub type EeeClkstopAckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Energy Efficient Ethernet clockstop acknowledge from CPSW"]
    #[inline(always)]
    pub fn eee_clkstop_ack(&self) -> EeeClkstopAckR {
        EeeClkstopAckR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Energy Efficient Ethernet clockstop acknowledge from CPSW"]
    #[inline(always)]
    #[must_use]
    pub fn eee_clkstop_ack(&mut self) -> EeeClkstopAckW<CpswNussVbuspSubssystemStatusRegSpec> {
        EeeClkstopAckW::new(self, 0)
    }
}
#[doc = "Subsystem Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_subssystem_status_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_subssystem_status_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspSubssystemStatusRegSpec;
impl crate::RegisterSpec for CpswNussVbuspSubssystemStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_subssystem_status_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspSubssystemStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_subssystem_status_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspSubssystemStatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_SUBSSYSTEM_STATUS_REG to value 0"]
impl crate::Resettable for CpswNussVbuspSubssystemStatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
