#[doc = "Register `CFG0_CHNG_DDR4_FSP_ACK` reader"]
pub type R = crate::R<Cfg0ChngDdr4FspAckSpec>;
#[doc = "Register `CFG0_CHNG_DDR4_FSP_ACK` writer"]
pub type W = crate::W<Cfg0ChngDdr4FspAckSpec>;
#[doc = "Field `CHNG_DDR4_FSP_ACK_ERROR` reader - 0:0\\]
Frequency change error"]
pub type ChngDdr4FspAckErrorR = crate::BitReader;
#[doc = "Field `CHNG_DDR4_FSP_ACK_ERROR` writer - 0:0\\]
Frequency change error"]
pub type ChngDdr4FspAckErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNG_DDR4_FSP_ACK_ACK` reader - 7:7\\]
Frequency change acknowledge."]
pub type ChngDdr4FspAckAckR = crate::BitReader;
#[doc = "Field `CHNG_DDR4_FSP_ACK_ACK` writer - 7:7\\]
Frequency change acknowledge."]
pub type ChngDdr4FspAckAckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Frequency change error"]
    #[inline(always)]
    pub fn chng_ddr4_fsp_ack_error(&self) -> ChngDdr4FspAckErrorR {
        ChngDdr4FspAckErrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Frequency change acknowledge."]
    #[inline(always)]
    pub fn chng_ddr4_fsp_ack_ack(&self) -> ChngDdr4FspAckAckR {
        ChngDdr4FspAckAckR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Frequency change error"]
    #[inline(always)]
    #[must_use]
    pub fn chng_ddr4_fsp_ack_error(&mut self) -> ChngDdr4FspAckErrorW<Cfg0ChngDdr4FspAckSpec> {
        ChngDdr4FspAckErrorW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Frequency change acknowledge."]
    #[inline(always)]
    #[must_use]
    pub fn chng_ddr4_fsp_ack_ack(&mut self) -> ChngDdr4FspAckAckW<Cfg0ChngDdr4FspAckSpec> {
        ChngDdr4FspAckAckW::new(self, 7)
    }
}
#[doc = "CFG0_CHNG_DDR4_FSP_ACK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_chng_ddr4_fsp_ack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_chng_ddr4_fsp_ack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0ChngDdr4FspAckSpec;
impl crate::RegisterSpec for Cfg0ChngDdr4FspAckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_chng_ddr4_fsp_ack::R`](R) reader structure"]
impl crate::Readable for Cfg0ChngDdr4FspAckSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_chng_ddr4_fsp_ack::W`](W) writer structure"]
impl crate::Writable for Cfg0ChngDdr4FspAckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CHNG_DDR4_FSP_ACK to value 0"]
impl crate::Resettable for Cfg0ChngDdr4FspAckSpec {
    const RESET_VALUE: u32 = 0;
}
