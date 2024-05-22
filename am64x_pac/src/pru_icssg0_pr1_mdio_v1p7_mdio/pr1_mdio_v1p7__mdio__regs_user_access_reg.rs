#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_USER_ACCESS_REG` reader"]
pub type R = crate::R<Pr1MdioV1p7_Mdio_RegsUserAccessRegSpec>;
#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_USER_ACCESS_REG` writer"]
pub type W = crate::W<Pr1MdioV1p7_Mdio_RegsUserAccessRegSpec>;
#[doc = "Field `DATA` reader - 15:0\\]
User data"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - 15:0\\]
User data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHYADR` reader - 20:16\\]
PHY address"]
pub type PhyadrR = crate::FieldReader;
#[doc = "Field `PHYADR` writer - 20:16\\]
PHY address"]
pub type PhyadrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REGADR` reader - 25:21\\]
Register address"]
pub type RegadrR = crate::FieldReader;
#[doc = "Field `REGADR` writer - 25:21\\]
Register address"]
pub type RegadrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ACK` reader - 29:29\\]
Acknowledge"]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - 29:29\\]
Acknowledge"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` reader - 30:30\\]
Write"]
pub type WriteR = crate::BitReader;
#[doc = "Field `WRITE` writer - 30:30\\]
Write"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GO` reader - 31:31\\]
Go"]
pub type GoR = crate::BitReader;
#[doc = "Field `GO` writer - 31:31\\]
Go"]
pub type GoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
User data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - 20:16\\]
PHY address"]
    #[inline(always)]
    pub fn phyadr(&self) -> PhyadrR {
        PhyadrR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - 25:21\\]
Register address"]
    #[inline(always)]
    pub fn regadr(&self) -> RegadrR {
        RegadrR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
Acknowledge"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Write"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Go"]
    #[inline(always)]
    pub fn go(&self) -> GoR {
        GoR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
User data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Pr1MdioV1p7_Mdio_RegsUserAccessRegSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
PHY address"]
    #[inline(always)]
    #[must_use]
    pub fn phyadr(&mut self) -> PhyadrW<Pr1MdioV1p7_Mdio_RegsUserAccessRegSpec> {
        PhyadrW::new(self, 16)
    }
    #[doc = "Bits 21:25 - 25:21\\]
Register address"]
    #[inline(always)]
    #[must_use]
    pub fn regadr(&mut self) -> RegadrW<Pr1MdioV1p7_Mdio_RegsUserAccessRegSpec> {
        RegadrW::new(self, 21)
    }
    #[doc = "Bit 29 - 29:29\\]
Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<Pr1MdioV1p7_Mdio_RegsUserAccessRegSpec> {
        AckW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Write"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WriteW<Pr1MdioV1p7_Mdio_RegsUserAccessRegSpec> {
        WriteW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Go"]
    #[inline(always)]
    #[must_use]
    pub fn go(&mut self) -> GoW<Pr1MdioV1p7_Mdio_RegsUserAccessRegSpec> {
        GoW::new(self, 31)
    }
}
#[doc = "user_access_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_user_access_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_user_access_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MdioV1p7_Mdio_RegsUserAccessRegSpec;
impl crate::RegisterSpec for Pr1MdioV1p7_Mdio_RegsUserAccessRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mdio_v1p7__mdio__regs_user_access_reg::R`](R) reader structure"]
impl crate::Readable for Pr1MdioV1p7_Mdio_RegsUserAccessRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mdio_v1p7__mdio__regs_user_access_reg::W`](W) writer structure"]
impl crate::Writable for Pr1MdioV1p7_Mdio_RegsUserAccessRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MDIO_V1P7__MDIO__REGS_USER_ACCESS_REG to value 0"]
impl crate::Resettable for Pr1MdioV1p7_Mdio_RegsUserAccessRegSpec {
    const RESET_VALUE: u32 = 0;
}
